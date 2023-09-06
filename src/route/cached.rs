use std::cell::RefMut;

use super::Router;
use crate::{hash_map::HashMap, Location};
use geo::GeodesicDistance;
use ordered_float::OrderedFloat;

#[derive(Debug, Default)]
pub struct CachedRouter<R: Router> {
    router: R,
    cache: RefMut<HashMap<(OrderedFloat<f64>, OrderedFloat<f64>), f64>>,
}

impl<R: Router> CachedRouter<R> {
    pub const fn new(router: R) -> Self {
        Self {
            router,
            cache: Default::default(),
        }
    }
}

impl<R: Router> Router for &CachedRouter<R> {
    fn route(&self, start: &Location, end: &Location) -> f64 {
        let cached = cache.get((location.clone(), location.clone()));
    }
}
