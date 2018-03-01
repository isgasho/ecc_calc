extern crate num;

use self::num::BigUint;
use self::num::bigint::ParseBigIntError;
use self::num::Num;
use self::num::pow;

use super::{Point, PointFrom, PointInto};

use std::ops::Add;
use std::cmp::PartialEq;
use std::marker::Sized;
use std::fmt;
use std::convert::TryFrom;

use super::affine::AffinePoint;
use super::Infinity;

#[derive(Debug, Clone)]
/// Jacobian Coordinates are used to represent elliptic curve points on prime curves
/// `y^2 = x^3 + ax + b`.
pub struct JacobianPoint {
   pub x: BigUint,
   pub y: BigUint,
   pub z: BigUint,
}

impl JacobianPoint {
   // TODO: Remove this and use struct `ECCurve`'s value.
   fn a() -> BigUint {
      return BigUint::from_str_radix(
         "FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFC",
         16,
      ).unwrap();
   }

   /// JacobianPoint::from(Infinity{})
   fn point_at_infinity() -> JacobianPoint {
      JacobianPoint {
         x: BigUint::from(0_usize),
         y: BigUint::from(0_usize),
         z: BigUint::from(0_usize),
      }
   }

   fn double(&self) -> JacobianPoint {
      if self.y == BigUint::from(0_u8) {
         return JacobianPoint::point_at_infinity();
      }

      let s = 4_usize * self.x.clone() * pow(self.y.clone(), 2);
      let m = 3_usize * pow(self.x.clone(), 2) + JacobianPoint::a() * pow(self.z.clone(), 4);
      let x = pow(m.clone(), 2) - 2_usize * s.clone();
      println!("s {}, x {}", s, x);
      let y1 = m.clone() * (s.clone() - x.clone());
      let y2 = 8_usize * pow(self.y.clone(), 4);
      println!("y1 {}, y2 {}", y1, y2);
      let y = y1 - y2;
      let z = 2_usize * self.y.clone() * self.z.clone();

      return JacobianPoint { x, y, z };
   }
}

/* -- Formatter impls -- */
impl fmt::Display for JacobianPoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "JacobianPoint(x: {}, y: {}, z: {})",
         self.x, self.y, self.z
      )
   }
}

impl fmt::LowerHex for JacobianPoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "JacobianPoint(x: {:x}, y: {:x}, z: {:x})",
         self.x, self.y, self.z
      )
   }
}

impl fmt::UpperHex for JacobianPoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "JacobianPoint(x: {:X}, y: {:X}, z: {:X})",
         self.x, self.y, self.z
      )
   }
}

impl fmt::Octal for JacobianPoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "JacobianPoint(x: {:o}, y: {:o}, z: {:o})",
         self.x, self.y, self.z
      )
   }
}
/* -- Formatter impls -- */

impl Point for JacobianPoint {}

/* -- Point Convertion impls -- */
impl PointFrom<AffinePoint> for JacobianPoint {
   fn convert_from(point: &AffinePoint, _i: &BigUint) -> JacobianPoint {
      JacobianPoint {
         x: point.x.clone(),
         y: point.y.clone(),
         z: BigUint::from(0_u8),
      }
   }
}

impl PointFrom<JacobianPoint> for JacobianPoint {
   fn convert_from(point: &JacobianPoint, _i: &BigUint) -> JacobianPoint { point.clone() }
}

impl TryFrom<Infinity> for JacobianPoint {
   type Error = super::ConvertionError;
   fn try_from(_: Infinity) -> Result<Self, Self::Error> {
      return Ok(JacobianPoint::point_at_infinity());
   }
}
/* -- Point Convertion impls -- */

pub trait NewPoint<T, U>
where
   Self: Sized,
{
   type Error;
   fn new(x_str: T, y_str: T, z_str: T, base: U) -> Result<Self, Self::Error>;
}

impl NewPoint<&'static str, u32> for JacobianPoint {
   type Error = ParseBigIntError;

   fn new(s1: &str, s2: &str, s3: &str, base: u32) -> Result<Self, Self::Error> {
      let x = BigUint::from_str_radix(s1, base);
      let y = BigUint::from_str_radix(s2, base);
      let z = BigUint::from_str_radix(s3, base);

      match (x, y, z) {
         (Ok(x), Ok(y), Ok(z)) => Ok(JacobianPoint { x, y, z }),
         _ => Err(ParseBigIntError::Other),
      }
   }
}

impl<'a> Add for &'a JacobianPoint {
   type Output = JacobianPoint;

   fn add(self, other: &JacobianPoint) -> JacobianPoint {
      let u1 = self.x.clone() * pow(other.z.clone(), 2);
      let u2 = other.x.clone() * pow(self.z.clone(), 2);
      let s1 = self.y.clone() * pow(other.z.clone(), 3);
      let s2 = other.y.clone() * pow(self.z.clone(), 3);

      if u1 == u2 {
         if s1 != s2 {
            return JacobianPoint::point_at_infinity();
         } else {
            return self.double();
         }
      }

      let h = u1.clone() - u2.clone();
      let r = s2.clone() - s1.clone();
      let x3 = pow(r.clone(), 2) - pow(h.clone(), 3) - 2_usize * u1.clone() * pow(h.clone(), 2);
      let y3 = r.clone() * (u1 * pow(h.clone(), 2) - x3.clone()) - s1.clone() * pow(h.clone(), 3);
      let z3 = h.clone() * self.z.clone() * other.z.clone();

      return JacobianPoint {
         x: x3,
         y: y3,
         z: z3,
      };
   }
}

impl PartialEq for JacobianPoint {
   fn eq(&self, other: &Self) -> bool {
      let i = BigUint::from(0_u8);
      AffinePoint::convert_from(self, &i) == other.convert_into(&i)
   }
}

#[cfg(test)]
#[allow(unused_qualifications)]
mod tests {
   use super::*;
   use super::super::point::TryPointFrom;
   use super::super::jacobian_point::TryPointFrom as JacobianTry;

   #[test]
   fn inf_plus_inf_is_inf() {
      let a: AffinePoint = AffinePoint::try_from("0", "0", 16).unwrap();
      let b: AffinePoint = AffinePoint::try_from("0", "0", 16).unwrap();
      let r: AffinePoint = AffinePoint::try_from("0", "0", 16).unwrap();

      let a = JacobianPoint::from(&a);
      let b = JacobianPoint::from(&b);
      let r = JacobianPoint::from(&r);

      let z = &a + &b;

      assert!(r == z);
   }

   #[test]
   fn g_plus_inf_is_g() {
      let a: JacobianPoint = JacobianTry::try_from(
         "18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c",
         "8571ff1825885d85d2e88688dd21f3258b4ab8e4ba19e45cddf25357ce95560a",
         "00000000fffffffeffffffffffffffffffffffff000000000000000000000001",
         16,
      ).unwrap();
      let b = JacobianPoint::point_at_infinity();

      let z = &a + &b;

      assert!(a == z);
   }
}