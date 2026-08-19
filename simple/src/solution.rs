use crate::Problem;
use allocator_api2::{
    alloc::{Allocator, Global},
    vec::Vec,
};
use core::{BasicProblem, BasicSolution, BasicStop, BasicVehicle};
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, GeometryValue};
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    ops::Range,
    rc::Rc,
};

// TODO Use persistent data structure.
// TODO Make it more compact.
#[derive(Clone, Debug)]
pub struct Solution<A: Allocator = Global> {
    routes: Vec<Rc<Vec<usize, A>>, A>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SerializableSolution {
    routes: Vec<Vec<usize>>,
}

impl<A: Allocator> Solution<A> {
    pub fn new(routes: Vec<Rc<Vec<usize, A>>, A>) -> Self {
        Self { routes }
    }

    pub fn routes(&self) -> &[Rc<Vec<usize, A>>] {
        &self.routes
    }

    pub fn has_stop(&self, stop_index: usize) -> bool {
        self.routes
            .iter()
            .any(|stop_indexes| stop_indexes.contains(&stop_index))
    }

    #[must_use]
    pub fn add_stop(&self, vehicle_index: usize, stop_index: usize) -> Self
    where
        A: Clone,
    {
        let mut route = self.clone_route(vehicle_index);
        route.push(stop_index);

        let mut routes = self.routes.clone();
        routes[vehicle_index] = route.into();

        Self::new(routes)
    }

    #[must_use]
    pub fn insert_stop(
        &self,
        vehicle_index: usize,
        insertion_index: usize,
        stop_index: usize,
    ) -> Self
    where
        A: Clone,
    {
        let mut route = self.clone_route(vehicle_index);
        route.insert(insertion_index, stop_index);

        let mut routes = self.routes.clone();
        routes[vehicle_index] = route.into();

        Self::new(routes)
    }

    #[must_use]
    pub fn extend_route(
        &self,
        vehicle_index: usize,
        stop_indexes: impl IntoIterator<Item = usize>,
    ) -> Self
    where
        A: Clone,
    {
        let mut route = self.clone_route(vehicle_index);
        route.extend(stop_indexes);

        let mut routes = self.routes.clone();
        routes[vehicle_index] = route.into();

        Self::new(routes)
    }

    #[must_use]
    pub fn drain_route(&self, vehicle_index: usize, stop_range: Range<usize>) -> Self
    where
        A: Clone,
    {
        let mut route = self.clone_route(vehicle_index);
        route.drain(stop_range);

        let mut routes = self.routes.clone();
        routes[vehicle_index] = route.into();

        Self::new(routes)
    }

    pub fn clone_in<B: Allocator + Clone>(&self, allocator: B) -> Solution<B> {
        Solution::new({
            let mut routes = Vec::with_capacity_in(self.routes.len(), allocator.clone());
            routes.extend(self.routes().iter().map(|original| {
                let mut route = Vec::with_capacity_in(original.len(), allocator.clone());
                route.extend(original.iter().copied());
                route.into()
            }));
            routes
        })
    }

    #[must_use]
    pub fn reverse_route(&self, vehicle_index: usize) -> Self
    where
        A: Clone,
    {
        let mut route = self.clone_route(vehicle_index);
        route.reverse();

        let mut routes = self.routes.clone();
        routes[vehicle_index] = route.into();

        Self::new(routes)
    }

    fn clone_route(&self, vehicle_index: usize) -> Vec<usize, A>
    where
        A: Clone,
    {
        self.routes[vehicle_index].as_ref().clone()
    }

    pub fn to_geojson(&self, problem: &Problem) -> GeoJson {
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
                        value: GeometryValue::LineString {
                            coordinates: [problem.vehicle(vehicle_index).start_location()]
                                .into_iter()
                                .chain(
                                    route
                                        .iter()
                                        .map(|&stop_index| problem.stop(stop_index).location()),
                                )
                                .chain([problem.vehicle(vehicle_index).end_location()])
                                .map(|index| {
                                    let coordinates = problem.location(index).as_point();

                                    [coordinates.x(), coordinates.y()].into()
                                })
                                .collect(),
                        },
                    }),
                    ..Default::default()
                })
                .collect(),
        }
        .into()
    }

    pub fn to_json(&self) -> Result<serde_json::value::Value, serde_json::Error> {
        serde_json::to_value(SerializableSolution {
            routes: self
                .routes
                .iter()
                .map(|route| route.iter().copied().collect())
                .collect(),
        })
    }
}

impl Solution<Global> {
    pub fn from_routes(routes: impl IntoIterator<Item = impl IntoIterator<Item = usize>>) -> Self {
        Self::new(
            routes
                .into_iter()
                .map(|route| Rc::new(route.into_iter().collect()))
                .collect(),
        )
    }

    pub fn from_json(value: serde_json::value::Value) -> Result<Self, serde_json::Error> {
        Ok(Self::from_routes(
            serde_json::from_value::<SerializableSolution>(value)?.routes,
        ))
    }
}

impl<A: Allocator> BasicSolution for Solution<A> {
    fn routes(&self) -> impl Iterator<Item = impl Iterator<Item = usize>> {
        self.routes.iter().map(|indexes| indexes.iter().copied())
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

impl<A: Allocator> Hash for Solution<A> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.routes.hash(hasher)
    }
}
