pub(crate) use crate::{
	model::*,
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
mod train;
mod util;

const SHOTS: usize = 10000;
const ACCURACY: usize = 100;
fn main() -> Result<()> {
	let model = Model {
		qbits: 2,
		cbits: 1,
		gates: vec![],
		measure_at_end: vec![(0, 0)],
	};

	let sum_evaluator = |input: &[i64], output: &[i64]| {
		println!("{:?} {:?}", input, output);
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
		(SHOTS, ACCURACY),
	)
}

fn default_model_manipulator(model: &Model) -> Vec<Model> {
	vec![
		action_upon_a_gate(model.clone()),
		action_upon_a_gate(model.clone()),
		action_upon_a_gate(model.clone()),
		action_upon_a_gate(action_upon_a_gate(model.clone())),
		action_upon_a_gate(action_upon_a_gate(model.clone())),
		action_upon_a_gate(action_upon_a_gate(model.clone())),
		action_upon_a_gate(action_upon_a_gate(action_upon_a_gate(model.clone()))),
		action_upon_a_gate(action_upon_a_gate(action_upon_a_gate(model.clone()))),
		action_upon_a_gate(action_upon_a_gate(action_upon_a_gate(model.clone()))),
	]
}
fn action_upon_a_gate(model: Model) -> Model {
	match fastrand::usize(..3) {
		0 => add_gate(model),
		1 => change_gate(model),
		2 => remove_gate(model),
		_ => panic!(),
	}
}
fn add_gate(mut model: Model) -> Model {
	model.gates.push(rand_gate(&model));
	model
}
fn change_gate(mut model: Model) -> Model {
	let gate_len = model.gates.len();
	if gate_len > 0 {
		model.gates[fastrand::usize(..gate_len)] = rand_gate(&model); // TODO: Manipulate parameters and whatnot
	}
	model
}
fn remove_gate(mut model: Model) -> Model {
	let gate_len = model.gates.len();
	if gate_len > 0 {
		model.gates.remove(fastrand::usize(..gate_len));
	}
	model
}
fn rand_gate(model: &Model) -> Gate {
	match fastrand::usize(..4) {
		3 => Gate::U(
			fastrand::f64() * PI,
			fastrand::f64() * PI,
			fastrand::f64() * PI,
			fastrand::usize(..model.qbits),
		),
		0 => Gate::X(fastrand::usize(..model.qbits)),
		1 => {
			if model.qbits < 2 {
				return Gate::NoOP();
			}
			let q1 = unused_qbit(model.qbits, &[]);
			let q2 = unused_qbit(model.qbits, &[q1]);
			Gate::CX(q1, q2)
		}
		2 => {
			if model.qbits < 3 {
				return Gate::NoOP();
			}
			let q1 = unused_qbit(model.qbits, &[]);
			let q2 = unused_qbit(model.qbits, &[q1]);
			let q3 = unused_qbit(model.qbits, &[q1, q2]);
			Gate::CCX(q1, q2, q3)
		}
		_ => panic!(),
	}
}

fn unused_qbit(qbits: usize, used_qbits: &[usize]) -> usize {
	let qbit = fastrand::usize(..qbits);
	if !used_qbits.is_empty()
		&& used_qbits
			.iter()
			.find(|used_qbit| used_qbit == &&qbit)
			.is_some()
	{
		unused_qbit(qbits, used_qbits)
	} else {
		qbit
	}
}
