use std::borrow::Cow;

use map::Map;
use vector::Vector;
//use set::Set;

#[cfg(feature = "immutable")]
use im::{HashMap, HashSet, Vector};
//#[cfg(not(feature = "immutable"))]
//use std::collections::{BTreeSet, BTreeMap};

#[cfg(feature = "immutable")]
use std::hash::Hash;


use std::str;

use super::Value;
use number::Number;

macro_rules! from_integer {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Value {
                fn from(n: $ty) -> Self {
                    Value::Number(n.into())
                }
            }
        )*
    };
}

from_integer! {
    i8 i16 i32 i64 isize
    u8 u16 u32 u64 usize
}

#[cfg(feature = "arbitrary_precision")]
serde_if_integer128! {
    from_integer! {
        i128 u128
    }
}

impl From<f32> for Value {
    /// Convert 32-bit floating point number to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use serde_json::Value;
    ///
    /// let f: f32 = 13.37;
    /// let x: Value = f.into();
    /// # }
    /// ```
    fn from(f: f32) -> Self {
        From::from(f as f64)
    }
}

impl From<f64> for Value {
    /// Convert 64-bit floating point number to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use serde_json::Value;
    ///
    /// let f: f64 = 13.37;
    /// let x: Value = f.into();
    /// # }
    /// ```
    fn from(f: f64) -> Self {
        Number::from_f64(f).map_or(Value::Nil, Value::Number)
    }
}

impl From<bool> for Value {
    /// Convert boolean to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use serde_json::Value;
    ///
    /// let b = false;
    /// let x: Value = b.into();
    /// # }
    /// ```
    fn from(f: bool) -> Self {
        Value::Boolean(f)
    }
}

impl From<String> for Value {
    /// Convert `String` to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use serde_json::Value;
    ///
    /// let s: String = "lorem".to_string();
    /// let x: Value = s.into();
    /// # }
    /// ```
    fn from(f: String) -> Self {
        Value::String(f)
    }
}

impl From<char> for Value {
    fn from(s: char) -> Self {
        Value::Char(s)
    }
}

impl<'a> From<&'a str> for Value {
    /// Convert string slice to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use serde_json::Value;
    ///
    /// let s: &str = "lorem";
    /// let x: Value = s.into();
    /// # }
    /// ```
    fn from(f: &str) -> Self {
        Value::String(f.to_string())
    }
}

impl<'a> From<Cow<'a, str>> for Value {
    /// Convert copy-on-write string to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use serde_json::Value;
    /// use std::borrow::Cow;
    ///
    /// let s: Cow<str> = Cow::Borrowed("lorem");
    /// let x: Value = s.into();
    /// # }
    /// ```
    ///
    /// ```rust
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use serde_json::Value;
    /// use std::borrow::Cow;
    ///
    /// let s: Cow<str> = Cow::Owned("lorem".to_string());
    /// let x: Value = s.into();
    /// # }
    /// ```
    fn from(f: Cow<'a, str>) -> Self {
        Value::String(f.into_owned())
    }
}

impl From<Map<Value, Value>> for Value {
    /// Convert map (with string keys) to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use serde_json::{Map, Value};
    ///
    /// let mut m = Map::new();
    /// m.insert("Lorem".to_string(), "ipsum".into());
    /// let x: Value = m.into();
    /// # }
    /// ```
    fn from(f: Map<Value, Value>) -> Self {
        Value::Map(f)
    }
}

impl<T: Into<Value>> From<Vector<T>> for Value {
    /// Convert a `Vec` to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use serde_json::Value;
    ///
    /// let v = vec!["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into();
    /// # }
    /// ```
    fn from(f: Vector<T>) -> Self {
        Value::Vector(f.into_iter().map(Into::into).collect())
    }
}

impl<'a, T: Clone + Into<Value>> From<&'a [T]> for Value {
    /// Convert a slice to `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use serde_json::Value;
    ///
    /// let v: &[&str] = &["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into();
    /// # }
    /// ```
    fn from(f: &'a [T]) -> Self {
        Value::Vector(f.iter().cloned().map(Into::into).collect())
    }
}

impl<T: Into<Value>> ::std::iter::FromIterator<T> for Value {
    /// Convert an iteratable type to a `Value`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use serde_json::Value;
    ///
    /// let v = std::iter::repeat(42).take(5);
    /// let x: Value = v.collect();
    /// # }
    /// ```
    ///
    /// ```rust
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use serde_json::Value;
    ///
    /// let v: Vec<_> = vec!["lorem", "ipsum", "dolor"];
    /// let x: Value = v.into_iter().collect();
    /// # }
    /// ```
    ///
    /// ```rust
    /// # extern crate serde_json;
    /// #
    /// # fn main() {
    /// use std::iter::FromIterator;
    /// use serde_json::Value;
    ///
    /// let x: Value = Value::from_iter(vec!["lorem", "ipsum", "dolor"]);
    /// # }
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Value::Vector(iter.into_iter().map(Into::into).collect())
    }
}


//#[cfg(not(feature = "immutable"))]
//impl<A> From<Vec<A>> for Value
//    where
//        Value: From<A>,
//{
//    fn from(s: Vec<A>) -> Self {
//        Value::Vector(s.into_iter().map(Value::from).collect())
//    }
//}
//
//#[cfg(feature = "immutable")]
//impl<A> From<Vector<A>> for Value
//    where
//        A: Clone + Hash + Eq,
//        Value: From<A>,
//{
//    fn from(s: Vector<A>) -> Self {
//        Value::Vector(s.iter().map(|a| Value::from(a.clone())).collect())
//    }
//}
//
//
//#[cfg(not(feature = "immutable"))]
//impl<K, V> From<BTreeMap<K, V>> for Value
//    where
//        Value: From<K>,
//        Value: From<V>,
//{
//    fn from(s: Map<K, V>) -> Self {
//        let mut map = Map::new();
//        for (k, v) in s {
//            map.insert(Value::from(k), Value::from(v));
//        }
//        Value::Map(map)
//    }
//}
//
//
//#[cfg(feature = "immutable")]
//impl<K, V> From<HashMap<K, V>> for Value
//    where
//        K: Clone + Hash + Eq,
//        V: Clone + Hash + Eq,
//        Value: From<K>,
//        Value: From<V>,
//{
//    fn from(s: HashMap<K, V>) -> Self {
//        Value::Map(
//            s.iter()
//                .map(|(k, v)|
//                    (Value::from(k.clone()), Value::from(v.clone()))).collect())
//    }
//}
//
//#[cfg(not(feature = "immutable"))]
//impl<A> From<BTreeSet<A>> for Value
//    where
//        Value: From<A>,
//{
//    fn from(s: BTreeSet<A>) -> Self {
//        let mut set = BTreeSet::new();
//        s.into_iter().for_each(|a| {
//            set.insert(Value::from(a));
//        });
//        Value::Set(set)
//    }
//}
//
//
//#[cfg(feature = "immutable")]
//impl<A> From<HashSet<A>> for Value
//    where
//        A: Clone + Hash + Eq,
//        Value: From<A>,
//{
//    fn from(s: HashSet<A>) -> Self {
//        Value::Set(s.iter().map(|v| Value::from(v.clone())).collect())
//    }
//}

///////////////


