use crate::utils;
use gtk::prelude::*;
use polars::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub fn render_page(window: &gtk::ApplicationWindow, _df_cell: Rc<RefCell<Option<DataFrame>>>) {
    let vbox = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .margin(10)
        .spacing(10)
        .build();

    let train_button = gtk::ButtonBuilder::new().label("Train").build();
    vbox.pack_start(&train_button, false, false, 0);

    let training_progress = gtk::ProgressBarBuilder::new()
        .name("Hello")
        .text("Training not started")
        .show_text(true)
        .build();
    vbox.pack_start(&training_progress, false, false, 0);

    let test_button = gtk::ButtonBuilder::new().label("Test").build();
    vbox.pack_start(&test_button, false, false, 0);

    let scroll_window = gtk::ScrolledWindowBuilder::new()
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .build();
    vbox.pack_start(&scroll_window, true, true, 0);

    window.add(&utils::wrap_in_header(
        "Model",
        "Train and test a linear regression model on the data",
        &vbox,
    ));
    window.show_all();
}
