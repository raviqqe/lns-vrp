// The fields are in the order in GeoJSON.
#[derive(Clone, Debug)]
pub struct Location {
    longitude: f64,
    latitude: f64,
}

impl Location {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self {
            longitude,
            latitude,
        }
    }

    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    pub fn latitude(&self) -> f64 {
        self.latitude
    }
}
