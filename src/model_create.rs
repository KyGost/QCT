use crate::*;
use bincode::Options;
use csv::Writer;
use dialoguer::{
	theme::ColorfulTheme,
	Confirm,
	Input,
};
use indicatif::ProgressBar;
use q1tsim::error::Result;

pub(crate) fn model_create(config: Config) -> Result<()> {
	let progress = ProgressBar::new(config.models as u64)
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
	while iters < config.models {
		iters += 1;
		models.push(make_model(&config)?);
		progress.inc(1);
	}
	progress.finish();

	models.par_sort_unstable_by(|a, b| b.0.overall.partial_cmp(&a.0.overall).unwrap());

	output_best_model_scores(&models);

	if Confirm::with_theme(&ColorfulTheme::default())
		.with_prompt("Save Results")
		.default(true)
		.interact()
		.unwrap()
	{
		save_models(&models);
	}

	Ok(())
}

fn output_best_model_scores(models: &Vec<(Value, Model)>) -> () {
	// Assumes models is sorted
	let mut top_vals = models.clone();
	top_vals.truncate(5);
	let top_vals = top_vals
		.into_iter()
		.map(|(val, _)| val.overall)
		.collect::<Vec<f64>>();

	println!("{:?}", top_vals);
}

fn save_models(models: &[(Value, Model)]) -> () {
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

fn make_model(config: &Config) -> Result<(Value, Model)> {
	let model = Model {
		qbits: config.oracle.inputs + config.oracle.outputs, // TODO: Don't use functions
		cbits: config.oracle.outputs,
		gates: vec![],
		measure_at_end: vec![(config.oracle.inputs, 0)],
	};

	let (model, val) = train(&config, &model, (Value::new(0.0, 0, 0), 0.5, 1))?;
	Ok((val, model))
}
