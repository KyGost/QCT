pub(crate) use crate::{
	model::*,
	model_manip::*,
	train::*,
	util::*,
};
use dialoguer::{
	theme::ColorfulTheme,
	Input,
	Select,
};
use indicatif::ProgressBar;
use q1tsim::error::Result;
use std::f64::consts::PI;

mod model;
mod model_manip;
mod oracles;
mod train;
mod util;

const SHOTS: usize = 2000;
const ACCURACY: usize = 200;

// TODO: Make an area for making new models and an area for refining existing
fn main() -> Result<()> {
	match Select::with_theme(&ColorfulTheme::default())
		.items(&["Create", "Refine", "Test"])
		.default(2)
		.interact()
		.unwrap()
	{
		0 => make_many_models(
			Input::new()
				.with_prompt("Iterations")
				.default(1000)
				.interact()
				.unwrap(),
		),
		1 => Ok(()),
		2 => test_model(),
		_ => panic!(),
	}
}

fn test_model() -> Result<()> {
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

fn make_many_models(number: u64) -> Result<()> {
	let progress = ProgressBar::new(number)
		.with_style(
			indicatif::ProgressStyle::default_bar()
				.template(
					"[{elapsed_precise}][{per_sec}] {bar:50.cyan/blue} {pos:>7}/{len:7} {msg}",
				)
				.progress_chars("##-"),
		)
		.with_message("Making models");
	progress.set_draw_rate(1);

	let mut iters = 0;
	let mut models = vec![];
	while iters < number {
		iters += 1;
		models.push(make_model()?);
		progress.inc(1);
	}
	progress.finish();
	Ok(())
}

fn make_model() -> Result<()> {
	let model = Model {
		qbits: 3,
		cbits: 1,
		gates: vec![],
		measure_at_end: vec![(2, 0)],
	};

	let input_supplier = || {
		let mut inputs = vec![];
		for _ in 0..10 {
			inputs.push(vec![
				fastrand::usize(0..((ACCURACY as f64).powf(0.5)) as usize) as i64,
				fastrand::usize(0..((ACCURACY as f64).powf(0.5)) as usize) as i64,
			])
		}
		inputs
	};

	let (model, val) = train(
		oracles::multiply_evaluator,
		input_supplier,
		default_model_manipulator,
		&model,
		(0.0, 0.5),
		(SHOTS, ACCURACY),
	)?;
	if val > 0.5 {
		println!("Produced {} model: {:?}", val, model); // TODO: Give a list of best few
	}
	Ok(())
}

fn default_model_manipulator(model: &Model) -> Vec<Model> {
	vec![
		model.clone(),
		model.clone(),
		model.clone(),
		action_upon_model(model.clone()),
		action_upon_model(action_upon_model(model.clone())),
		action_upon_model(action_upon_model(model.clone())),
		action_upon_model(action_upon_model(model.clone())),
		action_upon_model(action_upon_model(action_upon_model(model.clone()))),
		action_upon_model(action_upon_model(action_upon_model(action_upon_model(
			model.clone(),
		)))),
	]
}
