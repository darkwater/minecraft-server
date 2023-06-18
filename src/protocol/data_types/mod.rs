use bytes::Bytes;
use serde::{Serialize, Deserialize};

mod varint;
mod position;
mod string;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Boolean(pub bool);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Byte(pub i8);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnsignedByte(pub u8);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Short(pub i16);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnsignedShort(pub u16);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Int(pub i32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Long(pub i64);
pub struct Float(pub f32);
pub struct Double(pub f64);
#[derive(Debug)]
pub struct McString<const N: usize>(pub String);
pub struct Identifier(pub String);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarInt(pub i32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarLong(pub i64);
pub struct EntityMetadata();
pub struct Slot();
pub struct NbtTag();
pub struct Position { x: i32, z: i32, y: i16 }
pub struct Angle(pub u8);
pub struct Uuid(pub u128);
pub struct Optional<T>(pub Option<T>);
pub struct Array<T>(pub Vec<T>);
#[derive(Debug)]
pub struct Enum<T>(pub T);
pub struct ByteArray(pub Bytes);

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub text: String,
}
