use map::Map;
use set::Set;
use vector::Vector;

#[cfg(feature = "immutable")]
use im::{HashMap, HashSet, Vector};
//#[cfg(not(feature = "immutable"))]
//use std::collections::{BTreeSet, BTreeMap};

#[cfg(feature = "immutable")]
use std::hash::Hash;


use std::fmt::{self, Debug};
use std::io;
use std::str;
//use ser::Serializer;
use serde::de::DeserializeOwned;
use error::Error;
use value::ser::Serializer;
use serde::Serialize;
use number::Number;


#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Nil,
    Boolean(bool),
    String(String),
    Char(char),
    Symbol(String),
    Keyword(String),
    Number(Number),
    List(Vector<Value>),
    Vector(Vector<Value>),
    Map(Map<Value, Value>),
    Set(Set<Value>),

    Tagged(String, Box<Value>),
}

/// Prints Value with enum tags in tuple structure
impl Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Nil => formatter.debug_tuple("Nil").finish(),
            Value::Boolean(ref v) => formatter.debug_tuple("Boolean").field(v).finish(),
            Value::String(ref v) => formatter.debug_tuple("String").field(v).finish(),
            Value::Char(ref v) => formatter.debug_tuple("Char").field(v).finish(),
            Value::Symbol(ref v) => formatter.debug_tuple("Symbol").field(v).finish(),
            Value::Keyword(ref v) => formatter.debug_tuple("Keyword").field(v).finish(),
            Value::Number(ref v) => formatter.debug_tuple("Number").field(v).finish(),
            Value::List(ref v) => formatter.debug_tuple("List").field(v).finish(),
            Value::Vector(ref v) => formatter.debug_tuple("Vector").field(v).finish(),
            Value::Map(ref v) => formatter.debug_tuple("Map").field(v).finish(),
            Value::Set(ref v) => formatter.debug_tuple("Set").field(v).finish(),
            Value::Tagged(ref v, ref x) => formatter.debug_tuple("Tagged").field(v).field(x).finish(),
        }
    }
}

struct WriterFormatter<'a, 'b: 'a> {
    inner: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b> io::Write for WriterFormatter<'a, 'b> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        fn io_error<E>(_: E) -> io::Error {
            // Error value does not matter because fmt::Display impl below just
            // maps it to fmt::Error
            io::Error::new(io::ErrorKind::Other, "fmt error")
        }
        let s = try!(str::from_utf8(buf).map_err(io_error));
        try!(self.inner.write_str(s).map_err(io_error));
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl fmt::Display for Value {
    /// Display a JSON value as a string.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// let json = json!({ "city": "London", "street": "10 Downing Street" });
    ///
    /// // Compact format:
    /// //
    /// // {"city":"London","street":"10 Downing Street"}
    /// let compact = format!("{}", json);
    /// assert_eq!(compact,
    ///     "{\"city\":\"London\",\"street\":\"10 Downing Street\"}");
    ///
    /// // Pretty format:
    /// //
    /// // {
    /// //   "city": "London",
    /// //   "street": "10 Downing Street"
    /// // }
    /// let pretty = format!("{:#}", json);
    /// assert_eq!(pretty,
    ///     "{\n  \"city\": \"London\",\n  \"street\": \"10 Downing Street\"\n}");
    /// # }
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alternate = f.alternate();
        let mut wr = WriterFormatter { inner: f };
        if alternate {
            // {:#}
            super::ser::to_writer_pretty(&mut wr, self).map_err(|_| fmt::Error)
        } else {
            // {}
            super::ser::to_writer(&mut wr, self).map_err(|_| fmt::Error)
        }
    }
}

mod de;
mod from;
//mod index;
//mod partial_eq;
mod ser;

/// Convert a `T` into `serde_json::Value` which is an enum that can represent
/// any valid JSON data.
///
/// # Example
///
/// ```rust
/// extern crate serde;
///
/// #[macro_use]
/// extern crate serde_derive;
///
/// #[macro_use]
/// extern crate serde_json;
///
/// use std::error::Error;
///
/// #[derive(Serialize)]
/// struct User {
///     fingerprint: String,
///     location: String,
/// }
///
/// fn compare_json_values() -> Result<(), Box<Error>> {
///     let u = User {
///         fingerprint: "0xF9BA143B95FF6D82".to_owned(),
///         location: "Menlo Park, CA".to_owned(),
///     };
///
///     // The type of `expected` is `serde_json::Value`
///     let expected = json!({
///                            "fingerprint": "0xF9BA143B95FF6D82",
///                            "location": "Menlo Park, CA",
///                          });
///
///     let v = serde_json::to_value(u).unwrap();
///     assert_eq!(v, expected);
///
///     Ok(())
/// }
/// #
/// # fn main() {
/// #     compare_json_values().unwrap();
/// # }
/// ```
///
/// # Errors
///
/// This conversion can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
///
/// ```rust
/// extern crate serde_json;
///
/// use std::collections::BTreeMap;
///
/// fn main() {
///     // The keys in this map are vectors, not strings.
///     let mut map = BTreeMap::new();
///     map.insert(vec![32, 64], "x86");
///
///     println!("{}", serde_json::to_value(map).unwrap_err());
/// }
/// ```
// Taking by value is more friendly to iterator adapters, option and result
// consumers, etc. See https://github.com/serde-rs/json/pull/149.
pub fn to_value<T>(value: T) -> Result<Value, Error>
    where
        T: Serialize,
{
    value.serialize(Serializer)
}

/// Interpret a `serde_json::Value` as an instance of type `T`.
///
/// # Example
///
/// ```rust
/// #[macro_use]
/// extern crate serde_json;
///
/// #[macro_use]
/// extern crate serde_derive;
///
/// extern crate serde;
///
/// #[derive(Deserialize, Debug)]
/// struct User {
///     fingerprint: String,
///     location: String,
/// }
///
/// fn main() {
///     // The type of `j` is `serde_json::Value`
///     let j = json!({
///                     "fingerprint": "0xF9BA143B95FF6D82",
///                     "location": "Menlo Park, CA"
///                   });
///
///     let u: User = serde_json::from_value(j).unwrap();
///     println!("{:#?}", u);
/// }
/// ```
///
/// # Errors
///
/// This conversion can fail if the structure of the Value does not match the
/// structure expected by `T`, for example if `T` is a struct type but the Value
/// contains something other than a JSON map. It can also fail if the structure
/// is correct but `T`'s implementation of `Deserialize` decides that something
/// is wrong with the data, for example required struct fields are missing from
/// the JSON map or some number is too big to fit in the expected primitive
/// type.
pub fn from_value<T>(value: Value) -> Result<T, Error>
    where
        T: DeserializeOwned,
{
    T::deserialize(value)
}
