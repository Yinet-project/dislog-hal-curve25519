use crate::EccError;
use crate::ScalarInner;
use core::fmt::Debug;
use curve25519_dalek::traits::Identity;
use dislog_hal::{Bytes, DisLogPoint, Scalar};
use hex::{FromHex, ToHex};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::string::String;

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
    fn from_bytes(bytes: &[u8]) -> Result<Self, EccError> {
        assert_eq!(bytes.len(), 32);
        match curve25519_dalek::edwards::CompressedEdwardsY::from_slice(bytes).decompress() {
            Some(x) => Ok(Self { data: x }),
            None => Err(EccError::ParseError),
        }
    }

    fn to_bytes(&self) -> Self::BytesType {
        *self.data.compress().as_bytes()
    }
}

impl Copy for PointInner {}

impl Clone for PointInner {
    fn clone(&self) -> Self {
        Self { data: self.data }
    }
}

impl PartialEq for PointInner {
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

    fn get_x(&self) -> Scalar<Self::Scalar> {
        let num = [0u8; 32];
        //num.clone_from_slice(&self.data.X.to_bytes()[..]);

        Scalar(ScalarInner::from_bytes(&num).unwrap())
    }

    fn get_y(&self) -> Scalar<Self::Scalar> {
        let num = [0u8; 32];
        //num.clone_from_slice(&self.data.Y.to_bytes()[..]);

        Scalar(ScalarInner::from_bytes(&num).unwrap())
    }
}

impl Serialize for PointInner {
    fn serialize<SE>(&self, serializer: SE) -> Result<SE::Ok, SE::Error>
    where
        SE: Serializer,
    {
        serializer.serialize_str(&self.to_bytes().encode_hex_upper::<String>())
    }
}

impl<'de> Deserialize<'de> for PointInner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let d_str = String::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom(format_args!("invalid hex string")))?;
        let d_byte = Vec::<u8>::from_hex(d_str)
            .map_err(|_| serde::de::Error::custom(format_args!("invalid hex string")))?;
        PointInner::from_bytes(d_byte.as_slice())
            .map_err(|_| serde::de::Error::custom(format_args!("invalid hex string")))
    }
}
