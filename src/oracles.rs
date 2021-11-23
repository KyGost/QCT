use std::cmp::{
	max,
	min,
};

pub fn sum_evaluator(input: &[u64], output: &[u64]) -> f64 {
	// Input should be two numbers, output should be their addition
	let target = *input.get(0).unwrap_or(&0) + *input.get(1).unwrap_or(&0);
	let result = *output.get(0).unwrap_or(&0);
	if result == 0 {
		if target == 0 { 1.0 } else { 0.0 }
	} else {
		min(target, result) as f64 / max(target, result) as f64
	}
}

pub fn multiply_evaluator(input: &[u64], output: &[u64]) -> f64 {
	// Input should be two numbers, output should be their multiplication
	let target = *input.get(0).unwrap_or(&0) * *input.get(1).unwrap_or(&0);
	let result = *output.get(0).unwrap_or(&0);
	if result == 0 {
		if target == 0 { 1.0 } else { 0.0 }
	} else {
		min(target, result) as f64 / max(target, result) as f64
	}
}
