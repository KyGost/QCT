use crate::*;
use q1tsim::error::Result;

pub(crate) fn test_model() -> Result<()> {
	use Gate::*;
	let model = Model {
		gates: vec![
			CX(0, 1),
			U(
				0.22703964107094474,
				1.9715433431995129,
				0.45084032752183745,
				0,
			),
			CX(0, 2),
			CX(1, 2),
			CCX(1, 0, 2),
			X(1),
			CCX(1, 2, 0),
			CCX(1, 0, 2),
		],
		qbits: 3,
		cbits: 1,
		measure_at_end: vec![(2, 0)],
	};
	let results = run(&model, &[20, 74], (SHOTS * 100, ACCURACY));
	println!("{:?}", results);
	Ok(())
}
