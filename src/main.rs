use std::f64::consts::PI;

fn main() {
    let lats: Vec<f64> = vec![33.6956461, 34.1984435];
    let longs: Vec<f64> = vec![-78.8900409, -79.7671658];
    println!("{:?}", get_center_from_degrees(lats, longs));
}

fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn get_center_from_degrees(latitudes: Vec<f64>, longitudes: Vec<f64>) -> (f64, f64) {
    let num_coords = latitudes.len();
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut z: f64 = 0.0;
    let mut lat: f64;
    let mut lon: f64;

    for i in 0..num_coords {
        lat = degrees_to_radians(latitudes[i]);
        lon = degrees_to_radians(longitudes[i]);

        x += lat.cos() * lon.cos();
        y += lat.cos() * lon.sin();
        z += lat.sin();
    }

    let num_coords = num_coords as f64;

    x /= num_coords;
    y /= num_coords;
    z /= num_coords;

    lon = y.atan2(x);
    let hyp = (x * x + y * y).sqrt();
    lat = z.atan2(hyp);

    (radians_to_degrees(lat), radians_to_degrees(lon))
}
