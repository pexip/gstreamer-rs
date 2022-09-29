// Take a look at the license at the top of the repository in the LICENSE file.

macro_rules! impl_trait_op_same(
    ($name:ident, $op:ident, $op_name:ident, $op_assign:ident, $op_assign_name:ident) => {
        impl std::ops::$op<$name> for $name {
            type Output = Self;

            fn $op_name(self, rhs: $name) -> Self::Output {
                Self(self.0.$op_name(rhs.0))
            }
        }

        impl std::ops::$op_assign<$name> for $name {
            fn $op_assign_name(&mut self, rhs: $name) {
                self.0.$op_assign_name(rhs.0)
            }
        }
    };
);

macro_rules! impl_non_trait_op_same(
    ($name:ident, $inner_type:ty) => {
        impl $name {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn checked_add(self, rhs: Self) -> Option<Self> {
                match self.0.checked_add(rhs.0) {
                    Some(res) if res <= Self::MAX.0 => Some(Self(res)),
                    _ => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn saturating_add(self, rhs: Self) -> Self {
                let res = self.0.saturating_add(rhs.0);
                if res < Self::MAX.0 {
                    Self(res)
                } else {
                    Self::MAX
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                let self_u128 = self.0 as u128;
                let rhs_128 = rhs.0 as u128;
                let res_u128 = self_u128 + rhs_128;
                if res_u128 <= Self::MAX.0 as u128 {
                    (Self(<$inner_type>::try_from(res_u128).unwrap()), false)
                } else {
                    (Self(<$inner_type>::try_from((res_u128 - Self::MAX.0 as u128 - 1) as u64).unwrap()), true)
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn wrapping_add(self, rhs: Self) -> Self {
                self.overflowing_add(rhs).0
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            // FIXME Can't use `map` in a `const fn` as of rustc 1.53.0-beta.2
            #[allow(clippy::manual_map)]
            pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
                match self.0.checked_sub(rhs.0) {
                    Some(res) => Some(Self(res)),
                    None => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn saturating_sub(self, rhs: Self) -> Self {
                Self(self.0.saturating_sub(rhs.0))
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                if self.0 >= rhs.0 {
                    (Self(self.0 - rhs.0), false)
                } else {
                    (Self(Self::MAX.0 - rhs.0 + self.0 + 1), true)
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn wrapping_sub(self, rhs: Self) -> Self {
                self.overflowing_sub(rhs).0
            }
        }
    };
);

macro_rules! impl_trait_op_inner_type(
    ($name:ident, $inner_type:ty, $op:ident, $op_name:ident, $op_assign:ident, $op_assign_name:ident) => {
        impl std::ops::$op<$inner_type> for $name {
            type Output = $name;

            fn $op_name(self, rhs: $inner_type) -> Self::Output {
                $name(self.0.$op_name(rhs))
            }
        }

        impl std::ops::$op_assign<$inner_type> for $name {
            fn $op_assign_name(&mut self, rhs: $inner_type) {
                self.0.$op_assign_name(rhs)
            }
        }
    };
);

macro_rules! impl_non_trait_op_inner_type(
    ($name:ident, $inner_type:ty) => {
        impl $name {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn checked_div(self, rhs: $inner_type) -> Option<Self> {
                match self.0.checked_div(rhs) {
                    Some(val) => Some(Self(val)),
                    None => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn saturating_div(self, rhs: $inner_type) -> Self {
                Self(self.0.saturating_div(rhs))
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn checked_mul(self, rhs: $inner_type) -> Option<Self> {
                match self.0.checked_mul(rhs) {
                    Some(res) if res <= Self::MAX.0 => Some(Self(res)),
                    _ => None,
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn saturating_mul(self, rhs: $inner_type) -> Self {
                let res = self.0.saturating_mul(rhs);
                if res < Self::MAX.0 {
                    Self(res)
                } else {
                    Self::MAX
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn overflowing_mul(self, rhs: $inner_type) -> (Self, bool) {
                let self_u128 = self.0 as u128;
                let rhs_128 = rhs as u128;
                let res_u128 = self_u128 * rhs_128;
                if res_u128 <= Self::MAX.0 as u128 {
                    (Self(<$inner_type>::try_from(res_u128).unwrap()), false)
                } else {
                    (Self(<$inner_type>::try_from((res_u128 - Self::MAX.0 as u128 - 1) as u64).unwrap()), true)
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub fn wrapping_mul(self, rhs: $inner_type) -> Self {
                self.overflowing_mul(rhs).0
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            pub const fn checked_rem(self, rhs: $inner_type) -> Option<Self> {
                match self.0.checked_rem(rhs) {
                    Some(val) => Some(Self(val)),
                    None => None,
                }
            }
        }
    };
);

macro_rules! impl_unsigned_int_into_signed(
    ($type:ty) => {
        impl crate::UnsignedIntoSigned for $type {
            type Signed = crate::Signed<$type>;

            fn into_positive(self) -> Self::Signed {
                crate::Signed::Positive(self)
            }

            fn into_negative(self) -> Self::Signed {
                crate::Signed::Negative(self)
            }
        }

        impl crate::UnsignedIntoSigned for Option<$type> {
            type Signed = Option<crate::Signed<$type>>;

            fn into_positive(self) -> Self::Signed {
                Some(self?.into_positive())
            }

            fn into_negative(self) -> Self::Signed {
                Some(self?.into_negative())
            }
        }
    };

    ($type:ty, $inner_type:ty) => {
        impl_unsigned_int_into_signed!($type);

        impl crate::Signed<$type> {
            // rustdoc-stripper-ignore-next
            /// Returns a `Signed` containing the inner type of `self`.
            pub fn into_inner_signed(self) -> crate::Signed<$inner_type> {
                use crate::Signed::*;
                match self {
                    Positive(new_type) => Positive(*new_type),
                    Negative(new_type) => Negative(*new_type),
                }
            }
        }
    };
);

macro_rules! impl_common_ops_for_newtype_uint(
    ($name:ident, $inner_type:ty) => {
        impl $name {
            pub const ZERO: Self = Self(0);
            pub const NONE: Option<Self> = None;

            pub const MAX_SIGNED: crate::Signed::<$name> = crate::Signed::Positive(Self::MAX);
            pub const MIN_SIGNED: crate::Signed::<$name> = crate::Signed::Negative(Self::MAX);

            pub const fn is_zero(self) -> bool {
                self.0 == Self::ZERO.0
            }
        }

        impl_trait_op_same!($name, Add, add, AddAssign, add_assign);
        impl_trait_op_same!($name, Sub, sub, SubAssign, sub_assign);
        impl std::ops::Div<$name> for $name {
            type Output = $inner_type;

            fn div(self, rhs: $name) -> $inner_type {
                self.0.div(rhs.0)
            }
        }
        impl std::ops::Rem<$name> for $name {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self {
                Self(self.0.rem(rhs.0))
            }
        }

        impl_non_trait_op_same!($name, $inner_type);

        impl_trait_op_inner_type!($name, $inner_type, Mul, mul, MulAssign, mul_assign);
        impl std::ops::Mul<$name> for $inner_type {
            type Output = $name;

            fn mul(self, rhs: $name) -> $name {
                $name(self.mul(rhs.0))
            }
        }

        impl_trait_op_inner_type!($name, $inner_type, Div, div, DivAssign, div_assign);
        impl_trait_op_inner_type!($name, $inner_type, Rem, rem, RemAssign, rem_assign);

        impl_non_trait_op_inner_type!($name, $inner_type);

        impl_unsigned_int_into_signed!($name, $inner_type);

        impl_signed_ops!($name, $inner_type, $name::ZERO);

        impl muldiv::MulDiv<$inner_type> for $name {
            type Output = $name;

            fn mul_div_floor(self, num: $inner_type, denom: $inner_type) -> Option<Self::Output> {
                self.0
                    .mul_div_floor(num, denom)
                    .map($name)
            }

            fn mul_div_round(self, num: $inner_type, denom: $inner_type) -> Option<Self::Output> {
                self.0
                    .mul_div_round(num, denom)
                    .map($name)
            }

            fn mul_div_ceil(self, num: $inner_type, denom: $inner_type) -> Option<Self::Output> {
                self.0
                    .mul_div_ceil(num, denom)
                    .map($name)
            }
        }

        impl opt_ops::OptionOperations for $name {}

        impl opt_ops::OptionCheckedAdd for $name {
            type Output = Self;
            fn opt_checked_add(
                self,
                rhs: Self,
            ) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.checked_add(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingAdd for $name {
            type Output = Self;
            fn opt_saturating_add(self, rhs: Self) -> Option<Self::Output> {
                Some(self.saturating_add(rhs))
            }
        }

        impl opt_ops::OptionOverflowingAdd for $name {
            type Output = Self;
            fn opt_overflowing_add(self, rhs: Self) -> Option<(Self::Output, bool)> {
                let res = self.overflowing_add(rhs);
                Some((res.0, res.1))
            }
        }

        impl opt_ops::OptionWrappingAdd for $name {
            type Output = Self;
            fn opt_wrapping_add(self, rhs: Self) -> Option<Self::Output> {
                Some(self.wrapping_add(rhs))
            }
        }

        impl opt_ops::OptionCheckedDiv<$inner_type> for $name {
            type Output = Self;
            fn opt_checked_div(self, rhs: $inner_type) -> Result<Option<Self::Output>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self
                    .checked_div(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedDiv for $name {
            type Output = $inner_type;
            fn opt_checked_div(self, rhs: Self) -> Result<Option<Self::Output>, opt_ops::Error> {
                if rhs.0 == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.0
                    .checked_div(rhs.0)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedMul<$inner_type> for $name {
            type Output = Self;
            fn opt_checked_mul(
                self,
                rhs: $inner_type,
            ) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.checked_mul(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedMul<$name> for $inner_type {
            type Output = $name;
            fn opt_checked_mul(
                self,
                rhs: $name,
            ) -> Result<Option<Self::Output>, opt_ops::Error> {
                rhs.checked_mul(self)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingMul<$inner_type> for $name {
            type Output = Self;
            fn opt_saturating_mul(self, rhs: $inner_type) -> Option<Self::Output> {
                Some(self.saturating_mul(rhs))
            }
        }

        impl opt_ops::OptionSaturatingMul<$name> for $inner_type {
            type Output = $name;
            fn opt_saturating_mul(self, rhs: $name) -> Option<Self::Output> {
                Some(rhs.saturating_mul(self))
            }
        }

        impl opt_ops::OptionOverflowingMul<$inner_type> for $name {
            type Output = Self;
            fn opt_overflowing_mul(self, rhs: $inner_type) -> Option<(Self::Output, bool)> {
                let res = self.overflowing_mul(rhs);
                Some((res.0, res.1))
            }
        }

        impl opt_ops::OptionOverflowingMul<$name> for $inner_type {
            type Output = $name;
            fn opt_overflowing_mul(self, rhs: $name) -> Option<(Self::Output, bool)> {
                let res = rhs.overflowing_mul(self);
                Some((res.0, res.1))
            }
        }

        impl opt_ops::OptionWrappingMul<$inner_type> for $name {
            type Output = Self;
            fn opt_wrapping_mul(self, rhs: $inner_type) -> Option<Self::Output> {
                Some(self.wrapping_mul(rhs))
            }
        }

        impl opt_ops::OptionWrappingMul<$name> for $inner_type {
            type Output = $name;
            fn opt_wrapping_mul(self, rhs: $name) -> Option<Self::Output> {
                Some(rhs.wrapping_mul(self))
            }
        }

        impl opt_ops::OptionCheckedRem<$inner_type> for $name {
            type Output = Self;
            fn opt_checked_rem(self, rhs: $inner_type) -> Result<Option<Self::Output>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.checked_rem(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedRem for $name {
            type Output = Self;
            fn opt_checked_rem(self, rhs: Self) -> Result<Option<Self::Output>, opt_ops::Error> {
                if rhs.0 == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.checked_rem(rhs.0)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedSub for $name {
            type Output = Self;
            fn opt_checked_sub(
                self,
                rhs: Self,
            ) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.checked_sub(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingSub for $name {
            type Output = Self;
            fn opt_saturating_sub(self, rhs: Self) -> Option<Self::Output> {
                Some(self.saturating_sub(rhs))
            }
        }

        impl opt_ops::OptionOverflowingSub for $name {
            type Output = Self;
            fn opt_overflowing_sub(self, rhs: Self) -> Option<(Self::Output, bool)> {
                let res = self.overflowing_sub(rhs);
                Some((res.0, res.1))
            }
        }

        impl opt_ops::OptionWrappingSub for $name {
            type Output = Self;
            fn opt_wrapping_sub(self, rhs: Self) -> Option<Self::Output> {
                Some(self.wrapping_sub(rhs))
            }
        }
    };
);

macro_rules! impl_signed_ops(
    (u64) => {
        impl_signed_ops!(u64, u64, 0);
    };

    (u32) => {
        impl_signed_ops!(u32, u32, 0);
    };

    ($type:ty, $inner_type:ty, $zero:expr) => {
        impl crate::Signed<$type> {
            // rustdoc-stripper-ignore-next
            /// Returns the signum for this `Signed`.
            ///
            /// Returns:
            ///
            /// - `0` if the number is zero.
            /// - `1` if the value must be considered as positive.
            /// - `-1` if the value must be considered as negative.
            pub fn signum(self) -> i32 {
                use crate::Signed::*;
                match self {
                    Positive(val) | Negative(val) if val == $zero => 0i32,
                    Positive(_) => 1i32,
                    Negative(_) => -1i32,
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the checked subtraction `self - other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_sub(self, other: Self) -> Option<Self> {
                use crate::Signed::*;
                match (self, other) {
                    (Positive(a), Positive(b)) if a >= b => Some(Positive(a - b)),
                    (Positive(a), Positive(b)) => Some(Negative(b - a)),
                    (Negative(a), Negative(b)) if a >= b => Some(Negative(a - b)),
                    (Negative(a), Negative(b)) => Some(Positive(b - a)),
                    (Positive(a), Negative(b)) => a.checked_add(b).map(Positive),
                    (Negative(a), Positive(b)) => a.checked_add(b).map(Negative),
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the checked subtraction `self - other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_sub_unsigned(self, other: $type) -> Option<Self> {
                self.checked_sub(crate::Signed::Positive(other))
            }

            // rustdoc-stripper-ignore-next
            /// Returns the checked addition `self + other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_add(self, other: Self) -> Option<Self> {
                use crate::Signed::*;
                match (self, other) {
                    (Positive(a), Positive(b)) => a.checked_add(b).map(Positive),
                    (Negative(a), Negative(b)) => a.checked_add(b).map(Negative),
                    (Positive(_), Negative(_)) => self.checked_sub(-other),
                    (Negative(_), Positive(_)) => Some(-((-self).checked_sub(other)?))
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the checked addition `self + other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_add_unsigned(self, other: $type) -> Option<Self> {
                self.checked_add(crate::Signed::Positive(other))
            }

            // rustdoc-stripper-ignore-next
            /// Returns the saturating subtraction `self - other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_sub(self, other: Self) -> Self {
                use crate::Signed::*;
                match (self, other) {
                    (Positive(a), Positive(b)) if a >= b => Positive(a - b),
                    (Positive(a), Positive(b)) => Negative(b - a),
                    (Negative(a), Negative(b)) if a >= b => Negative(a - b),
                    (Negative(a), Negative(b)) => Positive(b - a),
                    (Positive(a), Negative(b)) => Positive(a.saturating_add(b)),
                    (Negative(a), Positive(b)) => Negative(a.saturating_add(b)),
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the saturating subtraction `self - other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_sub_unsigned(self, other: $type) -> Self {
                self.saturating_sub(crate::Signed::Positive(other))
            }

            // rustdoc-stripper-ignore-next
            /// Returns the saturating addition `self + other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_add(self, other: Self) -> Self {
                use crate::Signed::*;
                match (self, other) {
                    (Positive(a), Positive(b)) => Positive(a.saturating_add(b)),
                    (Negative(a), Negative(b)) => Negative(a.saturating_add(b)),
                    (Positive(_), Negative(_)) => self.saturating_sub(-other),
                    (Negative(_), Positive(_)) => -((-self).saturating_sub(other)),
                }
            }

            // rustdoc-stripper-ignore-next
            /// Returns the saturating addition `self + other`.
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_add_unsigned(self, other: $type) -> Self {
                self.saturating_add(crate::Signed::Positive(other))
            }
        }

        impl std::ops::Add<crate::Signed<$type>> for crate::Signed<$type> {
            type Output = crate::Signed<$type>;

            fn add(self, other: crate::Signed<$type>) -> crate::Signed<$type> {
                self.checked_add(other).expect("Overflowing addition")
            }
        }

        impl std::ops::AddAssign<crate::Signed<$type>> for crate::Signed<$type> {
            fn add_assign(&mut self, other: crate::Signed<$type>) {
                *self = self.checked_add(other).expect("Overflowing addition")
            }
        }

        impl std::ops::Sub<crate::Signed<$type>> for crate::Signed<$type> {
            type Output = crate::Signed<$type>;

            fn sub(self, other: crate::Signed<$type>) -> crate::Signed<$type> {
                self.checked_sub(other).expect("Overflowing subtraction")
            }
        }

        impl std::ops::SubAssign<crate::Signed<$type>> for crate::Signed<$type> {
            fn sub_assign(&mut self, other: crate::Signed<$type>) {
                *self = self.checked_sub(other).expect("Overflowing subtraction")
            }
        }

        impl std::ops::Add<$type> for crate::Signed<$type> {
            type Output = crate::Signed<$type>;

            fn add(self, other: $type) -> crate::Signed<$type> {
                self.checked_add(crate::Signed::Positive(other)).expect("Overflowing addition")
            }
        }

        impl std::ops::AddAssign<$type> for crate::Signed<$type> {
            fn add_assign(&mut self, other: $type) {
                *self = self.checked_add(crate::Signed::Positive(other)).expect("Overflowing addition")
            }
        }

        impl std::ops::Sub<$type> for crate::Signed<$type> {
            type Output = crate::Signed<$type>;

            fn sub(self, other: $type) -> crate::Signed<$type> {
                self.checked_sub(crate::Signed::Positive(other)).expect("Overflowing subtraction")
            }
        }

        impl std::ops::SubAssign<$type> for crate::Signed<$type> {
            fn sub_assign(&mut self, other: $type) {
                *self = self.checked_sub(crate::Signed::Positive(other)).expect("Overflowing subtraction")
            }
        }

        impl std::ops::Add<crate::Signed<$type>> for $type {
            type Output = crate::Signed<$type>;
            fn add(self, other: crate::Signed<$type>) -> crate::Signed<$type> {
                crate::Signed::Positive(self).checked_add(other).expect("Overflowing addition")
            }
        }

        impl std::ops::Sub<crate::Signed<$type>> for $type {
            type Output = crate::Signed<$type>;
            fn sub(self, other: crate::Signed<$type>) -> crate::Signed<$type> {
                crate::Signed::Positive(self).checked_sub(other).expect("Overflowing subtraction")
            }
        }

        impl std::cmp::PartialOrd<crate::Signed<$type>> for crate::Signed<$type> {
            fn partial_cmp(&self, other: &crate::Signed<$type>) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl std::cmp::PartialEq<$type> for crate::Signed<$type> {
            fn eq(&self, other: &$type) -> bool {
                self.eq(&crate::Signed::Positive(*other))
            }
        }

        impl std::cmp::PartialEq<crate::Signed<$type>> for $type {
            fn eq(&self, other: &crate::Signed<$type>) -> bool {
                crate::Signed::Positive(*self).eq(other)
            }
        }

        impl std::cmp::PartialOrd<$type> for crate::Signed<$type> {
            fn partial_cmp(&self, other: &$type) -> Option<std::cmp::Ordering> {
                Some(self.cmp(&crate::Signed::Positive(*other)))
            }
        }

        impl std::cmp::PartialOrd<crate::Signed<$type>> for $type {
            fn partial_cmp(&self, other: &crate::Signed<$type>) -> Option<std::cmp::Ordering> {
                Some(crate::Signed::Positive(*self).cmp(other))
            }
        }

        impl std::cmp::Ord for crate::Signed<$type> {
            fn cmp(&self, other: &crate::Signed<$type>) -> std::cmp::Ordering {
                use crate::Signed::*;
                match (self, other) {
                    (Positive(a), Positive(b)) => a.cmp(b),
                    (Negative(a), Negative(b)) => b.cmp(a),
                    (Positive(_), Negative(_)) => std::cmp::Ordering::Greater,
                    (Negative(_), Positive(_)) => std::cmp::Ordering::Less,
                }
            }
        }

        impl opt_ops::OptionOperations for crate::Signed<$type> {}

        impl opt_ops::OptionCheckedAdd for crate::Signed<$type> {
            type Output = Self;
            fn opt_checked_add(
                self,
                rhs: Self,
            ) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.checked_add(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingAdd for crate::Signed<$type> {
            type Output = Self;
            fn opt_saturating_add(self, rhs: Self) -> Option<Self::Output> {
                Some(self.saturating_add(rhs))
            }
        }

        impl opt_ops::OptionCheckedSub for crate::Signed<$type> {
            type Output = Self;
            fn opt_checked_sub(
                self,
                rhs: Self,
            ) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.checked_sub(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingSub for crate::Signed<$type> {
            type Output = Self;
            fn opt_saturating_sub(self, rhs: Self) -> Option<Self::Output> {
                Some(self.saturating_sub(rhs))
            }
        }

        impl opt_ops::OptionCheckedAdd<$type> for crate::Signed<$type> {
            type Output = Self;
            fn opt_checked_add(
                self,
                rhs: $type,
            ) -> Result<Option<Self>, opt_ops::Error> {
                self.opt_checked_add(crate::Signed::Positive(rhs))
            }
        }

        impl opt_ops::OptionSaturatingAdd<$type> for crate::Signed<$type> {
            type Output = Self;
            fn opt_saturating_add(self, rhs: $type) -> Option<Self> {
                self.opt_saturating_add(crate::Signed::Positive(rhs))
            }
        }

        impl opt_ops::OptionCheckedSub<$type> for crate::Signed<$type> {
            type Output = Self;
            fn opt_checked_sub(
                self,
                rhs: $type,
            ) -> Result<Option<Self>, opt_ops::Error> {
                self.opt_checked_sub(crate::Signed::Positive(rhs))
            }
        }

        impl opt_ops::OptionSaturatingSub<$type> for crate::Signed<$type> {
            type Output = Self;
            fn opt_saturating_sub(self, rhs: $type) -> Option<Self> {
                self.opt_saturating_sub(crate::Signed::Positive(rhs))
            }
        }

        impl opt_ops::OptionCheckedAdd<crate::Signed<$type>> for $type {
            type Output = crate::Signed<$type>;
            fn opt_checked_add(
                self,
                rhs: crate::Signed<$type>,
            ) -> Result<Option<Self::Output>, opt_ops::Error> {
                crate::Signed::Positive(self).opt_checked_add(rhs)
            }
        }

        impl opt_ops::OptionSaturatingAdd<crate::Signed<$type>> for $type {
            type Output = crate::Signed<$type>;
            fn opt_saturating_add(
                self,
                rhs: crate::Signed<$type>
            ) -> Option<Self::Output> {
                crate::Signed::Positive(self).opt_saturating_add(rhs)
            }
        }

        impl opt_ops::OptionCheckedSub<crate::Signed<$type>> for $type {
            type Output = crate::Signed<$type>;
            fn opt_checked_sub(
                self,
                rhs: crate::Signed<$type>,
            ) -> Result<Option<Self::Output>, opt_ops::Error> {
                crate::Signed::Positive(self).opt_checked_sub(rhs)
            }
        }

        impl opt_ops::OptionSaturatingSub<crate::Signed<$type>> for $type {
            type Output = crate::Signed<$type>;
            fn opt_saturating_sub(
                self,
                rhs: crate::Signed<$type>
            ) -> Option<Self::Output> {
                crate::Signed::Positive(self).opt_saturating_sub(rhs)
            }
        }
    };
);

macro_rules! impl_signed_div_mul(
    (u64) => {
        impl_signed_div_mul!(u64, u64, i64, |val: u64| val);
        impl_signed_extra_div_mul!(u64, i64);
        impl_signed_div_mul_trait!(u64, u64, i64, |val: u64| val);
    };

    (usize) => {
        impl_signed_div_mul!(usize, usize, isize, |val: usize| val);
        impl_signed_extra_div_mul!(usize, isize);
        // `MulDiv` not available for usize
    };

    (u32) => {
        impl_signed_div_mul!(u32, u32, i32, |val: u32| val);
        impl_signed_extra_div_mul!(u32, i32);
        impl_signed_div_mul_trait!(u32, u32, i32, |val: u32| val);
    };

    ($new_type:ty, u64) => {
        impl_signed_div_mul!($new_type, u64, i64, |val: $new_type| *val);
        impl_signed_extra_div_mul!($new_type, u64, i64);
        impl_signed_div_mul_trait!($new_type, u64, i64, |val: $new_type| *val);
    };

    ($new_type:ty, u32) => {
        impl_signed_div_mul!($new_type, u32, i32, |val: $new_type| *val);
        impl_signed_extra_div_mul!($new_type, u32, i32);
        impl_signed_div_mul_trait!($new_type, u32, i32, |val: $new_type| *val);
    };

    ($type:ty, $inner_type:ty, $signed_rhs:ty, $into_inner:expr) => {
        impl crate::Signed<$type> {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_div(self, rhs:$signed_rhs) -> Option<Self> {
                use crate::UnsignedIntoSigned;
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        if rhs.is_positive() {
                            lhs.checked_div(rhs as $inner_type).map(<$type>::into_positive)
                        } else {
                            lhs.checked_div(-rhs as $inner_type).map(<$type>::into_negative)
                        }
                    }
                    Negative(lhs) => {
                        if rhs.is_positive() {
                            lhs.checked_div(rhs as $inner_type).map(<$type>::into_negative)
                        } else {
                            lhs.checked_div(-rhs as $inner_type).map(<$type>::into_positive)
                        }
                    }
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_div_unsigned(self, rhs:$inner_type) -> Option<Self> {
                use crate::UnsignedIntoSigned;
                use crate::Signed::*;
                match self {
                    Positive(lhs) => lhs.checked_div(rhs).map(<$type>::into_positive),
                    Negative(lhs) => lhs.checked_div(rhs).map(<$type>::into_negative),
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_rem(self, rhs:$signed_rhs) -> Option<Self> {
                use crate::UnsignedIntoSigned;
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        if rhs.is_positive() {
                            lhs.checked_rem(rhs as $inner_type).map(<$type>::into_positive)
                        } else {
                            lhs.checked_rem(-rhs as $inner_type).map(<$type>::into_positive)
                        }
                    }
                    Negative(lhs) => {
                        if rhs.is_positive() {
                            lhs.checked_rem(rhs as $inner_type).map(<$type>::into_negative)
                        } else {
                            lhs.checked_rem(-rhs as $inner_type).map(<$type>::into_negative)
                        }
                    }
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_rem_unsigned(self, rhs:$inner_type) -> Option<Self> {
                use crate::UnsignedIntoSigned;
                use crate::Signed::*;
                match self {
                    Positive(lhs) => lhs.checked_rem(rhs).map(<$type>::into_positive),
                    Negative(lhs) => lhs.checked_rem(rhs).map(<$type>::into_negative),
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_mul(self, rhs:$signed_rhs) -> Option<Self> {
                use crate::UnsignedIntoSigned;
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        if rhs.is_positive() {
                            lhs.checked_mul(rhs as $inner_type).map(<$type>::into_positive)
                        } else {
                            lhs.checked_mul(-rhs as $inner_type).map(<$type>::into_negative)
                        }
                    }
                    Negative(lhs) => {
                        if rhs.is_positive() {
                            lhs.checked_mul(rhs as $inner_type).map(<$type>::into_negative)
                        } else {
                            lhs.checked_mul(-rhs as $inner_type).map(<$type>::into_positive)
                        }
                    }
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn checked_mul_unsigned(self, rhs:$inner_type) -> Option<Self> {
                use crate::UnsignedIntoSigned;
                use crate::Signed::*;
                match self {
                    Positive(lhs) => lhs.checked_mul(rhs).map(<$type>::into_positive),
                    Negative(lhs) => lhs.checked_mul(rhs).map(<$type>::into_negative),
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_mul(self, rhs:$signed_rhs) -> Self {
                use crate::UnsignedIntoSigned;
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        if rhs.is_positive() {
                            lhs.saturating_mul(rhs as $inner_type).into_positive()
                        } else {
                            lhs.saturating_mul(-rhs as $inner_type).into_negative()
                        }
                    }
                    Negative(lhs) => {
                        if rhs.is_positive() {
                            lhs.saturating_mul(rhs as $inner_type).into_negative()
                        } else {
                            lhs.saturating_mul(-rhs as $inner_type).into_positive()
                        }
                    }
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            pub fn saturating_mul_unsigned(self, rhs:$inner_type) -> Self {
                use crate::UnsignedIntoSigned;
                use crate::Signed::*;
                match self {
                    Positive(lhs) => lhs.saturating_mul(rhs).into_positive(),
                    Negative(lhs) => lhs.saturating_mul(rhs).into_negative(),
                }
            }
        }

        impl std::ops::Div<$signed_rhs> for crate::Signed<$type> {
            type Output = crate::Signed<$type>;

            fn div(self, rhs: $signed_rhs) -> crate::Signed<$type> {
                self.checked_div(rhs).expect("division overflowed")
            }
        }

        impl std::ops::DivAssign<$signed_rhs> for crate::Signed<$type> {
            fn div_assign(&mut self, rhs: $signed_rhs) {
                *self = std::ops::Div::div(*self, rhs);
            }
        }

        impl std::ops::Div<$inner_type> for crate::Signed<$type> {
            type Output = crate::Signed<$type>;

            fn div(self, rhs: $inner_type) -> crate::Signed<$type> {
                self.checked_div_unsigned(rhs).expect("division overflowed")
            }
        }

        impl std::ops::DivAssign<$inner_type> for crate::Signed<$type> {
            fn div_assign(&mut self, rhs: $inner_type) {
                *self = std::ops::Div::div(*self, rhs);
            }
        }

        impl std::ops::Rem<$signed_rhs> for crate::Signed<$type> {
            type Output = crate::Signed<$type>;

            fn rem(self, rhs: $signed_rhs) -> crate::Signed<$type> {
                self.checked_rem(rhs).expect("division overflowed")
            }
        }

        impl std::ops::RemAssign<$signed_rhs> for crate::Signed<$type> {
            fn rem_assign(&mut self, rhs: $signed_rhs) {
                *self = std::ops::Rem::rem(*self, rhs);
            }
        }

        impl std::ops::Rem<$inner_type> for crate::Signed<$type> {
            type Output = crate::Signed<$type>;

            fn rem(self, rhs: $inner_type) -> crate::Signed<$type> {
                self.checked_rem_unsigned(rhs).expect("division overflowed")
            }
        }

        impl std::ops::RemAssign<$inner_type> for crate::Signed<$type> {
            fn rem_assign(&mut self, rhs: $inner_type) {
                *self = std::ops::Rem::rem(*self, rhs);
            }
        }

        impl std::ops::Mul<$signed_rhs> for crate::Signed<$type> {
            type Output = crate::Signed<$type>;

            fn mul(self, rhs: $signed_rhs) -> crate::Signed<$type> {
                self.checked_mul(rhs).expect("multiplication overflowed")
            }
        }

        impl std::ops::MulAssign<$signed_rhs> for crate::Signed<$type> {
            fn mul_assign(&mut self, rhs: $signed_rhs) {
                *self = std::ops::Mul::mul(*self, rhs);
            }
        }

        impl std::ops::Mul<$inner_type> for crate::Signed<$type> {
            type Output = crate::Signed<$type>;

            fn mul(self, rhs: $inner_type) -> crate::Signed<$type> {
                self.checked_mul_unsigned(rhs).expect("multiplication overflowed")
            }
        }

        impl std::ops::MulAssign<$inner_type> for crate::Signed<$type> {
            fn mul_assign(&mut self, rhs: $inner_type) {
                *self = std::ops::Mul::mul(*self, rhs);
            }
        }

        impl opt_ops::OptionCheckedDiv<$signed_rhs> for crate::Signed<$type> {
            type Output = Self;
            fn opt_checked_div(self, rhs: $signed_rhs) -> Result<Option<Self::Output>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.checked_div(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedMul<$signed_rhs> for crate::Signed<$type> {
            type Output = Self;
            fn opt_checked_mul(self, rhs: $signed_rhs) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.checked_mul(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingMul<$signed_rhs> for crate::Signed<$type> {
            type Output = Self;
            fn opt_saturating_mul(self, rhs: $signed_rhs) -> Option<Self::Output> {
                Some(self.saturating_mul(rhs))
            }
        }

        impl opt_ops::OptionCheckedRem<$signed_rhs> for crate::Signed<$type> {
            type Output = Self;
            fn opt_checked_rem(self, rhs: $signed_rhs) -> Result<Option<Self::Output>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.checked_rem(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedDiv<$inner_type> for crate::Signed<$type> {
            type Output = Self;
            fn opt_checked_div(self, rhs: $inner_type) -> Result<Option<Self::Output>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.checked_div_unsigned(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionCheckedMul<$inner_type> for crate::Signed<$type> {
            type Output = Self;
            fn opt_checked_mul(self, rhs: $inner_type) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.checked_mul_unsigned(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }

        impl opt_ops::OptionSaturatingMul<$inner_type> for crate::Signed<$type> {
            type Output = Self;
            fn opt_saturating_mul(self, rhs: $inner_type) -> Option<Self::Output> {
                Some(self.saturating_mul_unsigned(rhs))
            }
        }

        impl opt_ops::OptionCheckedRem<$inner_type> for crate::Signed<$type> {
            type Output = Self;
            fn opt_checked_rem(self, rhs: $inner_type) -> Result<Option<Self::Output>, opt_ops::Error> {
                if rhs == 0 {
                    return Err(opt_ops::Error::DivisionByZero);
                }
                self.checked_rem_unsigned(rhs)
                    .ok_or(opt_ops::Error::Overflow)
                    .map(Some)
            }
        }
    };
);

macro_rules! impl_signed_extra_div_mul(
    ($type:ty, $signed:ty) => {
        impl std::ops::Div for crate::Signed<$type> {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                match rhs {
                    crate::Signed::Positive(rhs) => self.div(rhs),
                    crate::Signed::Negative(rhs) => std::ops::Neg::neg(self.div(rhs)),
                }
            }
        }

        impl std::ops::Rem for crate::Signed<$type> {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self {
                self.rem(rhs.abs())
            }
        }

        impl opt_ops::OptionCheckedDiv for crate::Signed<$type> {
            type Output = Self;
            fn opt_checked_div(self, rhs: Self) -> Result<Option<Self::Output>, opt_ops::Error> {
                match rhs {
                    crate::Signed::Positive(rhs) => self.opt_checked_div(rhs),
                    crate::Signed::Negative(rhs) => {
                        self.opt_checked_div(rhs)
                            .map(|res| res.map(std::ops::Neg::neg))
                    }
                }
            }
        }

        impl opt_ops::OptionCheckedRem for crate::Signed<$type> {
            type Output = Self;
            fn opt_checked_rem(self, rhs: Self) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.opt_checked_rem(rhs.abs())
            }
        }
    };
    ($new_type:ty, $inner_type:ty, $signed_innner_type:ty) => {
        impl std::ops::Div for crate::Signed<$new_type> {
            type Output = crate::Signed<$inner_type>;

            fn div(self, rhs: Self) -> Self::Output {
                self.into_inner_signed().div(rhs.into_inner_signed())
            }
        }

        impl std::ops::Rem for crate::Signed<$new_type> {
            type Output = crate::Signed<$new_type>;

            fn rem(self, rhs: Self) -> Self::Output {
                self.rem(rhs.abs().0)
            }
        }

        impl std::ops::Mul<crate::Signed<$new_type>> for $inner_type {
            type Output = crate::Signed<$new_type>;

            fn mul(self, rhs: crate::Signed<$new_type>) -> Self::Output {
                rhs.mul(self)
            }
        }

        impl std::ops::Mul<crate::Signed<$new_type>> for $signed_innner_type {
            type Output = crate::Signed<$new_type>;

            fn mul(self, rhs: crate::Signed<$new_type>) -> Self::Output {
                rhs.mul(self)
            }
        }

        impl opt_ops::OptionCheckedDiv for crate::Signed<$new_type> {
            type Output = crate::Signed<$inner_type>;
            fn opt_checked_div(self, rhs: Self) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.into_inner_signed().opt_checked_div(rhs.into_inner_signed())
            }
        }

        impl opt_ops::OptionCheckedRem for crate::Signed<$new_type> {
            type Output = crate::Signed<$inner_type>;
            fn opt_checked_rem(self, rhs: Self) -> Result<Option<Self::Output>, opt_ops::Error> {
                self.into_inner_signed().opt_checked_rem(rhs.abs().0)
            }
        }

        impl opt_ops::OptionCheckedMul<crate::Signed<$new_type>> for $signed_innner_type {
            type Output = crate::Signed<$new_type>;
            fn opt_checked_mul(self, rhs: crate::Signed<$new_type>) -> Result<Option<Self::Output>, opt_ops::Error> {
                rhs.opt_checked_mul(self)
            }
        }

        impl opt_ops::OptionSaturatingMul<crate::Signed<$new_type>> for $signed_innner_type {
            type Output = crate::Signed<$new_type>;
            fn opt_saturating_mul(self, rhs: crate::Signed<$new_type>) -> Option<Self::Output> {
                rhs.opt_saturating_mul(self)
            }
        }

        impl opt_ops::OptionCheckedMul<crate::Signed<$new_type>> for $inner_type {
            type Output = crate::Signed<$new_type>;
            fn opt_checked_mul(self, rhs: crate::Signed<$new_type>) -> Result<Option<Self::Output>, opt_ops::Error> {
                rhs.opt_checked_mul(self)
            }
        }

        impl opt_ops::OptionSaturatingMul<crate::Signed<$new_type>> for $inner_type {
            type Output = crate::Signed<$new_type>;
            fn opt_saturating_mul(self, rhs: crate::Signed<$new_type>) -> Option<Self::Output> {
                rhs.opt_saturating_mul(self)
            }
        }
    };
);

macro_rules! impl_signed_div_mul_trait(
    ($type:ty, $inner_type:ty, $signed_rhs:ty, $into_inner:expr) => {
        impl crate::Signed<$type> {
            fn signed_from_inner(val: $inner_type, sign: $signed_rhs) -> Option<crate::Signed<$type>> {
                skip_assert_initialized!();
                if sign.is_positive() {
                    Self::positive_from_inner(val)
                } else {
                    Self::negative_from_inner(val)
                }
            }

            fn positive_from_inner(val: $inner_type) -> Option<crate::Signed<$type>> {
                skip_assert_initialized!();
                use crate::UnsignedIntoSigned;
                <$type>::try_from(val).ok().map(<$type>::into_positive)
            }

            fn negative_from_inner(val: $inner_type) -> Option<crate::Signed<$type>> {
                skip_assert_initialized!();
                use crate::UnsignedIntoSigned;
                <$type>::try_from(val).ok().map(<$type>::into_negative)
            }
        }

        impl muldiv::MulDiv<$signed_rhs> for crate::Signed<$type> {
            type Output = crate::Signed<$type>;

            fn mul_div_floor(self, num: $signed_rhs, denom: $signed_rhs) -> Option<Self::Output> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        $into_inner(lhs)
                            .mul_div_floor(num.abs() as $inner_type, denom.abs() as $inner_type)
                            .and_then(|val| Self::signed_from_inner(val, num.signum() * denom.signum()))
                    }
                    Negative(lhs) => {
                        $into_inner(lhs)
                            .mul_div_floor(num.abs() as $inner_type, denom.abs() as $inner_type)
                            .and_then(|val| Self::signed_from_inner(val, -num.signum() * denom.signum()))
                    }
                }
            }

            fn mul_div_round(self, num: $signed_rhs, denom: $signed_rhs) -> Option<Self::Output> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        $into_inner(lhs)
                            .mul_div_round(num.abs() as $inner_type, denom.abs() as $inner_type)
                            .and_then(|val| Self::signed_from_inner(val, num.signum() * denom.signum()))
                    }
                    Negative(lhs) => {
                        $into_inner(lhs)
                            .mul_div_round(num.abs() as $inner_type, denom.abs() as $inner_type)
                            .and_then(|val| Self::signed_from_inner(val, -num.signum() * denom.signum()))
                    }
                }
            }

            fn mul_div_ceil(self, num: $signed_rhs, denom: $signed_rhs) -> Option<Self::Output> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        $into_inner(lhs)
                            .mul_div_ceil(num.abs() as $inner_type, denom.abs() as $inner_type)
                            .and_then(|val| Self::signed_from_inner(val, num.signum() * denom.signum()))
                    }
                    Negative(lhs) => {
                        $into_inner(lhs)
                            .mul_div_ceil(num.abs() as $inner_type, denom.abs() as $inner_type)
                            .and_then(|val| Self::signed_from_inner(val, -num.signum() * denom.signum()))
                    }
                }
            }
        }

        impl muldiv::MulDiv<$inner_type> for crate::Signed<$type> {
            type Output = crate::Signed<$type>;

            fn mul_div_floor(self, num: $inner_type, denom: $inner_type) -> Option<Self::Output> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        $into_inner(lhs)
                            .mul_div_floor(num, denom)
                            .and_then(Self::positive_from_inner)
                    }
                    Negative(lhs) => {
                        $into_inner(lhs)
                            .mul_div_floor(num, denom)
                            .and_then(Self::negative_from_inner)
                    }
                }
            }

            fn mul_div_round(self, num: $inner_type, denom: $inner_type) -> Option<Self::Output> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        $into_inner(lhs)
                            .mul_div_round(num, denom)
                            .and_then(Self::positive_from_inner)
                    }
                    Negative(lhs) => {
                        $into_inner(lhs)
                            .mul_div_round(num, denom)
                            .and_then(Self::negative_from_inner)
                    }
                }
            }

            fn mul_div_ceil(self, num: $inner_type, denom: $inner_type) -> Option<Self::Output> {
                use crate::Signed::*;
                match self {
                    Positive(lhs) => {
                        $into_inner(lhs)
                            .mul_div_ceil(num, denom)
                            .and_then(Self::positive_from_inner)
                    }
                    Negative(lhs) => {
                        $into_inner(lhs)
                            .mul_div_ceil(num, denom)
                            .and_then(Self::negative_from_inner)
                    }
                }
            }
        }
    };
);

macro_rules! impl_format_value_traits(
    ($name:ident, $format:ident, $format_value:ident, $inner_type:ty) => {
        impl FormattedValue for Option<$name> {
            type FullRange = Option<$name>;

            fn default_format() -> Format {
                Format::$format
            }

            fn format(&self) -> Format {
                Format::$format
            }

            fn is_some(&self) -> bool {
                Option::is_some(self)
            }

            unsafe fn into_raw_value(self) -> i64 {
                IntoGlib::into_glib(self) as i64
            }
        }

        impl FormattedValueFullRange for Option<$name> {
            unsafe fn from_raw(format: Format, value: i64) -> Option<$name> {
                debug_assert_eq!(format, Format::$format);
                FromGlib::from_glib(value as u64)
            }
        }

        impl FormattedValueNoneBuilder for Option<$name> {
            fn none() -> Option<$name> {
                None
            }
        }

        impl From<Option<$name>> for GenericFormattedValue {
            fn from(v: Option<$name>) -> Self {
                skip_assert_initialized!();
                Self::$format_value(v)
            }
        }

        impl From<$name> for GenericFormattedValue {
            fn from(v: $name) -> Self {
                skip_assert_initialized!();
                Self::$format_value(Some(v))
            }
        }

        impl FormattedValue for $name {
            type FullRange = Option<$name>;

            fn default_format() -> Format {
                Format::$format
            }

            fn format(&self) -> Format {
                Format::$format
            }

            fn is_some(&self) -> bool {
                true
            }

            unsafe fn into_raw_value(self) -> i64 {
                IntoGlib::into_glib(self) as i64
            }
        }

        impl SpecificFormattedValue for Option<$name> {}
        impl SpecificFormattedValueFullRange for Option<$name> {}
        impl SpecificFormattedValue for $name {}
        impl FormattedValueIntrinsic for $name {}
        impl SpecificFormattedValueIntrinsic for $name {}

        impl TryFrom<GenericFormattedValue> for Option<$name> {
            type Error = FormattedValueError;

            fn try_from(v: GenericFormattedValue) -> Result<Option<$name>, Self::Error> {
                skip_assert_initialized!();
                if let GenericFormattedValue::$format_value(v) = v {
                    Ok(v)
                } else {
                    Err(FormattedValueError(v.format()))
                }
            }
        }

        impl TryFrom<$inner_type> for $name {
            type Error = GlibNoneError;

            fn try_from(v: $inner_type) -> Result<$name, GlibNoneError> {
                skip_assert_initialized!();
                unsafe { Self::try_from_glib(v as i64) }
            }
        }

        impl TryFromGlib<i64> for $name {
            type Error = GlibNoneError;
            #[inline]
            unsafe fn try_from_glib(val: i64) -> Result<Self, GlibNoneError> {
                skip_assert_initialized!();
                <$name as TryFromGlib<u64>>::try_from_glib(val as u64)
            }
        }

        impl std::ops::Deref for $name {
            type Target = $inner_type;

            fn deref(&self) -> &$inner_type {
                &self.0
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $inner_type {
                &mut self.0
            }
        }

        impl AsRef<$inner_type> for $name {
            fn as_ref(&self) -> &$inner_type {
                &self.0
            }
        }

        impl AsMut<$inner_type> for $name {
            fn as_mut(&mut self) -> &mut $inner_type {
                &mut self.0
            }
        }
    };
);

macro_rules! option_glib_newtype_from_to {
    ($type_:ident, $none_value:expr) => {
        #[doc(hidden)]
        impl IntoGlib for $type_ {
            type GlibType = u64;

            fn into_glib(self) -> u64 {
                self.0
            }
        }

        #[doc(hidden)]
        impl OptionIntoGlib for $type_ {
            const GLIB_NONE: u64 = $none_value;
        }

        #[doc(hidden)]
        impl TryFromGlib<u64> for $type_ {
            type Error = GlibNoneError;
            #[inline]
            unsafe fn try_from_glib(val: u64) -> Result<Self, GlibNoneError> {
                skip_assert_initialized!();
                if val == $none_value {
                    return Err(GlibNoneError);
                }

                Ok($type_(val))
            }
        }
    };
}

// FIXME we could automatically build `$displayable_name` and
// `$displayable_option_name` if `concat_idents!` was stable.
// See: https://doc.rust-lang.org/std/macro.concat_idents.html
macro_rules! glib_newtype_display {
    ($name:ident, $displayable_name:ident, $unit:expr) => {
        pub struct $displayable_name($name);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::fmt::Write;

                std::fmt::Display::fmt(&self.0, f)?;
                f.write_char(' ')?;
                f.write_str($unit)
            }
        }

        impl crate::utils::Displayable for $name {
            type DisplayImpl = $name;

            fn display(self) -> $name {
                self
            }
        }
    };

    ($name:ident, $displayable_name:ident, $displayable_option_name:ident, $unit:expr) => {
        glib_newtype_display!($name, $displayable_name, $unit);

        pub struct $displayable_option_name(Option<$name>);

        impl std::fmt::Display for $displayable_option_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if let Some(val) = self.0.as_ref() {
                    std::fmt::Display::fmt(val, f)
                } else {
                    f.write_str("undef. ")?;
                    f.write_str($unit)
                }
            }
        }

        impl crate::utils::Displayable for Option<$name> {
            type DisplayImpl = $displayable_option_name;

            fn display(self) -> Self::DisplayImpl {
                $displayable_option_name(self)
            }
        }
    };
}
