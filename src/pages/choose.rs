use super::Pages;
use crate::utils;
use gio::prelude::*;
use gtk::prelude::*;
use polars::io::prelude::*;

pub fn processing_page(window: gtk::ApplicationWindow) -> super::Pages {
    let csv_filter = gtk::FileFilter::new();
    csv_filter.add_pattern("*.csv");
    let file_chooser = gtk::FileChooserButtonBuilder::new()
        .title("Choose a CSV file")
        .filter(&csv_filter)
        .build();

    let scroll_window = gtk::ScrolledWindowBuilder::new()
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .build();
    let scroll_window_clone = scroll_window.clone();

    file_chooser.connect_file_set(move |file_chooser_closure| {
        if let Some(file) = file_chooser_closure.get_file() {
            let dataframe =
                polars::io::csv::CsvReader::from_path(file.get_path().unwrap().to_str().unwrap())
                    .unwrap()
                    .infer_schema(None)
                    .has_header(true)
                    .finish()
                    .unwrap();

            let tree_view = utils::create_tree_view(&dataframe);
            tree_view.show();

            scroll_window_clone.add(&tree_view);
        }
    });

    let vbox = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .build();
    vbox.pack_start(&file_chooser, false, false, 0);
    vbox.pack_start(&scroll_window, true, true, 0);

    window.add(&vbox);
    window.show_all();

    Pages::Choose
}
