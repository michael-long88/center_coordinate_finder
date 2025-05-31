use anyhow::Result;
use std::{f64::consts::PI, iter::zip};

pub struct Coordinate {
    latitude: f64,
    longitude: f64,
}

impl Coordinate {
    pub fn new(latitude: f64, longitude: f64) -> Coordinate {
        Coordinate {
            latitude,
            longitude,
        }
    }

    pub fn degrees_to_radians(&self) -> (f64, f64) {
        (self.latitude * PI / 180.0, self.longitude * PI / 180.0)
    }
}

pub fn coordinates_from_vectors(
    latitudes: Vec<f64>,
    longitudes: Vec<f64>,
) -> Result<Vec<Coordinate>> {
    if latitudes.len() != longitudes.len() {
        return Err(anyhow::anyhow!(
            "Vectors of latitudes and longitudes must have the same length"
        ));
    }

    let coordinates: Vec<Coordinate> = zip(latitudes.iter(), longitudes.iter())
        .map(|(lat, lon)| Coordinate::new(*lat, *lon))
        .collect();

    Ok(coordinates)
}

pub fn get_center_from_degrees(coordinates: Vec<Coordinate>) -> (f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut z: f64 = 0.0;

    coordinates.iter().for_each(|coordinate| {
        let (lat, lon) = coordinate.degrees_to_radians();

        x += lat.cos() * lon.cos();
        y += lat.cos() * lon.sin();
        z += lat.sin();
    });

    let num_coords = coordinates.len() as f64;

    x /= num_coords;
    y /= num_coords;
    z /= num_coords;

    let longitude = y.atan2(x);
    let hyp = (x * x + y * y).sqrt();
    let latitude = z.atan2(hyp);

    (radians_to_degrees(latitude), radians_to_degrees(longitude))
}

fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}
