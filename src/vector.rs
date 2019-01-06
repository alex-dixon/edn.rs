#[cfg(feature = "immutable")]
use im::Vector;

#[cfg(feature = "immutable")]
pub type Vector<T> = Vector<T>;

#[cfg(not(feature = "immutable"))]
pub type Vector<T> = std::vec::Vec<T>;
