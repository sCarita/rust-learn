use std::fs::File;
use std::io::prelude::*;

pub fn run() {
	let mut file = File::open("src/fixtures/things.csv").expect("Invalid file!");

	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.expect("Cannot read the file!");

	let mut reading: Vec<&str>;
	let mut is_header: bool = true;
	let mut sensor_type: &str = "";
	let mut sensor_unit: &str = "";

	for w in contents.split('\n') {
		reading = w.split(',').collect::<Vec<&str>>();
		if is_header == false {
			println!(
				"sensor ({:?}): {:?} -> reading ({:?}): {:?}",
				sensor_type, reading[0],
				sensor_unit, reading[1]
			);
		} else {
			is_header = false;
			sensor_type = reading[0];
			sensor_unit = reading[1];
		}
	}
}