use std::collections::HashMap;

pub fn run() {
	// key => value
	let mut marks = HashMap::new();
	println!("{:?}", marks);

	// Add values
	marks.insert("Maths", 18);
	marks.insert("CS", 20);
	println!("marks: {:?} len {:?}", marks, marks.len());

	// Indexing
	match marks.get("Maths Advanced") {
		Some(mark) => println!("You got {} for web dev!", mark),
		None => println!("you never studied that")
	}
	marks.remove("CS");
	println!("marks: {:?} len {:?}", marks, marks.len());
	println!("did you studied Maths?: {:?}", marks.contains_key(""));

	// Iterate.
	for (subj, mark) in &marks {
		println!("subj {}, mark {}", subj, mark);
	}
}