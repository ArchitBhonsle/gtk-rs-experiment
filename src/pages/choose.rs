use super::Pages;
use crate::utils;
use gio::prelude::*;
use gtk::prelude::*;
use polars::io::prelude::*;

pub fn processing_page(window: gtk::ApplicationWindow) -> super::Pages {
    let vbox = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .margin(10)
        .spacing(10)
        .build();

    let file_chooser = create_file_chooser();
    vbox.pack_start(&file_chooser, false, false, 0);

    let scroll_window = gtk::ScrolledWindowBuilder::new()
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .build();
    vbox.pack_start(&scroll_window, true, true, 0);
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

    let next_page_button = gtk::ButtonBuilder::new().label("Preprocessing").build();
    vbox.pack_start(&next_page_button, false, false, 0);

    window.add(&utils::wrap_in_header(
        "Choose a file",
        "Must be a csv file containing only numeric data",
        &vbox,
    ));
    window.show_all();

    Pages::Choose
}

pub fn create_file_chooser() -> gtk::FileChooserButton {
    let csv_filter = gtk::FileFilter::new();
    csv_filter.add_pattern("*.csv");

    gtk::FileChooserButtonBuilder::new()
        .title("Choose a CSV file")
        .filter(&csv_filter)
        .build()
}
