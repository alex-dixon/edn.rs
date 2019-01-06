#[cfg(feature = "immutable")]
use im::HashSet;

#[cfg(not(feature = "immutable"))]
use std::collections::{BTreeSet};

#[cfg(feature = "immutable")]
pub type Set<T> = HashSet<T>;

#[cfg(not(feature = "immutable"))]
pub type Set<T> = BTreeSet<T>;




