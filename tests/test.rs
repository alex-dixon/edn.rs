extern crate edn;
extern crate serde;


use edn::de::{Deserializer,from_str, from_slice};
use edn::value::{Value,from_value,to_value};

use serde::de::{self,IgnoredAny};
use serde::ser;
use std::fmt::Debug;


fn test_parse_ok<T>(tests: Vec<(&str, T)>)
    where
        T: Clone + Debug + PartialEq + ser::Serialize + de::DeserializeOwned,
{
    for (s, value) in tests {
        let v: T = from_str(s).unwrap();
        assert_eq!(v, value.clone());

        let v: T = from_slice(s.as_bytes()).unwrap();
        assert_eq!(v, value.clone());

        // Make sure we can deserialize into a `Value`.
        let json_value: Value = from_str(s).unwrap();
        assert_eq!(json_value, to_value(&value).unwrap());

        // Make sure we can deserialize from a `&Value`.
        let v = T::deserialize(&json_value).unwrap();
        assert_eq!(v, value);

        // Make sure we can deserialize from a `Value`.
        let v: T = from_value(json_value.clone()).unwrap();
        assert_eq!(v, value);

        // Make sure we can round trip back to `Value`.
        let json_value2: Value = from_value(json_value.clone()).unwrap();
        assert_eq!(json_value2, json_value);

        // Make sure we can fully ignore.
        let twoline = s.to_owned() + "\n3735928559";
        let mut de = Deserializer::from_str(&twoline);
        IgnoredAny::deserialize(&mut de).unwrap();
        assert_eq!(0xDEAD_BEEF, u64::deserialize(&mut de).unwrap());

        // Make sure every prefix is an EOF error, except that a prefix of a
        // number may be a valid number.
        if !json_value.is_number() {
            for (i, _) in s.trim_end().char_indices() {
                assert!(from_str::<Value>(&s[..i]).unwrap_err().is_eof());
                assert!(from_str::<IgnoredAny>(&s[..i]).unwrap_err().is_eof());
            }
        }
    }
}

#[test]
fn test_parse_nil() {
    test_parse_err::<()>(&[
        ("n", "EOF while parsing a value at line 1 column 1"),
        ("ni", "EOF while parsing a value at line 1 column 2"),
        // ("nulla", "trailing characters at line 1 column 5"),
    ]);

    test_parse_ok(vec![("nil", ())]);
}
