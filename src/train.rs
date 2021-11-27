use crate::{
	chance_to_rot,
	get_bit_frequencies,
	get_ints_from_frequencies,
	Config,
	GatedCircuit,
	Model,
	Value,
};
use q1tsim::{
	circuit::Circuit,
	error::Result,
};
use rayon::prelude::*;
use std::panic::catch_unwind;

pub(crate) fn train(
	config: &Config,
	model: &Model,
	prev_stats: (Value, f64, usize),
) -> Result<(Model, Value)> {
	let (prev_best, rolling_avg, iters) = prev_stats;

	let models = (config.oracle.manipulate_models)(model);
	let inputs = (config.oracle.get_inputs)(config, iters);

	let (best_model, best_val) = models
		.into_par_iter()
		.map(|model| {
			let oracle_val = inputs
				.par_iter()
				.map(|input| {
					let output = catch_unwind(|| run(config, &model, input))
						.unwrap_or_else(|err| panic!("{:?}\n{:?}", err, model))?;
					Ok((config.oracle.evaluate)(input, &output))
				})
				.collect::<Result<Vec<f64>>>()?
				.into_iter()
				.fold(0.0, |sum, value| sum + value)
				/ (inputs.len() as f64);

			let val = Value::new(oracle_val, model.qbits, model.gates.len()); // Value consists of oracle score weighted against number of qubits and number of gates
			Ok((model, val))
		})
		.collect::<Result<Vec<(Model, Value)>>>()?
		.into_iter()
		.max_by_key(|(_model, value)| (value.overall * 1000.0) as i64)
		.unwrap();

	let change = best_val.overall - prev_best.overall;
	let rolling_avg = (rolling_avg + change) / 2.0;
	if iters > config.iterations
		|| ((iters > 5 && best_val.overall < 0.2) || (iters > 3 && best_val.overall < 0.1)/*abort*/)
	// TODO: Config
	{
		Ok((best_model, best_val))
	} else {
		train(config, &best_model, (best_val, rolling_avg, iters + 1))
	}
}

pub(crate) fn run(config: &Config, model: &Model, inputs: &[u64]) -> Result<Vec<u64>> {
	let input_len = inputs.len();

	let mut circuit = Circuit::new(model.qbits, model.cbits);
	for (qbit, input) in inputs.iter().enumerate() {
		circuit.ry(chance_to_rot(*input as f64 / config.accuracy as f64), qbit)?;
	}

	for gate in &model.gates {
		circuit.run_gate(gate)?;
	}

	for (qbit, cbit) in &model.measure_at_end {
		circuit.measure(*qbit, *cbit)?;
	}

	circuit.execute(config.shots)?;

	let result = circuit.histogram_string()?;
	let freqs: Vec<f64> = get_bit_frequencies(result, input_len, config.shots);

	Ok(get_ints_from_frequencies(freqs, config.accuracy))
}
