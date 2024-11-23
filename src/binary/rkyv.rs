use crate::{Decoder, Encoder};
use rkyv::{rancor, Archive, Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use rkyv::api::high::{HighDeserializer, HighSerializer, HighValidator};
use rkyv::bytecheck::CheckBytes;
use rkyv::ser::allocator::ArenaHandle;
use rkyv::util::AlignedVec;
use rkyv::validation::Validator;

type RkyvSerializer<'a> =
HighSerializer<AlignedVec, ArenaHandle<'a>, rancor::Error>;
type RkyvDeserializer = HighDeserializer<rancor::Error>;
type RkyvValidator<'a> = HighValidator<'a, rancor::Error>;

/// A codec that relies on `rkyv` to encode data in the msgpack format.
///
/// This is only available with the **`rkyv` feature** enabled.
pub struct RkyvCodec;

impl<T> Encoder<T> for RkyvCodec
where
    T: Archive + for<'a> Serialize<RkyvSerializer<'a>>,
{
    type Error = rancor::Error;
    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        Ok(rkyv::to_bytes::<rancor::Error>(val)?.into_vec())
    }
}

impl<T> Decoder<T> for RkyvCodec
where
    T: Archive,
    T::Archived:
    Deserialize<T, RkyvDeserializer>
    + for<'a> CheckBytes<RkyvValidator<'a>>,
{
    type Error = rancor::Error;
    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        rkyv::from_bytes::<T, rancor::Error>(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rkyv_codec() {
        #[derive(Clone, Debug, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        struct Test {
            s: String,
            i: i32,
        }
        let t = Test {
            s: String::from("party time 🎉"),
            i: 42,
        };
        let enc = RkyvCodec::encode(&t).unwrap();
        let dec: Test = RkyvCodec::decode(&enc).unwrap();
        assert_eq!(dec, t);
    }
}
