use q1tsim::{
	circuit::Circuit,
	error::Result,
	gates,
};
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
	pub gates: Vec<Gate>,
	pub qbits: usize,
	pub cbits: usize,
	pub measure_at_end: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gate {
	NoOP(),
	Measure(usize, usize),
	U(f64, f64, f64, usize),
	X(usize),
	CX(usize, usize),
	CCX(usize, usize, usize),
	Func(Func),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Func {
	Sum2(usize, usize, usize),
	Sum3(usize, usize, usize, usize),
}

pub trait CircuitRun {
	fn run(&self, circuit: &mut Circuit) -> Result<()>;
}

pub trait GatedCircuit {
	fn run_gate(&mut self, gate: &Gate) -> Result<()>;
}

impl CircuitRun for Gate {
	fn run(&self, circuit: &mut Circuit) -> Result<()> {
		use Gate::*;
		match self {
			NoOP() => Ok(()),
			Measure(q, c) => circuit.measure(*q, *c),
			U(p1, p2, p3, q) => circuit.u3(*p1, *p2, *p3, *q),
			X(q) => circuit.x(*q),
			CX(q1, q2) => circuit.cx(*q1, *q2),
			CCX(q1, q2, q3) => circuit.add_gate(gates::CCX::new(), &[*q1, *q2, *q3]),
			Func(func) => func.run(circuit),
		}
	}
}

impl CircuitRun for Func {
	fn run(&self, circuit: &mut Circuit) -> Result<()> {
		use self::Func::*;
		use Gate::*;
		match self {
			Sum2(i1, i2, o) => {
				circuit.run_gate(&CX(*i1, *o))?;
				circuit.run_gate(&CCX(*i1, *i2, *o))?;
				circuit.run_gate(&CX(*i2, *o))
			}
			Sum3(i1, i2, i3, o) => {
				circuit.run_gate(&CX(*i3, *o))?;
				circuit.run_gate(&CX(*i1, *o))?;
				circuit.run_gate(&CX(*i2, *i1))?;
				circuit.run_gate(&CX(*i2, *i3))?;
				circuit.run_gate(&CCX(*i1, *i3, *o))?;
				circuit.run_gate(&CX(*i2, *i3))
			}
		}
	}
}

impl GatedCircuit for Circuit {
	fn run_gate(&mut self, gate: &Gate) -> Result<()> {
		gate.run(self)
	}
}
