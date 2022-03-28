use arrayvec::ArrayVec;
use async_graphql::*;
use bamboo_rs_core_ed25519_yasmf::{entry::decode, entry::into_owned, Entry};

pub struct BambooEntry(Entry<ArrayVec<[u8; 32]>, ArrayVec<[u8; 64]>>);

#[Scalar]
impl ScalarType for BambooEntry {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            let bytes = base64::decode(value)?;
            let entry = decode(&bytes)?;
            let entry = into_owned(&entry);
            Ok(Self(entry))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        let mut buf = Vec::new();
        self.0
            .encode_write(&mut buf)
            .expect("unable to encode entry into bytes");
        let encoded = base64::encode(&buf);
        Value::String(encoded)
    }
}
