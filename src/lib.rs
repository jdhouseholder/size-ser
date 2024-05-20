//! Seralizer for `serde` to compute a lower bound for the size of any type that is `Serialize`.
use std::{fmt::Error, mem::size_of};

use paste::paste;
use serde::{ser, Serialize};

type Result<T> = std::result::Result<T, Error>;

pub struct Serializer {
    output: usize,
}

/// Computes a lower bound for the size of T.
pub fn to_size<T>(value: &T) -> Result<usize>
where
    T: Serialize,
{
    let mut serializer = Serializer { output: 0 };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

macro_rules! ser {
    ($($ty:ty),*) => {
        $(
        paste! {
            fn [<serialize_ $ty>](self, _: $ty) -> Result<()> {
                self.output += size_of::<$ty>();
                Ok(())
            }
        }
        )*
    };
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    ser!(bool, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, char);

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output += v.len();
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output += v.len();
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Ok(())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;

    use serde::Serialize;

    #[test]
    fn vec_u8_works() {
        let v = vec![1u8, 2, 3, 4, 5];

        let got = to_size(&v).unwrap();

        assert_eq!(got, v.len() * size_of::<u8>());
    }

    #[test]
    fn vec_u64_works() {
        let v = vec![1u64, 2, 3, 4, 5];

        let got = to_size(&v).unwrap();

        assert_eq!(got, v.len() * size_of::<u64>());
    }

    #[test]
    fn struct_works() {
        #[derive(Serialize)]
        struct Value {
            a: f64,
            b: String,
        }

        let v = Value {
            a: 23.,
            b: "cool".to_string(),
        };
        let got = to_size(&v).unwrap();

        let want = size_of::<f64>() + "cool".len();
        assert_eq!(got, want);
    }

    #[test]
    fn map_works() {
        let mut m: HashMap<&str, u128> = HashMap::new();
        m.insert("wow", 23);
        m.insert("ok", 9);

        let got = to_size(&m).unwrap();

        let want = size_of::<u128>() + "wow".len() + size_of::<u128>() + "ok".len();
        assert_eq!(got, want);
    }

    #[test]
    fn map_u128_struct_works() {
        #[derive(Serialize)]
        struct Value {
            a: f64,
            b: String,
        }

        let mut m: HashMap<u128, Value> = HashMap::new();
        m.insert(
            5,
            Value {
                a: 42.,
                b: "Hello world".to_string(),
            },
        );
        m.insert(
            8,
            Value {
                a: 24.,
                b: "world".to_string(),
            },
        );

        let got = to_size(&m).unwrap();

        let want = size_of::<u128>()
            + size_of::<f64>()
            + "Hello world".len()
            + size_of::<u128>()
            + size_of::<f64>()
            + "world".len();
        assert_eq!(got, want);
    }
}
