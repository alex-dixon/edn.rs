#[macro_use]
extern crate serde;

extern crate ordered_float;
extern crate itoa;
extern crate ryu;

#[cfg(feature = "immutable")]
extern crate im;

#[doc(inline)]
pub use self::value::{ Value};


// We only use our own error type; no need for From conversions provided by the
// standard library's try! macro. This reduces lines of LLVM IR by 4%.
macro_rules! try {
    ($e:expr) => {
        match $e {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
        }
    };
}

pub mod de;
pub mod error;
pub mod iter;
pub mod value;
pub mod parser;
pub mod read;
pub mod ser;
pub mod number;
pub mod map;
pub mod vector;
pub mod set;
