use alloc::vec::Vec;
use std::{
    alloc::{Allocator, Global},
    hash::{Hash, Hasher},
    ops::Range,
    rc::Rc,
};

// TODO Use persistent data structure.
// TODO Make it more compact.
#[derive(Clone, Debug)]
pub struct Solution<A: Allocator = Global> {
    routes: Vec<Rc<[usize], A>, A>,
}

impl<A: Allocator> Solution<A> {
    pub fn new(routes: Vec<Rc<[usize], A>, A>) -> Self {
        Self { routes }
    }

    pub fn routes(&self) -> &[Rc<[usize], A>] {
        &self.routes
    }

    pub fn add_stop(&self, vehicle_index: usize, stop_index: usize) -> Self
    where
        A: Clone,
    {
        let mut route = self.routes[vehicle_index].to_vec_in(self.routes.allocator().clone());
        route.push(stop_index);

        let mut routes = self.routes.clone();
        routes[vehicle_index] = route.into();

        Self::new(routes)
    }

    pub fn insert_stop(
        &self,
        vehicle_index: usize,
        insertion_index: usize,
        stop_index: usize,
    ) -> Self
    where
        A: Clone,
    {
        let mut route = self.routes[vehicle_index].to_vec_in(self.routes.allocator().clone());
        route.insert(insertion_index, stop_index);

        let mut routes = self.routes.clone();
        routes[vehicle_index] = route.into();

        Self::new(routes)
    }

    pub fn has_stop(&self, stop_index: usize) -> bool {
        self.routes
            .iter()
            .any(|stop_indexes| stop_indexes.contains(&stop_index))
    }

    pub fn ruin_route(&self, vehicle_index: usize, stop_range: Range<usize>) -> Self
    where
        A: Clone,
    {
        let mut route = self.clone_route(vehicle_index);
        route.drain(stop_range);

        let mut routes = self.routes.clone();
        routes[vehicle_index] = route.into();

        Self::new(routes)
    }

    pub fn to_global(&self) -> Solution<Global> {
        Solution::new(
            self.routes()
                .iter()
                .map(|route| route.to_vec().into())
                .collect(),
        )
    }

    fn clone_route(&self, vehicle_index: usize) -> Vec<usize, A>
    where
        A: Clone,
    {
        self.routes[vehicle_index].to_vec_in(self.routes.allocator().clone())
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
