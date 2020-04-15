use crate::ScalarInner;
use core::fmt::Debug;
use curve25519_dalek::traits::Identity;
use dislog_hal::Bytes;
use dislog_hal::DisLogPoint;

#[derive(Debug)]
pub enum EccError {
    ParseError,
}

pub struct PointInner {
    data: curve25519_dalek::edwards::EdwardsPoint,
}

impl PointInner {
    pub fn new(point: curve25519_dalek::edwards::EdwardsPoint) -> Self {
        Self { data: point }
    }
}

impl Debug for PointInner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "Scalar{{\n\tbytes: {:?},\n}}", &self.data)
    }
}

impl Bytes for PointInner {
    type BytesType = [u8; 32];
    type Error = EccError;
    fn from_bytes(bytes: Self::BytesType) -> Result<Self, EccError> {
        match curve25519_dalek::edwards::CompressedEdwardsY::from_slice(&bytes).decompress() {
            Some(x) => Ok(Self { data: x }),
            None => Err(EccError::ParseError),
        }
    }

    fn to_bytes(&self) -> Self::BytesType {
        self.data.compress().as_bytes().clone()
    }
}

impl Copy for PointInner {}

impl Clone for PointInner {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl PartialEq for PointInner {
    fn ne(&self, other: &Self) -> bool {
        !self.data.eq(&other.data)
    }

    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl DisLogPoint for PointInner {
    type Scalar = ScalarInner;

    fn order() -> Self::Scalar {
        Self::Scalar::set_inner(curve25519_dalek::constants::BASEPOINT_ORDER)
    }

    fn zero() -> Self {
        Self {
            data: curve25519_dalek::edwards::EdwardsPoint::identity(),
        }
    }

    fn one() -> Self {
        Self {
            data: curve25519_dalek::constants::ED25519_BASEPOINT_POINT,
        }
    }

    fn generator() -> Self {
        Self {
            data: curve25519_dalek::constants::ED25519_BASEPOINT_POINT,
        }
    }

    fn add(&self, rhs: &Self) -> Self {
        Self {
            data: self.data + rhs.data,
        }
    }

    fn mul(&self, rhs: &Self::Scalar) -> Self {
        Self {
            data: self.data * rhs.get_inner(),
        }
    }

    fn neg(&self) -> Self {
        Self { data: -self.data }
    }
}
