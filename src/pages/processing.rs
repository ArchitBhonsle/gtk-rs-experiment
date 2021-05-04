use super::{paint, Pages};
use crate::utils;
use gtk::prelude::*;
use polars::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub fn render_page(
    window: &gtk::ApplicationWindow,
    page_cell: Rc<RefCell<Pages>>,
    df_cell: Rc<RefCell<Option<DataFrame>>>,
) {
    let vbox = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .margin(10)
        .spacing(10)
        .build();

    let normalize_button = gtk::ButtonBuilder::new().label("Normalize").build();
    vbox.pack_start(&normalize_button, false, false, 0);

    let scroll_window = gtk::ScrolledWindowBuilder::new()
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .build();
    vbox.pack_start(&scroll_window, true, true, 0);

    let df_cell_cloned = Rc::clone(&df_cell);
    let tree_view = utils::create_tree_view(df_cell_cloned.borrow().as_ref().unwrap());
    tree_view.show();
    scroll_window.add(&tree_view);

    let scroll_window_clone = scroll_window.clone();
    let df_cell_cloned = Rc::clone(&df_cell);
    normalize_button.connect_clicked(move |_| {
        utils::kill_children(&scroll_window_clone);

        let normalized_dataframe = normalize_dataframe(df_cell_cloned.borrow().as_ref().unwrap());
        df_cell.replace(Some(normalized_dataframe.clone()));

        let tree_view = utils::create_tree_view(&normalized_dataframe);
        tree_view.show();
        scroll_window.add(&tree_view);
    });

    let next_page_button = gtk::ButtonBuilder::new().label("Train").build();
    let window_clone = window.clone();
    next_page_button.connect_clicked(move |_| {
        *page_cell.borrow_mut() = Pages::Model;
        paint(&window_clone);
    });
    vbox.pack_start(&next_page_button, false, false, 0);

    window.add(&utils::wrap_in_header(
        "Processing",
        "Apply transformations on the data",
        &vbox,
    ));
    window.show_all();
}

fn normalize_dataframe(df: &DataFrame) -> DataFrame {
    DataFrame::new(
        df.get_columns()
            .iter()
            .map(|series| {
                let (min, max): (f64, f64) = (series.min().unwrap(), series.max().unwrap());

                Series::new(
                    series.name(),
                    series
                        .cast_with_datatype(&datatypes::DataType::Float64)
                        .unwrap()
                        .f64()
                        .unwrap()
                        .into_iter()
                        .map(|x| (x.unwrap() - min) / (max - min))
                        .collect::<Vec<f64>>(),
                )
            })
            .collect(),
    )
    .unwrap()
}
