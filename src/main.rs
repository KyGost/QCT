pub(crate) use crate::{
	model::*,
	model_manip::*,
	train::*,
	util::*,
	valuate::*,
};
use dialoguer::{
	theme::ColorfulTheme,
	Confirm,
	Input,
	Select,
};
use model_create::*;
use model_refine::*;
use model_test::*;
use q1tsim::error::Result;
use rayon::prelude::*;
use std::f64::consts::PI;

mod model;
mod model_create;
mod model_manip;
mod model_refine;
mod model_test;
mod oracles;
mod train;
mod util;
mod valuate;

const SHOTS: usize = 2000;
const ACCURACY: usize = 200;
const ITERS: u8 = 20;

// TODO: Make an area for making new models and an area for refining existing
fn main() -> Result<()> {
	match Select::with_theme(&ColorfulTheme::default())
		.items(&["Create", "Refine", "Test"])
		.default(2)
		.interact()
		.unwrap()
	{
		0 => make_many_models(
			Input::with_theme(&ColorfulTheme::default())
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
