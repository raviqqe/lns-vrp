use ahash::AHasher;
use std::{alloc::Global, hash::BuildHasherDefault};

// We do not use random state just for determinism in testing.
// Do not do this on production!
pub type HashMap<K, V, A = Global> = hashbrown::HashMap<K, V, BuildHasherDefault<AHasher>, A>;
