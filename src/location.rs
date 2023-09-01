use ordered_float::OrderedFloat;

// The fields are in the order in GeoJSON.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Location {
    longitude: OrderedFloat<f64>,
    latitude: OrderedFloat<f64>,
}

impl Location {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self {
            longitude: OrderedFloat(longitude),
            latitude: OrderedFloat(latitude),
        }
    }

    pub fn longitude(&self) -> f64 {
        self.longitude.0
    }

    pub fn latitude(&self) -> f64 {
        self.latitude.0
    }
}
