use errors::ArmorlibError;
use scan_module::ScanModule;
use scan_object::ScanObject;
use finding::{Finding,Severity};

pub struct ExtensionCheckScanModule;

impl ScanModule for ExtensionCheckScanModule {

	fn scan(&self, scan_object: &ScanObject) -> Result<Vec<Finding>, ArmorlibError> {
		let mut findings: Vec<Finding> = Vec::new();

		let user_filetype = match &scan_object.filetype {
			Some(user_filetype) => user_filetype,
				None => "NO_EXTEN",
		};

		if user_filetype == "NO_EXTEN" {return Ok(findings);}
		let detec_file_types = &scan_object.detected_filetypes;
		if detec_file_types.len() == 0 {

			findings.push(Finding {
											title: String::from("No file type:"),
											description: format!("No file types were detected to match user extension: {}",user_filetype),
											id: String::from("NO_FILETYPE"),
											severity: Severity::Ok(String::from("may indicate: error in preprocessing or not enough information to determine file type."))
											});
		} 
		
		let mut flag = 0;
		if detec_file_types.len() > 0 { 
			for filetype in detec_file_types.iter() {
				if user_filetype == "cpp" && filetype == "c" {
					continue;
				}					
				else if user_filetype != *filetype {
					
					flag = 1; 
				}
			}
			if flag == 1 {
				findings.push(Finding {
												title: String::from("Mismatch Found:"),
												description: format!("the file extension {}, does not match the filetype detected : {:?}",user_filetype,detec_file_types),
												id: String::from("EXTENSION_MISMATCH"),
												severity: Severity::Warn(String::from("may indicate: malicious file or extension mismatch"))
											});			
			}
		}
		Ok(findings)
	}


	fn info(&self) -> (&'static str, &'static str) {
		("exten_check", "Checks file extension against file magic.")
	}

	fn required_preprocessors(&self) -> Vec<&'static str> {
		vec!["filetype"]
	}
}
