#[cfg(feature = "immutable")]
use im::HashMap;

#[cfg(not(feature = "immutable"))]
use std::collections::{BTreeMap};

#[cfg(feature = "immutable")]
pub type Map<K, V> = HashMap<K, V>;

#[cfg(not(feature = "immutable"))]
pub type Map<K, V> = BTreeMap<K, V>;
