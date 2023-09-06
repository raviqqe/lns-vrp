use crate::problem::BaseProblem;
use alloc::vec::Vec;
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use im_rc::Vector;
use std::{
    alloc::{Allocator, Global},
    hash::{Hash, Hasher},
    ops::Range,
    rc::Rc,
};

// TODO Make it more compact.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Solution {
    routes: Vector<Vector<usize>>,
}

impl Solution {
    pub fn new(routes: Vector<Vector<usize>>) -> Self {
        Self { routes }
    }

    pub fn routes(&self) -> &Vector<Vector<usize>> {
        &self.routes
    }

    pub fn has_stop(&self, stop_index: usize) -> bool {
        self.routes
            .iter()
            .any(|stop_indexes| stop_indexes.contains(&stop_index))
    }

    #[must_use]
    pub fn add_stop(&self, vehicle_index: usize, stop_index: usize) -> Self {
        let mut route = self.routes[vehicle_index].clone();
        route.push_back(stop_index);

        let mut routes = self.routes.clone();
        routes.update(vehicle_index, route);

        Self::new(routes)
    }

    #[must_use]
    pub fn insert_stop(
        &self,
        vehicle_index: usize,
        insertion_index: usize,
        stop_index: usize,
    ) -> Self {
        let mut route = self.routes[vehicle_index].clone();
        route.insert(insertion_index, stop_index);

        let mut routes = self.routes.clone();
        routes.update(vehicle_index, route);

        Self::new(routes)
    }

    #[must_use]
    pub fn ruin_route(&self, vehicle_index: usize, stop_range: Range<usize>) -> Self {
        let mut route = self.routes[vehicle_index].clone();
        route.drain(stop_range);

        let mut routes = self.routes.clone();
        routes.update(vehicle_index, route);

        Self::new(routes)
    }

    pub fn to_geojson(&self, problem: impl BaseProblem) -> GeoJson {
        FeatureCollection {
            bbox: None,
            foreign_members: None,
            features: self
                .routes
                .iter()
                .enumerate()
                .map(|(vehicle_index, route)| Feature {
                    geometry: Some(Geometry {
                        bbox: None,
                        foreign_members: None,
                        value: Value::LineString(
                            [problem.vehicle_start_location(vehicle_index)]
                                .into_iter()
                                .chain(
                                    route
                                        .iter()
                                        .map(|&stop_index| problem.stop_location(stop_index)),
                                )
                                .chain([problem.vehicle_end_location(vehicle_index)])
                                .map(|index| {
                                    let coordinates = problem.location(index).as_point();

                                    vec![coordinates.x(), coordinates.y()]
                                })
                                .collect(),
                        ),
                    }),
                    ..Default::default()
                })
                .collect(),
        }
        .into()
    }
}
