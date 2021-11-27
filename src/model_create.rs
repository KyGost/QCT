use crate::*;
use bincode::Options;
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
		let bincoder = bincode::DefaultOptions::new();
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
					format!("{}", base64::encode(bincoder.serialize(&model).unwrap())),
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
		qbits: INPUT_CNT + OUTPUT_CNT,
		cbits: OUTPUT_CNT,
		gates: vec![],
		measure_at_end: vec![(INPUT_CNT, 0)],
	};

	let (model, val) = train(
		oracles::if30_evaluator,
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
		action_upon_model(model.clone()),
		action_upon_model(model.clone()),
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

pub static mut INPUTS: Vec<Vec<u64>> = vec![];

fn input_supplier(_iters: usize) -> Vec<Vec<u64>> {
	unsafe { INPUTS.clone() }
}
