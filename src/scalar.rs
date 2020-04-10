use crate::PointInner;
use core::fmt::Debug;
use dislog_hal::Bytes;
use dislog_hal::ScalarNumber;

pub struct ScalarInner {
    data: curve25519_dalek::scalar::Scalar,
}

impl ScalarInner {
    pub fn new(scalar: [u8; 32]) -> Self {
        Self {
            data: curve25519_dalek::scalar::Scalar::from_bytes_mod_order(scalar),
        }
    }
    pub(crate) fn setInner(scalar: curve25519_dalek::scalar::Scalar) -> Self {
        Self { data: scalar }
    }
    pub(crate) fn getInner(&self) -> curve25519_dalek::scalar::Scalar {
        self.data
    }
}

impl Bytes for ScalarInner {
    type BytesType = [u8; 32];
    fn from_bytes(bytes: Self::BytesType) -> Self {
        Self {
            data: curve25519_dalek::scalar::Scalar::from_bytes_mod_order(bytes),
        }
    }

    fn to_bytes(&self) -> Self::BytesType {
        self.data.to_bytes()
    }
}

impl Clone for ScalarInner {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl Copy for ScalarInner {}

impl PartialEq for ScalarInner {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.data.eq(&other.data)
    }
}

impl Debug for ScalarInner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(
            f,
            "Scalar{{\n\tbytes: {:?},\n\torder: {:?},\n}}",
            &self.data,
            curve25519_dalek::constants::BASEPOINT_ORDER.to_bytes()
        )
    }
}

impl ScalarNumber for ScalarInner {
    type Point = PointInner;

    fn order() -> Self {
        Self {
            data: curve25519_dalek::constants::BASEPOINT_ORDER,
        }
    }

    fn zero() -> Self {
        Self {
            data: curve25519_dalek::scalar::Scalar::zero(),
        }
    }

    fn one() -> Self {
        Self {
            data: curve25519_dalek::scalar::Scalar::one(),
        }
    }

    fn add(&self, rhs: &ScalarInner) -> ScalarInner {
        Self {
            data: &self.data + &rhs.data,
        }
    }

    fn mul(&self, rhs: &Self) -> Self {
        Self {
            data: &self.data * &rhs.data,
        }
    }

    fn inv(&self) -> Self {
        Self {
            data: self.data.invert(),
        }
    }

    fn neg(&self) -> Self {
        Self { data: -&self.data }
    }
}
