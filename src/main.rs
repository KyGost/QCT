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

// TODO: Make an area for making new models and an area for refining existing
fn main() -> Result<()> {
	make_many_models();
	Ok(())
}

fn test_model() -> Result<()> {
	let model = Model {
		gates: vec![
			Gate::U(
				0.14755214188660948,
				1.0541597080510559,
				0.3975370955526963,
				0,
			),
			Gate::CX(1, 2),
			Gate::CX(0, 2),
			Gate::CCX(0, 1, 2),
			Gate::X(1),
			Gate::CCX(0, 2, 1),
		],
		qbits: 3,
		cbits: 1,
		measure_at_end: vec![(2, 0)],
	};
	let results = run(&model, &[6, 9], (SHOTS, ACCURACY));
	println!("{:?}", results);
	Ok(())
}

fn make_many_models() -> Result<()> {
	let mut iters = 0;
	while iters < 1000 {
		iters += 1;
		if iters % 50 == 0 {
			println!("Working on model {}", iters);
		}
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
		let mut inputs = vec![];
		for _ in 0..10 {
			inputs.push(vec![
				fastrand::usize(0..(ACCURACY / 2)) as i64,
				fastrand::usize(0..(ACCURACY / 2)) as i64,
			])
		}
		inputs
	};

	let (model, val) = train(
		sum_evaluator,
		input_supplier,
		default_model_manipulator,
		&model,
		(0.0, 0.5),
		(SHOTS, ACCURACY),
	)?;
	if val > 0.65 {
		println!("Produced {} model: {:?}", val, model);
	}
	Ok(())
}

fn default_model_manipulator(model: &Model) -> Vec<Model> {
	vec![
		model.clone(),
		model.clone(),
		model.clone(),
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
