use super::super::prime;
use std::fmt;
use super::num::BigInt;

pub trait Point
   : fmt::Debug + fmt::Display + fmt::LowerHex + fmt::UpperHex + Clone {
   // fn point_at_infinity() -> Self;
   // fn double() -> Self;
}

pub trait PointCalculation<Curve: prime::ECCurve>: Point {
   fn point_addition(&Curve, &Self, &Self) -> Self;

   fn point_subtraction(&Curve, &Self, &Self) -> Self;

   fn point_doublation(&Curve, &Self) -> Self;

   fn point_multipication(&Curve, &Self, BigInt) -> Self;
}

pub trait PointFrom<P: Point>: Point {
   /// `p`: FIELD
   fn convert_from(point: &P, p: &BigInt) -> Self;
}

pub trait PointInto<T: Point>: Sized + Point {
   /// Performs the conversion.
   fn convert_into(&self, p: &BigInt) -> T;
}

impl<T, U> PointInto<U> for T
where
   U: PointFrom<T>,
   T: Point,
{
   fn convert_into(&self, i: &BigInt) -> U { U::convert_from(self, &i) }
}

pub mod affine;
pub mod jacobian;
pub mod standard_projective;

pub use self::affine::AffineCoordinates;
pub use self::jacobian::JacobianCoordinates;
pub use self::standard_projective::StandardProjectiveCoordinates;

mod errors;
pub use self::errors::convertion::ConvertionError;
