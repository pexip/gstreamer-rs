// This example demonstrates how to use gstreamer in conjunction with the gtk widget toolkit.
// This example shows the video produced by a videotestsrc within a small gtk gui.
// For this, the gtkglsink is used, which creates a gtk widget one can embed the gtk gui.
// For this, there multiple types of widgets. gtkglsink uses OpenGL to render frames, and
// gtksink uses the CPU to render the frames (which is way slower).
// So the example application first tries to use OpenGL, and when that fails, fall back.
// The pipeline looks like the following:

// gtk-gui:          {gtkglsink}-widget
//                      (|)
// {videotestsrc} - {glsinkbin}

use gst::prelude::*;

use gio::prelude::*;

use gtk::prelude::*;

use std::cell::RefCell;

fn create_ui(app: &gtk::Application) {
    let pipeline = gst::Pipeline::default();
    let src = gst::ElementFactory::make("videotestsrc").build().unwrap();
    // Create the gtk sink and retrieve the widget from it. The sink element will be used
    // in the pipeline, and the widget will be embedded in our gui.
    // Gstreamer then displays frames in the gtk widget.
    // First, we try to use the OpenGL version - and if that fails, we fall back to non-OpenGL.
    let (sink, widget) = if let Ok(gtkglsink) = gst::ElementFactory::make("gtkglsink").build() {
        // Using the OpenGL widget succeeded, so we are in for a nice playback experience with
        // low cpu usage. :)
        // The gtkglsink essentially allocates an OpenGL texture on the GPU, that it will display.
        // Now we create the glsinkbin element, which is responsible for conversions and for uploading
        // video frames to our texture (if they are not already in the GPU). Now we tell the OpenGL-sink
        // about our gtkglsink element, form where it will retrieve the OpenGL texture to fill.
        let glsinkbin = gst::ElementFactory::make("glsinkbin")
            .property("sink", &gtkglsink)
            .build()
            .unwrap();
        // The gtkglsink creates the gtk widget for us. This is accessible through a property.
        // So we get it and use it later to add it to our gui.
        let widget = gtkglsink.property::<gtk::Widget>("widget");
        (glsinkbin, widget)
    } else {
        // Unfortunately, using the OpenGL widget didn't work out, so we will have to render
        // our frames manually, using the CPU. An example why this may fail is, when
        // the PC doesn't have proper graphics drivers installed.
        let sink = gst::ElementFactory::make("gtksink").build().unwrap();
        // The gtksink creates the gtk widget for us. This is accessible through a property.
        // So we get it and use it later to add it to our gui.
        let widget = sink.property::<gtk::Widget>("widget");
        (sink, widget)
    };

    pipeline.add_many(&[&src, &sink]).unwrap();
    src.link(&sink).unwrap();

    // Create a simple gtk gui window to place our widget into.
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_default_size(320, 240);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    // Add our widget to the gui
    vbox.pack_start(&widget, true, true, 0);
    let label = gtk::Label::new(Some("Position: 00:00:00"));
    vbox.pack_start(&label, true, true, 5);
    window.add(&vbox);
    window.show_all();

    app.add_window(&window);

    // Need to move a new reference into the closure.
    // !!ATTENTION!!:
    // It might seem appealing to use pipeline.clone() here, because that greatly
    // simplifies the code within the callback. What this actually does, however, is creating
    // a memory leak. The clone of a pipeline is a new strong reference on the pipeline.
    // Storing this strong reference of the pipeline within the callback (we are moving it in!),
    // which is in turn stored in another strong reference on the pipeline is creating a
    // reference cycle.
    // DO NOT USE pipeline.clone() TO USE THE PIPELINE WITHIN A CALLBACK
    let pipeline_weak = pipeline.downgrade();
    // Add a timeout to the main loop that will periodically (every 500ms) be
    // executed. This will query the current position within the stream from
    // the underlying pipeline, and display it in our gui.
    // Since this closure is called by the mainloop thread, we are allowed
    // to modify the gui widgets here.
    let timeout_id = glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        // Here we temporarily retrieve a strong reference on the pipeline from the weak one
        // we moved into this callback.
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return glib::Continue(true),
        };

        // Query the current playing position from the underlying pipeline.
        let position = pipeline.query_position::<gst::ClockTime>();
        // Display the playing position in the gui.
        label.set_text(&format!("Position: {:.0}", position.display()));
        // Tell the callback to continue calling this closure.
        glib::Continue(true)
    });

    let bus = pipeline.bus().unwrap();

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let app_weak = app.downgrade();
    bus.add_watch_local(move |_, msg| {
        use gst::MessageView;

        let app = match app_weak.upgrade() {
            Some(app) => app,
            None => return glib::Continue(false),
        };

        match msg.view() {
            MessageView::Eos(..) => app.quit(),
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                app.quit();
            }
            _ => (),
        };

        glib::Continue(true)
    })
    .expect("Failed to add bus watch");

    // Pipeline reference is owned by the closure below, so will be
    // destroyed once the app is destroyed
    let timeout_id = RefCell::new(Some(timeout_id));
    let pipeline = RefCell::new(Some(pipeline));
    app.connect_shutdown(move |_| {
        // Optional, by manually destroying the window here we ensure that
        // the gst element is destroyed when shutting down instead of having to wait
        // for the process to terminate, allowing us to use the leaks tracer.
        unsafe {
            window.destroy();
        }

        // GTK will keep the Application alive for the whole process lifetime.
        // Wrapping the pipeline in a RefCell<Option<_>> and removing it from it here
        // ensures the pipeline is actually destroyed when shutting down, allowing us
        // to use the leaks tracer for example.
        if let Some(pipeline) = pipeline.borrow_mut().take() {
            pipeline
                .set_state(gst::State::Null)
                .expect("Unable to set the pipeline to the `Null` state");
            pipeline.bus().unwrap().remove_watch().unwrap();
        }

        if let Some(timeout_id) = timeout_id.borrow_mut().take() {
            timeout_id.remove();
        }
    });
}

fn main() {
    // Initialize gstreamer and the gtk widget toolkit libraries.
    gst::init().unwrap();
    gtk::init().unwrap();

    {
        let app = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE);

        app.connect_activate(create_ui);
        app.run();
    }

    // Optional, can be used to detect leaks using the leaks tracer
    unsafe {
        gst::deinit();
    }
}
