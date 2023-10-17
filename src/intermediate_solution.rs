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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IntermediateSolution<A: Allocator = Global> {
    routes: Vec<IntermediateRoute, A>,
}

impl<A: Allocator> IntermediateSolution<A> {
    pub fn new(routes: Vec<IntermediateRoute, A>) -> Self {
        Self { routes }
    }

    pub fn routes(&self) -> &[IntermediateRoute] {
        &self.routes
    }
}
