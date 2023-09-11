use std::collections::HashMap;
use preprocessor::Preprocessor;
use binary_object::BinaryObject;
use entropy::Entropy;
use std::fs::File;

pub struct EntropyPreprocessor;

impl Preprocessor for EntropyPreprocessor {
	fn process(&self, binary_object: &BinaryObject) -> HashMap<String, String> {
		let path = match &binary_object.file_path {
			Some(path) => path, 
				None => {
					return HashMap::new();
				}
		};
		let f = match File::open(path) {
			Ok(f) => f,
				Err(err) => {
					println!("Unable to open file for entropy preprocessor. Error: {}",err);
					return HashMap::new();
				}
		};
		let entr = Entropy::new(&f);					
		let shan_entr = entr.shannon_entropy();
		hashmap!{
			String::from("shan") => shan_entr.to_string(),
		}

	}

	fn info(&self) -> (&'static str, &'static str) {
		("entropy_cal", "calculates the entropy of a given file")
	}
}
