extern crate gtk;
extern crate gio;
extern crate glib;

use gio::prelude::*;
use gtk::{SettingsExt, Settings};
use std::env::args;

mod ui;

fn main() {
	let application = gtk::Application::new(Some("com.skylinecc.finance"), Default::default())
		.expect("Initialization failed...");
	
	application.connect_activate(|app| {
		ui::build_ui(app);
	});

	application.connect_startup(|_| {
		Settings::get_default()
		.unwrap()
		.set_property_gtk_application_prefer_dark_theme(true);
	});


	application.run(&args().collect::<Vec<_>>());
}

pub fn pr() {
	println!("Works in main.rs");
}
