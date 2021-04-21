extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate polars;

use gio::prelude::*;
use gtk::prelude::*;
use polars::io::prelude::*;

use std::env;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.architbhonsle.gtk-rs-experiment"),
        Default::default(),
    )
    .unwrap();

    application.connect_activate(build_ui);

    application.run(&env::args().collect::<Vec<_>>());
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Logistic Regression");
    window.set_default_size(700, 600);

    let label = gtk::Label::new(Some("Drag a file below"));

    let text_view = gtk::TextView::new();
    text_view.set_cursor_visible(false);
    text_view.set_property_monospace(true);

    let scrolled_text_view = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolled_text_view.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_text_view.add(&text_view);

    let targets = vec![gtk::TargetEntry::new(
        "text/uri-list",
        gtk::TargetFlags::OTHER_APP,
        0,
    )];
    text_view.drag_dest_set(
        gtk::DestDefaults::HIGHLIGHT,
        &targets,
        gdk::DragAction::COPY,
    );

    text_view.connect_drag_data_received(|drag_context, _, _, _, selection_data, _, _| {
        let buffer = drag_context.get_buffer().unwrap();
        let file_uris = selection_data.get_uris();

        if file_uris.len() != 1 {
            panic!("Multiple files found");
        }

        let file = file_uris[0].to_owned();
        let file = gio::File::new_for_uri(&file);

        let dataframe =
            polars::io::csv::CsvReader::from_path(file.get_path().unwrap().to_str().unwrap())
                .unwrap()
                .infer_schema(None)
                .has_header(true)
                .finish()
                .unwrap();
        let csv = dataframe.to_string();

        buffer.set_text(&csv);
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.set_border_width(5);
    vbox.pack_start(&label, false, false, 0);
    vbox.pack_start(&scrolled_text_view, true, true, 0);

    window.add(&vbox);
    window.show_all();
}
