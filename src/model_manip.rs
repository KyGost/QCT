use crate::*;

pub(crate) fn action_upon_model(model: Model) -> Model {
	match fastrand::usize(..5) {
		0 => increase_qbits(model),
		1 => decrease_qbits(model),
		2 => add_gate(model),
		3 => change_gate(model),
		4 => remove_gate(model),
		_ => panic!(),
	}
}
fn increase_qbits(mut model: Model) -> Model {
	model.qbits += 1;
	model
}
fn decrease_qbits(mut model: Model) -> Model {
	if model.qbits > 1 {
		//model.qbits -= 1; TODO: Remove qubit and remove any gates with it
	}
	model
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
