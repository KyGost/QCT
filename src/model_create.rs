use crate::*;
use csv::Writer;
use dialoguer::{
	theme::ColorfulTheme,
	Confirm,
	Input,
	Select,
};
use indicatif::ProgressBar;
use q1tsim::error::Result;
use rayon::prelude::*;

pub(crate) fn make_many_models(number: u64) -> Result<()> {
	setup_inputs();

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
	models.par_sort_unstable_by(|a, b| b.0.overall.partial_cmp(&a.0.overall).unwrap());
	let mut top_vals = models.clone();
	top_vals.truncate(5);
	let top_vals = top_vals
		.into_iter()
		.map(|(val, _)| val.overall)
		.collect::<Vec<f64>>();

	println!("{:?}", top_vals);

	if Confirm::with_theme(&ColorfulTheme::default())
		.with_prompt("Save Results")
		.default(true)
		.interact()
		.unwrap()
	{
		let name = Input::with_theme(&ColorfulTheme::default())
			.with_prompt(String::from("Name"))
			.default(String::from("output"))
			.interact()
			.unwrap();
		let mut writer = Writer::from_path(format!("output/{}.csv", name)).unwrap();
		models.into_iter().for_each(|(val, model)| {
			writer
				.write_record(&[
					format!("{}", val.overall),
					format!("{}", val.oracle),
					format!("{:?}", model),
				])
				.unwrap();
		});
		writer.flush().unwrap();
	}

	Ok(())
}

fn make_model() -> Result<(Value, Model)> {
	let model = Model {
		qbits: 3,
		cbits: 1,
		gates: vec![],
		measure_at_end: vec![(2, 0)],
	};

	let (model, val) = train(
		oracles::sum_evaluator,
		input_supplier,
		default_model_manipulator,
		&model,
		(Value::new(0.0, 0, 0), 0.5, 1),
		(SHOTS, ACCURACY),
		ITERS,
	)?;
	Ok((val, model))
}

fn default_model_manipulator(model: &Model) -> Vec<Model> {
	vec![
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

static mut INPUTS: Vec<Vec<u64>> = vec![];
fn setup_inputs() {
	unsafe {
		for _ in 0..10 {
			// TODO: Add config
			INPUTS.push(vec![
				fastrand::usize(0..ACCURACY / 2) as u64,
				fastrand::usize(0..ACCURACY / 2) as u64,
			]);
		}
	}
}

fn input_supplier(iters: u8) -> Vec<Vec<u64>> {
	unsafe { INPUTS.clone() }
}
