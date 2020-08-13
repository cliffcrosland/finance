use gtk::prelude::*;
use gtk::Label;

pub fn build_ui(application: &gtk::Application) {

//	Value Section
	let stock_code = "wxyz";
	let stock_value = "12.98";
	
	let stock_code_markup = format!("<span weight=\"heavy\" font=\"72\">{}</span>", stock_code);
	let stock_value_markup = format!("<span weight=\"normal\" font=\"12\">${}</span>", stock_value);
	
	let stock_code_label = Label::new(None);
	stock_code_label.set_markup(&stock_code_markup);
	
	let stock_value_label = Label::new(None);
	stock_value_label.set_markup(&stock_value_markup);
	
	let value_window = gtk::Box::new(gtk::Orientation::Vertical, 0);
	value_window.pack_start(&stock_code_label, false, false, 0);
	value_window.pack_start(&stock_value_label, false, false, 0);

//	Graph Section
	let graph_button = gtk::Button::new();
	graph_button.set_label("graphs");

	let graph_window = gtk::Box::new(gtk::Orientation::Vertical, 0);
	graph_window.pack_start(&graph_button, false, false, 0);	

//	Define HeaderBar Stack	
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

//	Basic ApplicationWindow properties
	let window = gtk::ApplicationWindow::new(application);
	window.set_title("Finance");
	window.set_icon_name(Some("org.skylinecc.finance"));
	window.set_border_width(10);
	window.set_titlebar(Some(&headerbar));
	window.set_default_size(1050, 650);
	
        window.add(&stack);
	window.show_all();
}
