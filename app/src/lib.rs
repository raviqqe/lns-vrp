#![feature(allocator_api)]

extern crate alloc;
extern crate core;

#[doc(hidden)]
pub mod bin_utility;
pub mod cost;
mod hash_map;
mod problem;
pub mod route;
pub mod solve;
mod stop;
mod utility;
#[macro_use]
mod trace;
mod vehicle;

pub use self::{
    location::Location, problem::SimpleProblem, solution::Solution, stop::Stop, vehicle::Vehicle,
};
