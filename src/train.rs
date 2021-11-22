use crate::{
	chance_to_rot,
	get_bit_frequencies,
	get_ints_from_frequencies,
	GatedCircuit,
	Model,
};
use q1tsim::{
	circuit::Circuit,
	error::Result,
};
use rayon::prelude::*;
use std::{
	marker::Sync,
	panic::catch_unwind,
};

pub(crate) fn train<Oracle, InputGiver, ModelManipulator>(
	oracle: Oracle,
	input_giver: InputGiver,
	model_manipulator: ModelManipulator,
	model: &Model,
	prev_stats: (f64, f64),
	consts: (usize, usize),
) -> Result<(Model, f64)>
where
	Oracle: Fn(&[i64], &[i64]) -> f64 + Sync,
	InputGiver: Fn() -> Vec<Vec<i64>>,
	ModelManipulator: Fn(&Model) -> Vec<Model>,
{
	let (prev_best, rolling_avg) = prev_stats;

	let models = model_manipulator(model);
	let inputs = input_giver();

	let (best_model, best_val) = models
		.into_par_iter()
		.map(|model| {
			let oracle_val = inputs
				.par_iter()
				.map(|input| {
					let output = catch_unwind(|| run(&model, input, consts))
						.unwrap_or_else(|err| panic!("{:?}\n{:?}", err, model))?;
					Ok(oracle(input, &output))
				})
				.collect::<Result<Vec<f64>>>()?
				.into_iter()
				.fold(0.0, |sum, value| sum + value)
				/ (inputs.len() as f64);

			let val = valuate(&model, oracle_val); // Value consists of oracle score weighted against number of qubits and number of gates
			Ok((model, val))
		})
		.collect::<Result<Vec<(Model, f64)>>>()?
		.into_iter()
		.max_by_key(|(model, value)| (value * 1000.0) as i64)
		.unwrap_or((model.clone(), 0.0));

	//println!("{:?}: {:?}", best_val, best_model);

	let change = best_val - prev_best;
	let rolling_avg = (rolling_avg + change) / 2.0;
	if rolling_avg > -0.01 && best_val < 0.8 {
		train(
			oracle,
			input_giver,
			model_manipulator,
			&best_model,
			(best_val, rolling_avg),
			consts,
		)
	} else {
		Ok((best_model, best_val))
	}
}

pub(crate) fn run(model: &Model, inputs: &[i64], consts: (usize, usize)) -> Result<Vec<i64>> {
	let (shots, accuracy) = consts;
	let input_len = inputs.len();

	let mut circuit = Circuit::new(model.qbits, model.cbits);
	for (qbit, input) in inputs.iter().enumerate() {
		circuit.ry(chance_to_rot(*input as f64 / accuracy as f64), qbit)?;
	}

	for gate in &model.gates {
		circuit.run_gate(gate)?;
	}

	for (qbit, cbit) in &model.measure_at_end {
		circuit.measure(*qbit, *cbit)?;
	}

	circuit.execute(shots)?;

	let result = circuit.histogram_string()?;
	let freqs: Vec<f64> = get_bit_frequencies(result, input_len, shots);

	Ok(get_ints_from_frequencies(freqs, accuracy))
}

fn valuate(model: &Model, oracle_val: f64) -> f64 {
	// TODO: Cleanup and justify
	let oracle_val = oracle_val.powf(4.0);
	let qbit_val = (model.qbits as f64 / 16.0).powf(4.0);
	let gate_val = (model.gates.len() as f64 / 64.0).powf(4.0);
	oracle_val - (qbit_val + gate_val)
}
