use std::io::Write;
use rmp_serde::encode;
use serde::Serialize;

pub fn serialize<T, W>(value: T, writer: W) -> Result<(), encode::Error>
where
    T: Serialize,
    W: Write,
{
    let mut ser = rmp_serde::Serializer::new(writer)
        .with_struct_map();

    value.serialize(&mut ser)
}

pub fn serialize_to_vec<T: Serialize>(value: T) -> Result<Vec<u8>, encode::Error> {
    let mut bytes = Vec::new();
    serialize(value, &mut bytes)?;
    Ok(bytes)
}
