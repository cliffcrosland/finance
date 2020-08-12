extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Finance");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(1050, 650);

//    let headerbar = gtk::HeaderBar::set_title("Finance");

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("com.skylinecc.finance"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
