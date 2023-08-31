use csv::ReaderBuilder;
use geo_quadkey_rs::Quadkey;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug)]
struct Airport {
    type_: String,
    name: String,
    latitude_deg: f64,
    longitude_deg: f64,
    elevation_ft: Option<i32>,
    quadkey: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Geometry {
    #[serde(rename = "type")]
    type_: String,
    coordinates: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Properties {
    name: String,
    type_: String,
    elevation_ft: String,
    quadkey: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Feature {
    #[serde(rename = "type")]
    type_: String,
    geometry: Geometry,
    properties: Properties,
}

#[derive(Serialize, Deserialize, Debug)]
struct GeoJson {
    #[serde(rename = "type")]
    type_: String,
    features: Vec<Feature>,
}

fn get_airports() -> Result<HashMap<String, Airport>, Box<dyn std::error::Error>> {
    let mut airports = HashMap::new();
    let reader = ReaderBuilder::new().from_path("./data/airport_code.csv")?;
    for result in reader.into_records() {
        let record = result?;
        let latitude_deg: f64 = record[4].parse()?;
        let longitude_deg: f64 = record[5].parse()?;
        let elevation_ft: Option<i32> = record[6].parse().ok();
        let quadkey = Quadkey::encode(latitude_deg, longitude_deg, 20).to_string();
        let airport = Airport {
            type_: record[2].to_string(),
            name: record[3].to_string(),
            latitude_deg,
            longitude_deg,
            elevation_ft,
            quadkey: quadkey.clone(),
        };
        airports.insert(record[1].to_string(), airport);
    }
    Ok(airports)
}

fn write_geojson(airports: &HashMap<String, Airport>) -> Result<(), Box<dyn std::error::Error>> {
    let mut features = Vec::new();
    for airport in airports.values() {
        let feature = Feature {
            type_: "Feature".to_string(),
            geometry: Geometry {
                type_: "Point".to_string(),
                coordinates: vec![airport.longitude_deg, airport.latitude_deg],
            },
            properties: Properties {
                name: airport.name.clone(),
                type_: airport.type_.clone(),
                elevation_ft: match airport.elevation_ft {
                    Some(elevation_ft) => elevation_ft.to_string(),
                    None => "".to_string(),
                },
                quadkey: airport.quadkey.clone(),
            },
        };
        features.push(feature);
    }
    let geojson = GeoJson {
        type_: "FeatureCollection".to_string(),
        features,
    };
    let json = to_string(&geojson)?;
    let file = File::create("./data/airport_code_quadkey.geojson")?;
    let mut writer = BufWriter::new(file);
    writer.write_all(json.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let airports = get_airports()?;
    write_geojson(&airports)?;
    Ok(())
}
