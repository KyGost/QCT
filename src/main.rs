pub(crate) use crate::{
	config::*,
	model::*,
	model_manip::*,
	oracles::*,
	train::*,
	util::*,
	valuate::*,
};
use dialoguer::{
	theme::ColorfulTheme,
	Select,
};
use model_create::*;
//use model_refine::*;
use model_test::*;
use q1tsim::error::Result;
use rayon::prelude::*;
use std::f64::consts::PI;

mod config;
mod model;
mod model_create;
mod model_manip;
mod model_refine;
mod model_test;
mod oracles;
mod train;
mod util;
mod valuate;

// TODO: Make an area for making new models and an area for refining existing
fn main() -> Result<()> {
	match Select::with_theme(&ColorfulTheme::default())
		.items(&["Create", "Refine", "Test"])
		.default(0)
		.interact()
		.unwrap()
	{
		0 => model_create(Config::new_via_input()),
		1 => Ok(()),
		2 => test_model(Config::new_via_input()),
		_ => panic!(),
	}
}
