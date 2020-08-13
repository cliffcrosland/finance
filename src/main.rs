extern crate gtk;
extern crate gio;


use gio::prelude::*;

use std::env::args;

mod ui;

fn main() {
	let application = gtk::Application::new(Some("com.skylinecc.finance"), Default::default())
		.expect("Initialization failed...");
		
	application.connect_activate(|app| {
		ui::build_ui(app);
	});

	application.run(&args().collect::<Vec<_>>());
}
