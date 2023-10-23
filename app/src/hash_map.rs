use ahash::AHasher;
use std::hash::BuildHasherDefault;

// We do not use random state just for determinism in testing.
// Do not do this on production!
pub type HashMap<K, V> = std::collections::HashMap<K, V, BuildHasherDefault<AHasher>>;
