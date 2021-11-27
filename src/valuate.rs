const MAX_QBITS: usize = 16; // Qubits are real slow on classical
const MAX_GATES: usize = 128;

#[derive(Clone, Debug)]
pub(crate) struct Value {
	pub oracle: f64,
	pub qbit: f64,
	pub gate: f64,
	pub overall: f64,
}

impl Value {
	pub fn new(oracle_val: f64, qbits: usize, gates: usize) -> Self {
		let qbit_val = qbits as f64 / MAX_QBITS as f64;
		let gate_val = gates as f64 / MAX_GATES as f64;
		let overall_val =
			oracle_val - ((((qbit_val + gate_val) / 2.0).powf(2.0) * oracle_val) / 2.0);
		Self {
			oracle: oracle_val,
			qbit: qbit_val,
			gate: gate_val,
			overall: overall_val,
		}
	}
}
