use crate::EccError;
use crate::PointInner;
use core::fmt::Debug;
use dislog_hal::Bytes;
use dislog_hal::ScalarNumber;
use hex::{FromHex, ToHex};
use rand::RngCore;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::string::String;

pub struct ScalarInner {
    data: curve25519_dalek::scalar::Scalar,
}

impl ScalarInner {
    pub(crate) fn set_inner(scalar: curve25519_dalek::scalar::Scalar) -> Self {
        Self { data: scalar }
    }
    pub(crate) fn get_inner(&self) -> curve25519_dalek::scalar::Scalar {
        self.data
    }
}

impl Bytes for ScalarInner {
    type BytesType = [u8; 32];
    type Error = EccError;
    fn from_bytes(bytes: &[u8]) -> Result<Self, EccError> {
        assert!(bytes.len() == 32 || bytes.len() == 64);
        if bytes.len() == 64 {
            let mut ary = [0u8; 64];
            ary.clone_from_slice(bytes);
            Ok(Self {
                data: curve25519_dalek::scalar::Scalar::from_bytes_mod_order_wide(&ary),
            })
        } else {
            let mut ary = [0u8; 32];
            ary.clone_from_slice(bytes);
            Ok(Self {
                data: curve25519_dalek::scalar::Scalar::from_bytes_mod_order(ary),
            })
        }
    }

    fn to_bytes(&self) -> Self::BytesType {
        self.data.to_bytes()
    }
}

impl Clone for ScalarInner {
    fn clone(&self) -> Self {
        Self { data: self.data }
    }
}

impl Copy for ScalarInner {}

impl PartialEq for ScalarInner {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
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

    fn random<R: RngCore>(rng: &mut R) -> Self {
        let mut input = [0u8; 32];
        rng.fill_bytes(&mut input);

        loop {
            if let Ok(ret) = Self::from_bytes(&input) {
                if ret != Self::zero() {
                    return ret;
                }
            }
        }
    }

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
            data: self.data + rhs.data,
        }
    }

    fn mul(&self, rhs: &Self) -> Self {
        Self {
            data: self.data * rhs.data,
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

impl Serialize for ScalarInner {
    fn serialize<SE>(&self, serializer: SE) -> Result<SE::Ok, SE::Error>
    where
        SE: Serializer,
    {
        serializer.serialize_str(&self.to_bytes().encode_hex_upper::<String>())
    }
}

impl<'de> Deserialize<'de> for ScalarInner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let d_str = String::deserialize(deserializer)
            .map_err(|_| serde::de::Error::custom(format_args!("invalid hex string")))?;
        let d_byte = Vec::<u8>::from_hex(d_str)
            .map_err(|_| serde::de::Error::custom(format_args!("invalid hex string")))?;
        ScalarInner::from_bytes(d_byte.as_slice())
            .map_err(|_| serde::de::Error::custom(format_args!("invalid hex string")))
    }
}
