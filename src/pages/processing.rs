use super::Pages;
use crate::utils;
use gtk::prelude::*;
use polars::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub fn processing_page(
    window: &gtk::ApplicationWindow,
    page_cell: Rc<RefCell<Pages>>,
    df_cell: Rc<RefCell<Option<DataFrame>>>,
) {
    let vbox = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .margin(10)
        .spacing(10)
        .build();

    window.add(&utils::wrap_in_header(
        "Processing",
        "Apply transformations on the data",
        &vbox,
    ));
    window.show_all();
}
