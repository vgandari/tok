use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{collections::HashMap, fs::File, io::prelude::*};

/// Struct containing HashMap of key value pairs in YAML file
#[derive(Serialize, Deserialize)]
pub struct DeserializedMap {
	#[serde(flatten)]
	pub pairs: HashMap<String, Value>,
}

/// Read data from a YAML file
pub fn read_from_yaml(filename: &String) -> DeserializedMap {
	println!("Reading {}", filename);
	let mut file = File::open(filename).expect("Can't open file!");
	let mut contents = String::new();
	file
		.read_to_string(&mut contents)
		.expect("Cannot read data");
	// let empty_map: HashMap<String, Value> = HashMap::new();
	// let empty_node = DeserializedMap { pairs: empty_map };
	// serde_yaml::from_str(&contents).unwrap_or(empty_node)
	serde_yaml::from_str(&contents).unwrap_or(DeserializedMap {
		pairs: HashMap::new(),
	})
}
