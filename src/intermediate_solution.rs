mod intermediate_route;

use self::intermediate_route::IntermediateRoute;
use crate::problem::BaseProblem;
use alloc::vec::Vec;
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use std::{
    alloc::{Allocator, Global},
    hash::{Hash, Hasher},
    ops::Range,
    rc::Rc,
};

#[derive(Clone, Debug)]
pub struct IntermediateSolution<A: Allocator = Global> {
    routes: Vec<IntermediateRoute, A>,
}

impl<A: Allocator> Solution<A> {
    pub fn new(routes: Vec<IntermediateRoute, A>) -> Self {
        Self { routes }
    }

    pub fn routes(&self) -> &[IntermediateRoute] {
        &self.routes
    }
}

impl<A: Allocator> Eq for Solution<A> {}

impl<A: Allocator> PartialEq for Solution<A> {
    fn eq(&self, other: &Self) -> bool {
        self.routes.len() == other.routes.len()
            && self.routes.iter().zip(&other.routes).all(|(one, other)| {
                one.len() == other.len()
                    && one
                        .as_ref()
                        .iter()
                        .zip(other.as_ref())
                        .all(|(one, other)| one == other)
            })
    }
}

impl<A: Allocator> Hash for IntermediateSolution<A> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.routes.hash(hasher)
    }
}
