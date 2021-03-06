use crate::*;

pub(crate) fn action_upon_model(model: Model) -> Model {
	match fastrand::usize(..6) {
		0 => increase_qbits(model),
		1 => decrease_qbits(model),
		2 => add_gate(model),
		3 => change_gate(model),
		4 => remove_gate(model),
		5 => model, // NOOP
		_ => panic!(),
	}
}
fn increase_qbits(mut model: Model) -> Model {
	model.qbits += 1;
	model
}
fn decrease_qbits(mut model: Model) -> Model {
	if model.qbits > 5
	//(config.oracle.inputs + config.oracle.outputs)
	/* TODO: Config */
	{
		let remove = fastrand::usize(..model.qbits);
		model.gates = model
			.gates
			.into_iter()
			.filter_map(|gate| {
				use self::Func::*;
				use Gate::*;
				match gate {
					// TODO: Clean up somehow
					NoOP() => None, // not technically relevant but might as well.
					Func(func) => match func {
						Sum2(q1, q2, o) => {
							if q1 == remove || q2 == remove || o == remove {
								None
							} else {
								Some(Func(Sum2(
									if q1 > remove { q1 - 1 } else { q1 },
									if q2 > remove { q2 - 1 } else { q2 },
									if o > remove { o - 1 } else { o },
								)))
							}
						}
						Sum3(q1, q2, q3, o) => {
							if q1 == remove || q2 == remove || q3 == remove || o == remove {
								None
							} else {
								Some(Func(Sum3(
									if q1 > remove { q1 - 1 } else { q1 },
									if q2 > remove { q2 - 1 } else { q2 },
									if q3 > remove { q3 - 1 } else { q3 },
									if o > remove { o - 1 } else { o },
								)))
							}
						}
					},
					Measure(q, c) => {
						if q == remove {
							None
						} else {
							Some(Measure(if q > remove { q - 1 } else { q }, c))
						}
					}
					U(p1, p2, p3, q) => {
						if q == remove {
							None
						} else {
							Some(U(p1, p2, p3, if q > remove { q - 1 } else { q }))
						}
					}
					X(q) => {
						if q == remove {
							None
						} else {
							Some(X(if q > remove { q - 1 } else { q }))
						}
					}
					CX(q1, q2) => {
						if q1 == remove || q2 == remove {
							None
						} else {
							Some(CX(
								if q1 > remove { q1 - 1 } else { q1 },
								if q2 > remove { q2 - 1 } else { q2 },
							))
						}
					}
					CCX(q1, q2, q3) => {
						if q1 == remove || q2 == remove || q3 == remove {
							None
						} else {
							Some(CCX(
								if q1 > remove { q1 - 1 } else { q1 },
								if q2 > remove { q2 - 1 } else { q2 },
								if q3 > remove { q3 - 1 } else { q3 },
							))
						}
					}
				}
			})
			.collect();
		model.qbits -= 1;
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
		use Gate::*;
		model.gates[fastrand::usize(..gate_len)] =
			match model.gates[fastrand::usize(..gate_len)].clone() {
				U(p1, p2, p3, q) => U(
					p1 + ((fastrand::f64() - 0.5).powf(0.6) * p1),
					p2 + ((fastrand::f64() - 0.5).powf(0.6) * p2),
					p3 + ((fastrand::f64() - 0.5).powf(0.6) * p3),
					q,
				), // Only change values with non-qubit parameters // TODO: Range config // TODO: Make this actually reasonable // No idea if this actually works reasonably at all
				other => other,
			}
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
	match fastrand::usize(..5) {
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
		3 => Gate::U(
			fastrand::f64() * PI,
			fastrand::f64() * PI,
			fastrand::f64() * PI,
			fastrand::usize(..model.qbits),
		),
		4 => match fastrand::usize(..2) {
			0 => {
				if model.qbits < 3 {
					return Gate::NoOP();
				}
				let i1 = unused_qbit(model.qbits, &[]);
				let i2 = unused_qbit(model.qbits, &[i1]);
				let o = unused_qbit(model.qbits, &[i1, i2]);
				Gate::Func(Func::Sum2(i1, i2, o))
			}
			1 => {
				if model.qbits < 4 {
					return Gate::NoOP();
				}
				let i1 = unused_qbit(model.qbits, &[]);
				let i2 = unused_qbit(model.qbits, &[i1]);
				let i3 = unused_qbit(model.qbits, &[i1, i2]);
				let o = unused_qbit(model.qbits, &[i1, i2, i3]);
				Gate::Func(Func::Sum3(i1, i2, i3, o))
			}
			_ => panic!(),
		},
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
