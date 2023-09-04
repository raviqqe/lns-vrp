use geo::Point;
use ordered_float::OrderedFloat;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Location(Point);

impl Location {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self(Point::new(longitude, latitude))
    }

    pub fn as_point(&self) -> &Point {
        &self.0
    }
}

impl From<Location> for Point {
    fn from(val: Location) -> Self {
        val.0
    }
}

impl Eq for Location {}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        OrderedFloat(self.0.x()) == OrderedFloat(other.0.x())
            && OrderedFloat(self.0.y()) == OrderedFloat(other.0.y())
    }
}

impl Hash for Location {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        OrderedFloat(self.0.x()).hash(hasher);
        OrderedFloat(self.0.y()).hash(hasher);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    fn hash_location(location: &Location) -> u64 {
        let mut hasher = DefaultHasher::new();

        location.hash(&mut hasher);

        hasher.finish()
    }

    #[test]
    fn partial_eq() {
        assert_eq!(Location::new(0.0, 0.0), Location::new(0.0, 0.0));
        assert_ne!(Location::new(1.0, 0.0), Location::new(0.0, 0.0));
        assert_ne!(Location::new(0.0, 1.0), Location::new(0.0, 0.0));
    }

    #[test]
    fn hash() {
        assert_eq!(
            hash_location(&Location::new(0.0, 0.0)),
            hash_location(&Location::new(0.0, 0.0))
        );
        assert_ne!(
            hash_location(&Location::new(1.0, 0.0)),
            hash_location(&Location::new(0.0, 0.0))
        );
        assert_ne!(
            hash_location(&Location::new(1.0, 0.0)),
            hash_location(&Location::new(0.0, 0.0))
        );
    }
}
