use gtk::prelude::*;

pub fn build_ui(application: &gtk::Application) {

	let value_button = gtk::Button::new();
	value_button.set_label("values");
	
	let graph_button = gtk::Button::new();
	graph_button.set_label("graphs");


	let value_window = gtk::Box::new(gtk::Orientation::Horizontal, 0);
	value_window.pack_start(&value_button, false, false, 0);
		
	let graph_window = gtk::Box::new(gtk::Orientation::Vertical, 0);
	graph_window.pack_start(&graph_button, false, false, 0);	
	
	let stack = gtk::Stack::new();
	stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
	stack.set_transition_duration(500);
	
	stack.add_titled(&value_window, "value_page", "Stock Value");
	stack.add_titled(&graph_window, "graph_page", "Stock Graph");
	
	let stack_switcher = gtk::StackSwitcher::new();
	stack_switcher.set_stack(Some(&stack));

	let headerbar = gtk::HeaderBar::new();
	headerbar.set_title(Some("Finance"));
	headerbar.set_show_close_button(true);
	headerbar.set_custom_title(Some(&stack_switcher));

	let window = gtk::ApplicationWindow::new(application);
	window.set_title("Finance");
	window.set_icon_name(Some("org.skylinecc.finance"));
	window.set_border_width(10);
	window.set_titlebar(Some(&headerbar));
	window.set_default_size(1050, 650);
	
        window.add(&stack);
	window.show_all();
}
