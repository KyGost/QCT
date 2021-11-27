use crate::*;
use std::cmp::{
	max,
	min,
};

pub(crate) struct Oracle {
	pub inputs: usize,
	pub outputs: usize,
	pub evaluate: fn(&[u64], &[u64]) -> f64,
	pub get_inputs: fn(&Config, usize) -> Vec<Vec<u64>>,
	pub manipulate_models: fn(&Model) -> Vec<Model>,
}

pub(crate) fn oracle(oracle: &str) -> Oracle {
	match oracle {
		"Sum" => Oracle {
			inputs: 2,
			outputs: 2,
			evaluate: sum_evaluate,
			get_inputs: sum_get_inputs,
			manipulate_models: sum_manipulate_models,
		},
		_ => unimplemented!(),
	}
}

fn sum_evaluate(input: &[u64], output: &[u64]) -> f64 {
	let target = *input.get(0).unwrap_or(&0) + *input.get(1).unwrap_or(&0);
	let result = *output.get(0).unwrap_or(&0);
	let accuracy = if result == 0 {
		if target == 0 { 1.0 } else { 0.0 }
	} else {
		min(target, result) as f64 / max(target, result) as f64
	};
	((accuracy + 0.1).powf(2.0 / 3.0) + accuracy.powf(4.0)) / 2.0
}
fn sum_get_inputs(config: &Config, _iters: usize) -> Vec<Vec<u64>> {
	unsafe {
		if INPUTS.is_empty() {
			for _ in 0..(config.accuracy / 8) {
				// TODO: Add config
				INPUTS.push(vec![
					fastrand::u64(0..config.accuracy as u64 / 2),
					fastrand::u64(0..config.accuracy as u64 / 2),
				]);
			}
		}
		INPUTS.clone()
	}
}
fn sum_manipulate_models(model: &Model) -> Vec<Model> {
	default_model_manipulator(model)
}

fn default_model_manipulator(model: &Model) -> Vec<Model> {
	vec![
		action_upon_model(model.clone()),
		action_upon_model(model.clone()),
		action_upon_model(model.clone()),
		action_upon_model(action_upon_model(model.clone())),
		action_upon_model(action_upon_model(model.clone())),
		action_upon_model(action_upon_model(model.clone())),
		action_upon_model(action_upon_model(action_upon_model(model.clone()))),
		action_upon_model(action_upon_model(action_upon_model(action_upon_model(
			model.clone(),
		)))),
	]
}

pub static mut INPUTS: Vec<Vec<u64>> = vec![];
/*
pub(crate) fn triple_sum_evaluator(input: &[u64], output: &[u64]) -> f64 {
	// Input should be two numbers, output should be their addition
	let target =
		*input.get(0).unwrap_or(&0) + *input.get(1).unwrap_or(&0) + *input.get(2).unwrap_or(&0);
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

pub(crate) fn x2_evaluator(input: &[u64], output: &[u64]) -> f64 {
	// Input should be two numbers, output should be their multiplication
	let target = *input.get(0).unwrap_or(&0) * 2;
	let result = *output.get(0).unwrap_or(&0);
	let accuracy = if result == 0 {
		if target == 0 { 1.0 } else { 0.0 }
	} else {
		min(target, result) as f64 / max(target, result) as f64
	};
	((accuracy + 0.1).powf(2.0 / 3.0) + accuracy.powf(4.0)) / 2.0
}

pub(crate) fn if30_evaluator(input: &[u64], output: &[u64]) -> f64 {
	// Input should be two numbers, output should be their multiplication
	let closeness = (*input.get(0).unwrap_or(&0) as i64 - 30).abs();
	let confidence = *output.get(0).unwrap_or(&0) as f64 / ACCURACY as f64;
	if closeness < 15 {
		confidence
	//} else if closeness < 30 {
	//	((confidence - 0.5) / 2.0) + 0.5
	} else {
		(1.0 - confidence) / 2.0
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
			model_create::INPUTS.push(vec![val, val, 10]);
		}
	}
}

pub(crate) fn multiply_setup_inputs() {
	unsafe {
		for _ in 0..((ACCURACY as f64).powf(0.5) as u64) {
			// TODO: Add config
			model_create::INPUTS.push(vec![
				fastrand::u64(0..(ACCURACY as f64).powf(0.5) as u64),
				fastrand::u64(0..(ACCURACY as f64).powf(0.5) as u64),
			]);
		}
	}
}

pub(crate) fn triple_sum_setup_inputs() {
	unsafe {
		for _ in 0..(ACCURACY / 8) {
			// TODO: Add config
			model_create::INPUTS.push(vec![
				fastrand::u64(0..ACCURACY as u64 / 3),
				fastrand::u64(0..ACCURACY as u64 / 3),
				fastrand::u64(0..ACCURACY as u64 / 3),
			]);
		}
	}
}

pub(crate) fn triple_sum_weighted_setup_inputs() {
	unsafe {
		for _ in 0..(ACCURACY / 8) {
			// TODO: Add config
			model_create::INPUTS.push(vec![
				(fastrand::u64(0..ACCURACY as u64 / 3) as f64 * fastrand::f64()) as u64,
				(fastrand::u64(0..ACCURACY as u64 / 3) as f64 * fastrand::f64()) as u64,
				(fastrand::u64(0..ACCURACY as u64 / 3) as f64 * fastrand::f64()) as u64,
			]);
		}
	}
}

pub(crate) fn x2_setup_inputs() {
	unsafe {
		for _ in 0..(ACCURACY / 8) {
			// TODO: Add config
			model_create::INPUTS.push(vec![fastrand::u64(0..ACCURACY as u64 / 2), 2]);
		}
	}
}

pub(crate) fn if30_setup_inputs() {
	unsafe {
		for _ in 0..(ACCURACY / 2) {
			// TODO: Add config
			model_create::INPUTS.push(vec![fastrand::u64(0..ACCURACY as u64), 30]);
		}
	}
}
*/
