pub(crate) use crate::{
	model::*,
	model_manip::*,
	train::*,
	util::*,
};
use q1tsim::error::Result;
use std::{
	cmp::{
		max,
		min,
	},
	f64::consts::PI,
};

mod model;
mod model_manip;
mod train;
mod util;

const SHOTS: usize = 10000;
const ACCURACY: usize = 100;
fn main() -> Result<()> {
	loop {
		println!("New model");
		make_model()?;
	}
	Ok(())
}

fn make_model() -> Result<()> {
	let model = Model {
		qbits: 3,
		cbits: 1,
		gates: vec![],
		measure_at_end: vec![(2, 0)],
	};

	let sum_evaluator = |input: &[i64], output: &[i64]| {
		// Input should be two numbers, output should be their addition
		let sum = *input.get(0).unwrap_or(&0) + *input.get(1).unwrap_or(&0);
		let result = *output.get(0).unwrap_or(&0);
		if result == 0 {
			if sum == 0 { 1.0 } else { 0.0 }
		} else {
			min(sum, result) as f64 / max(sum, result) as f64
		}
	};

	let input_supplier = || vec![vec![5, 86], vec![74, 63], vec![11, 2], vec![45, 7]];

	train(
		sum_evaluator,
		input_supplier,
		default_model_manipulator,
		&model,
		(0.0, 0.5),
		(SHOTS, ACCURACY),
	)?;
	Ok(())
}

fn default_model_manipulator(model: &Model) -> Vec<Model> {
	vec![
		model.clone(),
		action_upon_model(model.clone()),
		action_upon_model(model.clone()),
		action_upon_model(model.clone()),
		action_upon_model(model.clone()),
		action_upon_model(action_upon_model(model.clone())),
		action_upon_model(action_upon_model(model.clone())),
		action_upon_model(action_upon_model(model.clone())),
		action_upon_model(action_upon_model(model.clone())),
		action_upon_model(action_upon_model(action_upon_model(model.clone()))),
		action_upon_model(action_upon_model(action_upon_model(model.clone()))),
		action_upon_model(action_upon_model(action_upon_model(action_upon_model(
			model.clone(),
		)))),
		action_upon_model(action_upon_model(action_upon_model(action_upon_model(
			model.clone(),
		)))),
		action_upon_model(action_upon_model(action_upon_model(action_upon_model(
			action_upon_model(model.clone()),
		)))),
		action_upon_model(action_upon_model(action_upon_model(action_upon_model(
			action_upon_model(model.clone()),
		)))),
	]
}
