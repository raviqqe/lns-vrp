use crate::problem::BaseProblem;
use alloc::vec::Vec;
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use pvec::PVec;
use std::{
    alloc::{Allocator, Global},
    hash::{Hash, Hasher},
    ops::Range,
    rc::Rc,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Solution {
    routes: PVec<PVec<usize>>,
}

impl Solution {
    pub fn new(routes: Vec<Rc<[usize]>>) -> Self {
        Self { routes }
    }

    pub fn routes(&self) -> &[Rc<[usize]>] {
        &self.routes
    }

    pub fn has_stop(&self, stop_index: usize) -> bool {
        self.routes
            .into_iter()
            .any(|stop_indexes| stop_indexes.contains(&stop_index))
    }

    #[must_use]
    pub fn add_stop(&self, vehicle_index: usize, stop_index: usize) -> Self {
        Self::new(self.routes.update(
            vehicle_index,
            self.clone_route(vehicle_index).push(stop_index),
        ))
    }

    #[must_use]
    pub fn insert_stop(
        &self,
        vehicle_index: usize,
        insertion_index: usize,
        stop_index: usize,
    ) -> Self {
        Self::new(self.routes.update(
            vehicle_index,
            self.routes[vehicle_index].insert(insertion_index, stop_index),
        ))
    }

    #[must_use]
    pub fn ruin_route(&self, vehicle_index: usize, stop_range: Range<usize>) -> Self {
        let mut route = self.clone_route(vehicle_index);
        route.drain(stop_range);

        let mut routes = self.routes.clone();
        routes[vehicle_index] = route.into();

        Self::new(routes)
    }

    pub fn to_geojson(&self, problem: impl BaseProblem) -> GeoJson {
        FeatureCollection {
            bbox: None,
            foreign_members: None,
            features: self
                .routes
                .into_iter()
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
                                        .into_iter()
                                        .map(|stop_index| problem.stop_location(stop_index)),
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

// TODO Implement Hash.
impl Hash for Solution {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        for route in self.routes {
            for index in route {
                index.hash(hasher);
            }
        }
    }
}
