use crate::temperature;

pub fn degrees_to_fahrenheit(degrees: f64, unit: temperature::Unit) -> f64 {
	let mut conversion: f64 = 0.0;

	match unit {
		temperature::Unit::Celsius => {
			conversion = degrees * (9.0 / 5.0) + 32.0;
		},
		temperature::Unit::Kelvin => {
			conversion = (degrees - 273.15) * (9.0 / 5.0) + 32.0;
		},
		temperature::Unit::Fahrenheit => {
			conversion = degrees;
		}
	}

	return conversion;
}

pub fn degrees_to_celsius(degrees: f64, unit: temperature::Unit) -> f64 {
	let mut conversion: f64 = 0.0;

	match unit {
		temperature::Unit::Fahrenheit => {
			conversion = (degrees - 32.0) * (5.0 / 9.0);
		},
		temperature::Unit::Kelvin => {
			conversion = degrees - 273.15;
		},
		temperature::Unit::Celsius => {
			conversion = degrees;
		}
	}
	

	return conversion;
}

pub fn degrees_to_kelvin(degrees: f64, unit: temperature::Unit) -> f64 {
	let mut conversion: f64 = 0.0;

	match unit {
		temperature::Unit::Celsius => {
			conversion = degrees + 273.15;
		},
		temperature::Unit::Fahrenheit => {
			conversion = (degrees - 32.0) * (5.0 / 9.0) + 273.15;
		},
		temperature::Unit::Kelvin => {
			conversion = degrees;
		}
	}

	return conversion;
}