use gtk::prelude::*;
use gtk::Label;

pub fn build_ui(application: &gtk::Application) {

    let sidebar_icon = gtk::Image::new();
    sidebar_icon.set_from_file("/usr/share/icons/hicolor/scalable/actions/view-left-pane-symbolic.svg");

    let sidebar_button = gtk::ToggleButton::new();
    sidebar_button.set_image(Some(&sidebar_icon));

//  Define Refresh Button
    let refresh_icon = gtk::Image::new();
    refresh_icon.set_from_icon_name(Some("view-refresh-symbolic"), gtk::IconSize::Button);

    let refresh_button = gtk::Button::new();
    refresh_button.set_image(Some(&refresh_icon));

//	Define Search Bar and Button
	let search_entry = gtk::SearchEntry::new();
	search_entry.set_activates_default(true);
	search_entry.set_width_chars(35);
	
	let search_bar = gtk::SearchBar::new();
	search_bar.add(&search_entry);

	let search_icon = gtk::Image::new();
	search_icon.set_from_icon_name(Some("system-search-symbolic"), gtk::IconSize::Button);

	let search_button = gtk::ToggleButton::new();
	search_button.set_image(Some(&search_icon));
	
	search_button.connect_toggled({
		let search_bar = search_bar.clone();
		let search_entry = search_entry.clone();

		move |btn| {
			if btn.get_active() {
				search_bar.set_search_mode(true);
				search_entry.grab_focus();
			} else {
				search_bar.set_search_mode(false);
			}
		}
	});

    search_entry.connect_search_changed(move |entry| {
        let en = entry.get_text();
        println!("{}", en)
    });


//	Explore Section
	let stock_symbol = "AAPL";
	let stock_value = crate::yfin::quote(stock_symbol.to_string());
	
	let stock_code_markup = format!("<span weight=\"heavy\" font=\"72\">{}</span>", stock_symbol);
	let stock_value_markup = format!("<span font=\"12\">{}</span>", stock_value);
	
	let stock_code_label = Label::new(None);
	stock_code_label.set_markup(&stock_code_markup);
	
	let stock_value_label = Label::new(None);
	stock_value_label.set_markup(&stock_value_markup);
	
	let explore_window = gtk::Box::new(gtk::Orientation::Vertical, 0);
    explore_window.pack_start(&search_bar, false, false, 0);
	explore_window.pack_start(&stock_code_label, false, false, 0);
	explore_window.pack_start(&stock_value_label, false, false, 0);

//	Portfolio Section
	let portfolio_button = gtk::Button::new();
	portfolio_button.set_label("graphs");

	let portfolio_window = gtk::Box::new(gtk::Orientation::Vertical, 0);
	portfolio_window.set_center_widget(Some(&portfolio_button));

//	Define HeaderBar Stack
	let stack = gtk::Stack::new();
	stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
	stack.set_transition_duration(500);
	stack.add_titled(&explore_window, "explore_page", "Explore");
	stack.add_titled(&portfolio_window, "portfolio_page", "Portfolio");

	let stack_switcher = gtk::StackSwitcher::new();
	stack_switcher.set_stack(Some(&stack));

	let window_frame = gtk::Box::new(gtk::Orientation::Vertical, 0);
	window_frame.pack_start(&stack, false, false, 0);

	let header_bar = gtk::HeaderBar::new();
	header_bar.set_title(Some("Finance"));
	header_bar.set_show_close_button(true);
	header_bar.set_custom_title(Some(&stack_switcher));
	header_bar.pack_start(&search_button);
    header_bar.pack_start(&sidebar_button);
	header_bar.pack_end(&refresh_button);

//	Basic ApplicationWindow properties
	let window = gtk::ApplicationWindow::new(application);
	window.set_title("Finance");
	window.set_icon_name(Some("org.skylinecc.finance"));
	window.set_border_width(0);
	window.set_titlebar(Some(&header_bar));
	window.set_default_size(1050, 650);

    window.add(&window_frame);
	window.show_all();
}
