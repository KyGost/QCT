use q1tsim::{
	circuit::Circuit,
	error::Result,
	gates,
};

#[derive(Debug, Clone)]
pub struct Model {
	pub gates: Vec<Gate>,
	pub qbits: usize,
	pub cbits: usize,
	pub measure_at_end: Vec<(usize, usize)>,
}

#[derive(Debug, Clone)]
pub enum Gate {
	NoOP(),
	Measure(usize, usize),
	U(f64, f64, f64, usize),
	X(usize),
	CX(usize, usize),
	CCX(usize, usize, usize),
}
impl Gate {
	pub fn run(&self, circuit: &mut Circuit) -> Result<()> {
		use Gate::*;
		match self {
			NoOP() => Ok(()),
			Measure(q, c) => circuit.measure(*q, *c),
			U(p1, p2, p3, q) => circuit.u3(*p1, *p2, *p3, *q),
			X(q) => circuit.x(*q),
			CX(q1, q2) => circuit.cx(*q1, *q2),
			CCX(q1, q2, q3) => circuit.add_gate(gates::CCX::new(), &[*q1, *q2, *q3]),
		}
	}
}

pub trait GatedCircuit {
	fn run_gate(&mut self, gate: &Gate) -> Result<()>;
}

impl GatedCircuit for Circuit {
	fn run_gate(&mut self, gate: &Gate) -> Result<()> {
		gate.run(self)
	}
}
