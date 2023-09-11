use std::collections::HashMap;
use std::process::Command;
use preprocessor::Preprocessor;
use binary_object::BinaryObject;
pub struct FiletypePrepreprocessor;

/// Standard filetypes are paired by their magic number and typical file extension.
/// See https://en.wikipedia.org/wiki/List_of_file_signatures for more information.
/// Lowercase filetypes indicate standard file extensions, while uppercase filetypes
/// indicate filetypes that operate without an extension (for example, a Mach-O binary).

fn exten_check(exten:&str) -> &'static str{
	match exten {
		"ASCII" => "txt",  
			"GIF" => "gif", 
			"TIFF" => "tiff",  
			"JPEG" => "jpeg", 
			"Zip" => "zip", 
			"lzip" => "lz", 
			"RAR" => "rar", 
			"PNG" => "png", 
			"executable" => "exe",
			"PDF" => "pdf", 
			"tar" => "tar",  
			"C" => "c", 
			"Java" => "java", 
			"7-zip" => "7z", 
			"ISO" => "iso",  
			"compiled" => "class", 
			"PC" => "bmp",  
			"Image" => "img", 
			"Rich" => "rtf", 
			"System" => "sdi", 
			"XML" => "xml", 
			"NTFS" => "bin",
			"data" => "bin", 
			_ => "",
	}
}

impl Preprocessor for FiletypePrepreprocessor {
	/// Returns a map where the keys are file types (without the leading `.`).
	/// Values are not currently used.
	fn process(&self, binary_object: &BinaryObject) -> HashMap<String, String> {
		let mut map: HashMap<String, String> = HashMap::new();
		let f_path = match &binary_object.file_path {
			Some(f) => f,
				None => {
					println!("No file path was detected for filetype preprocessor.");
					return HashMap::new();
				}
		};
		let out = match Command::new("file")
			.args(vec!["-b",&f_path])
			.output() {
				Ok(out) => out,
					Err(err) => {
						println!("Unable to execute file command. Error: {}",err);
						return HashMap::new();
					}
			};
		let out_str = match std::str::from_utf8(&out.stdout) {
			Ok(string) => string.trim(),
				Err(err) => {
					println!("Unable to convert stdout to string. Error: {}",err);
					return HashMap::new();
				}
		};
		let det_exten = out_str.split(" ").collect::<Vec<&str>>();
		let check_exten = exten_check(det_exten[0]).to_string();
		if check_exten == "" {
			return HashMap::new();
		}
		map.insert(check_exten, String::from(""));

		map

	}

	fn info(&self) -> (&'static str, &'static str) {
		("filetype", "determines the filetype using the file command")
	}
}

