extern crate gio;
extern crate gtk;

use std::env::args;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
	ApplicationWindow, CellRendererText, SortColumn, TreeModelSort, TreeStore, TreeView,
	TreeViewColumn, WindowPosition,
};

enum Columns {
	Text = 0,
}

fn build_ui(application: &gtk::Application) {
	let window = ApplicationWindow::new(application);

	window.set_title("TreeView panic reproduce");
	window.set_position(WindowPosition::Center);

	let my_store = TreeStore::new(&[String::static_type()]);
	let my_store_sort = TreeModelSort::new(&my_store);
	let my_tree_view = TreeView::new_with_model(&my_store_sort);
	my_tree_view.set_headers_visible(true);

	my_store_sort.set_sort_func(
		SortColumn::Index(Columns::Text as u32),
		move |_w, _l_it, _r_it| {
			println!("Comparison executed!");
			core::cmp::Ordering::Equal
		},
	);

	let column = TreeViewColumn::new();
	column.set_title("Column header title");
	column.set_sort_indicator(true);
	column.set_clickable(true);
	column.set_sort_column_id(Columns::Text as i32);

	let renderer_text = CellRendererText::new();
	column.pack_start(&renderer_text, false);
	column.add_attribute(&renderer_text, "text", Columns::Text as i32);

	// https://github.com/gtk-rs/gtk/issues/701
	// Must insert at least one entry, or else it crashes on startup (different bug)
	// https://github.com/gtk-rs/gtk/issues/960
	// Must insert at least two entries to trigger sort panic when column header is clicked
	for string in vec!["Test1".to_string(), "Test2".to_string()] {
		my_store.insert_with_values(None, None, &[Columns::Text as u32], &[&string]);
	}
	my_tree_view.append_column(&column);

	window.add(&my_tree_view);
	window.show_all();
}

fn main() {
	let application = gtk::Application::new(
		Some("com.github.BenjaminRi.sort_func_panic"),
		Default::default(),
	)
	.expect("Initialization failed...");

	application.connect_activate(|app| {
		build_ui(app);
	});

	application.run(&args().collect::<Vec<_>>());
}
