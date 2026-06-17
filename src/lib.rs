use rand::distr::{Distribution, Uniform};
use std::f64::consts::TAU; // TAU is 2 * PI
// inspired by https://github.com/faker-js/faker/blob/b8abfc6415fe5be3a207b1b3dd4266905b924f84/src/modules/location/index.ts#L131

#[derive(Debug)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

pub struct NearbyCoordinate {
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
    pub is_metric: bool,
}

impl NearbyCoordinate {
    fn get_random_angle_radians() -> f64 {
        let mut rng = rand::rng();
        // 1. Get a uniform float between 0.0 and 2 * PI
        let raw_angle: f64 = Uniform::new(0.0, TAU).unwrap().sample(&mut rng);

        (raw_angle * 100_000.0).round() / 100_000.0
    }

    fn get_distance_in_kms(radius: f64) -> f64 {
        let mut rng = rand::rng();
        // Get a uniform float between 0.0 and 2 * PI
        let raw_angle: f64 = Uniform::new(0.0, radius).unwrap().sample(&mut rng);

        (raw_angle * 1000.0).round() / 1000.0
    }

    pub fn new(latitude: f64, longitude: f64, radius: f64, is_metric: bool) -> Self {
        Self {
            latitude,
            longitude,
            radius,
            is_metric,
        }
    }

    pub fn get_random_coordinate(&self) -> Coordinate {
        let angle: f64 = Self::get_random_angle_radians();
        let radius_metric: f64 = if self.is_metric {
            self.radius
        } else {
            self.radius * 1.60934
        }; //in kms
        let distance_in_kms = Self::get_distance_in_kms(radius_metric);

        // The distance in km per degree for earth
        let km_per_degree: f64 = (40_000 / 360) as f64; // in km/°
        let distance_in_degree: f64 = distance_in_kms / km_per_degree; // in °

        let coordinate: [f64; 2] = [
            self.latitude + angle.sin() * distance_in_degree,
            self.longitude + angle.cos() * distance_in_degree,
        ];

        // Box latitude [-90°, 90°]
        let mut new_latitude: f64 = coordinate[0].rem_euclid(180.0);
        let mut new_longitude: f64 = coordinate[1];

        if new_latitude < -90.0 || new_latitude > 90.0 {
            new_latitude = new_latitude - 180.0;
            new_longitude += 180.0;
        }

        // Box longitude [-180°, 180°]
        new_longitude = (((new_longitude % 360.0) + 540.0) % 360.0) - 180.0;

        Coordinate {
            latitude: new_latitude,
            longitude: new_longitude,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nearby_coordinate =
            NearbyCoordinate::new(41.896738135197026, -87.62393942418863, 10.0, false);
        let coordinate = nearby_coordinate.get_random_coordinate();
        assert!(coordinate.latitude >= -90.0 && coordinate.latitude <= 90.0);
        assert!(coordinate.longitude >= -180.0 && coordinate.longitude <= 180.0);
    }
}
