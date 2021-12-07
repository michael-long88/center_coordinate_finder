use std::f32::consts::PI;

fn main() {
    let lats: Vec<f32> = vec![33.6956461, 34.1984435];
    let longs: Vec<f32> = vec![-78.8900409, -79.7671658];
    println!("{:?}", get_center_from_degrees(lats, longs));
}

fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / PI
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

fn get_center_from_degrees(latitudes: Vec<f32>, longitudes: Vec<f32>) -> (f32, f32) {
    let num_coords = latitudes.len();
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = 0.0;
    let mut lat: f32;
    let mut lon: f32;

    for i in 0..num_coords {
        lat = degrees_to_radians(latitudes[i]);
        lon = degrees_to_radians(longitudes[i]);

        x = x + (lat.cos() * lon.cos());
        y = y + (lat.cos() * lon.sin());
        z = z + lat.sin();
    }

    let num_coords = num_coords as f32;

    x = x / num_coords;
    y = y / num_coords;
    z = z / num_coords;

    lon = y.atan2(x);
    let mut hyp = &x * &x + &y * &y;
    hyp = hyp.sqrt();
    lat = z.atan2(hyp);

    (radians_to_degrees(lat), radians_to_degrees(lon))
}
