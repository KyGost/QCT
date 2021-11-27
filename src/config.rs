use crate::{
	oracle,
	Oracle,
};
use dialoguer::{
	theme::ColorfulTheme,
	Input,
};

pub(crate) struct Config {
	pub models: usize,
	pub iterations: usize,
	pub accuracy: usize,
	pub shots: usize,
	pub oracle: Oracle,
}
impl Config {
	pub fn new(
		models: usize,
		iterations: usize,
		accuracy: usize,
		shot_multiplier: usize,
		oracle: Oracle,
	) -> Self {
		Self {
			models,
			iterations,
			accuracy,
			shots: accuracy * shot_multiplier,
			oracle,
		}
	}
	pub fn new_via_input() -> Self {
		Self::new(
			Input::with_theme(&ColorfulTheme::default())
				.with_prompt("Models")
				.default(10)
				.interact()
				.unwrap(),
			Input::with_theme(&ColorfulTheme::default())
				.with_prompt("Iterations")
				.default(1000)
				.interact()
				.unwrap(),
			Input::with_theme(&ColorfulTheme::default())
				.with_prompt("Accuracy")
				.default(1000)
				.interact()
				.unwrap(),
			Input::with_theme(&ColorfulTheme::default())
				.with_prompt("Shot Multiplier")
				.default(10)
				.interact()
				.unwrap(),
			oracle(
				&Input::with_theme(&ColorfulTheme::default())
					.with_prompt(String::from("Oracle"))
					.default(String::from("Sum"))
					.interact()
					.unwrap(),
			),
		)
	}
}
