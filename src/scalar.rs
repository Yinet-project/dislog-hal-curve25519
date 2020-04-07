use dislog_hal::ScalarNumber;
use crate::PointInner;

pub struct ScalarInner {
    data: curve25519_dalek::scalar::Scalar,
}

impl ScalarInner {
    fn new(scalar: curve25519_dalek::scalar::Scalar) -> Self {
        Self { data: scalar }
    }
}

impl ScalarNumber for ScalarInner {
    type Point = PointInner;

    const ZERO: Self = Self {
        data: curve25519_dalek::scalar::Scalar::zero(),
    };
    const ONE: Self = Self {
        data: curve25519_dalek::scalar::Scalar::one(),
    };

    fn add(self, rhs: Self) -> Self {
        Self {
            data: self.data + rhs.data,
        }
    }

    fn mul(self, rhs: Self) -> ScalarInner {
        ScalarInner {
            data: self.data * rhs.data,
        }
    }

    fn inv(self) -> ScalarInner {
        ScalarInner {
            data: self.data.invert(),
        }
    }

    fn neg(self) -> ScalarInner {
        ScalarInner { data: -self.data }
    }
}

