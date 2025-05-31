mod cli;
mod coordinates;

use clap::Parser;
use cli::Cli;
use coordinates::{coordinates_from_vectors, get_center_from_degrees};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let (latitudes, longitudes) = cli.parse_coordinates().map_err(|e| e.to_string())?;

    let coordinates = coordinates_from_vectors(latitudes, longitudes)?;
    let center = get_center_from_degrees(coordinates);

    println!("Center coordinates: ({}, {})", center.0, center.1);

    Ok(())
}
