extern crate rustc_serialize;

use std::marker::PhantomData;
use std::ops::Deref;

use serde;

use self::rustc_serialize::json;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RustcSerializeWrapper<V> {
    value: V,
}

impl<V> RustcSerializeWrapper<V> {
    pub fn new(value: V) -> Self {
        RustcSerializeWrapper {
            value: value,
        }
    }
}

impl<V> serde::Deserialize for RustcSerializeWrapper<V>
    where V: self::rustc_serialize::Decodable,
{
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: serde::Deserializer,
    {
        deserializer.deserialize(WrappedTypeVisitor { _marker: PhantomData })
    }
}

struct WrappedTypeVisitor<T> {
    _marker: PhantomData<T>,
}

impl<V> serde::de::Visitor for WrappedTypeVisitor<V>
    where V: self::rustc_serialize::Decodable,
{
    type Value = RustcSerializeWrapper<V>;

    fn visit_str<E>(&mut self, value: &str) -> Result<RustcSerializeWrapper<V>, E>
        where E: serde::de::Error,
    {
        Ok(RustcSerializeWrapper { value: json::decode(value).unwrap() })
    }
}

impl<V> serde::Serialize for RustcSerializeWrapper<V>
    where V: self::rustc_serialize::Encodable,
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer,
    {
        serializer.serialize_str(json::encode(&self.value).unwrap().as_ref())
    }
}

impl<T> Deref for RustcSerializeWrapper<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}
