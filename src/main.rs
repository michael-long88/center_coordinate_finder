mod coordinates;
use coordinates::{coordinates_from_vectors, get_center_from_degrees};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lats: Vec<f64> = vec![33.6956461, 34.1984435];
    let longs: Vec<f64> = vec![-78.8900409, -79.7671658];
    let coordinates = coordinates_from_vectors(lats, longs)?;
    let center = get_center_from_degrees(coordinates);

    println!("Center coordinates: ({}, {})", center.0, center.1);

    Ok(())
}
