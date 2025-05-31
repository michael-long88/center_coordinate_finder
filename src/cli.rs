use clap::Parser;

/// Find the center coordinate from a set of latitude and longitude pairs
#[derive(Parser, Debug)]
pub struct Cli {
    /// Coordinates in the format of "lat,lon" pairs (e.g., "33.69564610,-78.89004090 34.19844350,-79.76716580")
    #[clap(required = true)]
    coordinates: Vec<String>,
}

impl Cli {
    pub fn parse_coordinates(&self) -> Result<(Vec<f64>, Vec<f64>), String> {
        let mut latitudes = Vec::new();
        let mut longitudes = Vec::new();

        for coord_str in &self.coordinates {
            let parts: Vec<&str> = coord_str.split(',').collect();
            if parts.len() != 2 {
                return Err(format!(
                    "Invalid coordinate format: '{}'. Expected 'lat,lon'",
                    coord_str
                ));
            }

            let lat = parts[0]
                .trim()
                .parse::<f64>()
                .map_err(|_| format!("Invalid latitude: '{}'", parts[0]))?;
            let lon = parts[1]
                .trim()
                .parse::<f64>()
                .map_err(|_| format!("Invalid longitude: '{}'", parts[1]))?;

            latitudes.push(lat);
            longitudes.push(lon);
        }

        Ok((latitudes, longitudes))
    }
}
