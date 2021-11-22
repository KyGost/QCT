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
use std::panic::catch_unwind;

pub(crate) fn train<Oracle, InputGiver, ModelManipulator>(
	oracle: Oracle,
	input_giver: InputGiver,
	model_manipulator: ModelManipulator,
	model: &Model,
	prev_best_val: f64,
	consts: (usize, usize),
) -> Result<(Model, f64)>
where
	Oracle: Fn(&[i64], &[i64]) -> f64,
	InputGiver: Fn() -> Vec<Vec<i64>>,
	ModelManipulator: Fn(&Model) -> Vec<Model>,
{
	let models = model_manipulator(model);
	let inputs = input_giver();

	let (best_model, best_val) = models
		.into_iter()
		.map(|model| {
			let value = inputs
				.iter()
				.map(|input| {
					let output = catch_unwind(|| run(&model, input, consts))
						.unwrap_or_else(|err| panic!("{:?}\n{:?}", err, model))?;
					Ok(oracle(input, &output))
				})
				.try_fold(0.0, |sum, value: Result<f64>| {
					value.map(|value| sum + value)
				})? / (inputs.len() as f64);

			//println!("{}", value);

			Ok((model, value))
		})
		.collect::<Result<Vec<(Model, f64)>>>()?
		.into_iter()
		.max_by_key(|(model, value)| (value * 1000.0) as i64)
		.unwrap_or((model.clone(), 0.0));

	println!("{:?}: {:?}", best_val, best_model);

	if best_val > prev_best_val {
		train(
			oracle,
			input_giver,
			model_manipulator,
			&best_model,
			best_val,
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
