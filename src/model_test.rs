use core::num;

use crate::*;
use bincode::Options;
use dialoguer::{
	theme::ColorfulTheme,
	Input,
};
use q1tsim::error::Result;

pub(crate) fn test_model() -> Result<()> {
	let bincoder = bincode::DefaultOptions::new();
	let model_base64: String = Input::with_theme(&ColorfulTheme::default())
		.with_prompt(String::from("Model"))
		.interact()
		.unwrap();
	let model = bincoder
		.deserialize(&base64::decode(model_base64).unwrap())
		.unwrap();
	let mut inputs = vec![];
	for num in 0..INPUT_CNT {
		inputs.push(
			Input::with_theme(&ColorfulTheme::default())
				.with_prompt(format!("Input {}", num))
				.interact()
				.unwrap(),
		);
	}
	let results = run(&model, &inputs, (SHOTS * 100, ACCURACY));
	println!("{:?}", results);
	Ok(())
}
