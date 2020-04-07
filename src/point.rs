use dislog_hal::DisLogPoint;
use crate::ScalarInner;

pub struct PointInner {
    data: curve25519_dalek::edwards::EdwardsPoint,
}

impl PointInner {
    fn new(point: curve25519_dalek::edwards::EdwardsPoint) -> Self {
        Self { data: point }
    }
}

impl DisLogPoint for PointInner {
    type Scalar = ScalarInner;

    const ZERO: Self = Self { data: curve25519_dalek::constants::ED25519_BASEPOINT_POINT };
    const ONE: Self = Self { data: curve25519_dalek::constants::ED25519_BASEPOINT_POINT };
    const GENERATOR: Self = Self { data: curve25519_dalek::constants::ED25519_BASEPOINT_POINT };

    fn add(self, rhs: Self) -> Self {
        Self {
            data: self.data + rhs.data,
        }
    }

    fn mul(self, rhs: Self::Scalar) -> Self{
        Self {
            data: self.data + rhs.data,
        }
    }

    fn neg(self) -> Self{
        Self {
            data: self.data
        }
    }

    fn inv(self) -> Self {
        Self {
            data: self.data
        }
    }
}

