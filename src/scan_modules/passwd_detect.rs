use errors::ArmorlibError;
use scan_module::ScanModule;
use scan_object::ScanObject;
use finding::{Finding,Severity};
use ArmorlibError::MissingMetadata;
pub struct PasswordCheckScanModule;

impl ScanModule for PasswordCheckScanModule {

	fn scan(&self, scan_object: &ScanObject) -> Result<Vec<Finding>, ArmorlibError> {
	
		let mut findings: Vec<Finding> = Vec::new();
		let stringtext = match scan_object.get_metadata("text/text") {
			Ok(st) => st,
				Err(err) => {
					return Err(MissingMetadata(format!("Unable to get metadata for text/text. Error: {}",err)));
				}
		};
		let temp_vec: Vec::<&str> = stringtext.split("\n").collect();

		let mut foundnum;
		
		for i in 0..temp_vec.len() {
			let loopstr = temp_vec[i].trim().to_string();
			if loopstr.contains("passw") {
				foundnum = i+1;

				findings.push(Finding {
																title: String::from("Found:"),
																description: format!("Potential password at line: {}", foundnum),
																id: String::from("Located Password keyword"),
																severity: Severity::Warn(String::from("Potential Password(s)"))
				});
			}
			else if loopstr.contains("pswd") {
				foundnum = i+1;

				findings.push(Finding {
																title: String::from("Found:"),
																description: format!("Potential password at line: {}", foundnum),
																id: String::from("Located Password keyword"),
																severity: Severity::Warn(String::from("Potential Password(s)"))
											});
			}
		}
		
		return Ok(findings);
		
	}


	fn info(&self) -> (&'static str, &'static str) {
		("password_check", "Checks for potential passwords located in file")
	}

	fn required_preprocessors(&self) -> Vec<&'static str> {
		vec!["text"]
	}

}
