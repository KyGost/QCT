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
const ACCURACY: usize = 20;

// TODO: Make an area for making new models and an area for refining existing
fn main() -> Result<()> {
	let mut iters = 0;
	while iters < 100 {
		iters += 1;
		println!("Working on model {}", iters);
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

	let input_supplier = || {
		vec![
			vec![5, 4],
			vec![2, 3],
			vec![1, 9],
			vec![8, 7],
			vec![3, 7],
			vec![3, 10],
			vec![0, 2],
		]
	};

	let (model, val) = train(
		sum_evaluator,
		input_supplier,
		default_model_manipulator,
		&model,
		(0.0, 0.5),
		(SHOTS, ACCURACY),
	)?;
	if val > 0.4 {
		println!("Produced {} model: {:?}", val, model);
	}
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
