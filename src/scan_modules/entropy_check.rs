use errors::ArmorlibError;
use scan_module::ScanModule;
use scan_object::ScanObject;
use finding::{Finding, Severity};
use ArmorlibError::ParseError;

pub struct EntropyCheckScanModule;

impl ScanModule for EntropyCheckScanModule {
	fn scan(&self, scan_object: &ScanObject) -> Result<Vec<Finding>, ArmorlibError> {
		let mut findings: Vec<Finding> = Vec::new();

		let data = match scan_object.get_metadata("entropy_cal/shan") {
			Ok(f) => f,
			Err(err) => {
				println!("Unable to retrieve Metric_entropy field. Error: {}",err);	
				return Err(err);
			}
		};
		

		let conv_data = match data.parse::<f32>() {
									Ok(val) => val,
									Err(err) => {
										return Err(ParseError(format!("Unable to parse string value to f32. Error: {}",err)));
									}
								};

		if conv_data < 3.0 {
					findings.push(Finding {
								title: String::from("Low entropy detected"),
								description: format!("the shannon entropy , {}, is lower than average",conv_data),
								id: String::from("LOW_ENTROPY"),
								severity: Severity::Warn(String::from("may want to increase entropy if sensitive information is present"))
							});
		}
		else if conv_data > 6.0 && conv_data < 7.2 {
			findings.push(Finding {
								title: String::from("Moderately high entropy detected"),
								description: format!("the shannon entropy , {}, is moderately higher than average",conv_data),
								id: String::from("MODERATELY_HIGH_ENTROPY"),
								severity: Severity::Ok(String::from("may indicate: compressed file"))
							});
		}
		else if conv_data > 7.2 {
				findings.push(Finding {
								title: String::from("high entropy detected"),
								description: format!("the shannon entropy , {}, is higher than average",conv_data),
								id: String::from("HIGH_ENTROPY"),
								severity: Severity::Ok(String::from("may indicate: encrypted or malicious file"))
							});
		}	
		Ok(findings)
				

	}

	fn info(&self) -> (&'static str, &'static str) {
		("entropy_check","determines if shannon entropy is out of normal range")
	}

	fn required_preprocessors(&self) -> Vec<&'static str> {
		vec!["entropy_cal"]
	}

}
