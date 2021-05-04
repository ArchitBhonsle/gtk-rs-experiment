pub mod choose;
pub mod processing;
pub mod test;
pub mod train;

use crate::utils;
use glib::IsA;
use gtk::Container;
use polars::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {static PAGE: Rc<RefCell<Pages>> = Rc::new(RefCell::new(Pages::Choose))}
thread_local! {static DF: Rc<RefCell<Option<DataFrame>>> = Rc::new(RefCell::new(None))}

#[derive(Debug)]
pub enum Pages {
    Choose,
    Processing,
    // Train,
    // Test,
}

pub fn paint(window: &gtk::ApplicationWindow) {
    utils::kill_children(window);

    PAGE.with(|p| {
        DF.with(|d| {
            match *p.borrow() {
                Pages::Choose => {
                    choose::choose_page(&window, Rc::clone(p), Rc::clone(d));
                }
                Pages::Processing => {
                    processing::processing_page(&window, Rc::clone(p), Rc::clone(d));
                } // Pages::Train => {}
                  // Pages::Test => {}
            }
        })
    });
}
