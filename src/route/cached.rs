use super::Router;
use crate::{hash_map::HashMap, Location};
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct CachedRouter<R: Router> {
    router: R,
    cache: RefCell<HashMap<(Location, Location), f64>>,
}

impl<R: Router> CachedRouter<R> {
    pub fn new(router: R) -> Self {
        Self {
            router,
            cache: RefCell::new(HashMap::default()),
        }
    }
}

impl<R: Router> Router for &CachedRouter<R> {
    fn route(&self, start: &Location, end: &Location) -> f64 {
        if let Some(&cached) = self.cache.borrow().get(&(start.clone(), end.clone())) {
            return cached;
        }

        let value = self.router.route(start, end);

        self.cache
            .borrow_mut()
            .insert((start.clone(), end.clone()), value);

        value
    }
}
