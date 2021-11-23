use crate::*;
use std::cmp::{
	max,
	min,
};

pub(crate) fn sum_evaluator(input: &[u64], output: &[u64]) -> f64 {
	// Input should be two numbers, output should be their addition
	let target = *input.get(0).unwrap_or(&0) + *input.get(1).unwrap_or(&0);
	let result = *output.get(0).unwrap_or(&0);
	let accuracy = if result == 0 {
		if target == 0 { 1.0 } else { 0.0 }
	} else {
		min(target, result) as f64 / max(target, result) as f64
	};
	((accuracy + 0.1).powf(2.0 / 3.0) + accuracy.powf(4.0)) / 2.0
}

pub(crate) fn multiply_evaluator(input: &[u64], output: &[u64]) -> f64 {
	// Input should be two numbers, output should be their multiplication
	let target = *input.get(0).unwrap_or(&0) * *input.get(1).unwrap_or(&0);
	let result = *output.get(0).unwrap_or(&0);
	let accuracy = if result == 0 {
		if target == 0 { 1.0 } else { 0.0 }
	} else {
		min(target, result) as f64 / max(target, result) as f64
	};
	((accuracy + 0.1).powf(2.0 / 3.0) + accuracy.powf(4.0)) / 2.0
}

pub(crate) fn x10_evaluator(input: &[u64], output: &[u64]) -> f64 {
	// Input should be two numbers, output should be their multiplication
	let target = *input.get(0).unwrap_or(&0) * 10;
	let result = *output.get(0).unwrap_or(&0);
	let accuracy = if result == 0 {
		if target == 0 { 1.0 } else { 0.0 }
	} else {
		min(target, result) as f64 / max(target, result) as f64
	};
	((accuracy + 0.1).powf(2.0 / 3.0) + accuracy.powf(4.0)) / 2.0
}

pub(crate) fn sum_setup_inputs() {
	unsafe {
		for _ in 0..(ACCURACY / 8) {
			// TODO: Add config
			model_create::INPUTS.push(vec![
				fastrand::u64(0..ACCURACY as u64 / 2),
				fastrand::u64(0..ACCURACY as u64 / 2),
			]);
		}
	}
}

pub(crate) fn doubled_sum_setup_inputs() {
	unsafe {
		for _ in 0..(ACCURACY / 8) {
			// TODO: Add config
			let val1 = fastrand::u64(0..ACCURACY as u64 / 10);
			let val2 = fastrand::u64(0..ACCURACY as u64 / 10);
			model_create::INPUTS.push(vec![val1, val2, val1, val2]);
		}
	}
}

pub(crate) fn x10_setup_inputs() {
	unsafe {
		for _ in 0..(ACCURACY / 8) {
			// TODO: Add config
			let val = fastrand::u64(0..ACCURACY as u64 / 10);
			model_create::INPUTS.push(vec![val, val]);
		}
	}
}
