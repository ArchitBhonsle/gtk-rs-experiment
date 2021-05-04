pub mod choose;
pub mod model;
pub mod processing;

use crate::utils;
use polars::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {static PAGE: Rc<RefCell<Pages>> = Rc::new(RefCell::new(Pages::Choose))}
thread_local! {static DF: Rc<RefCell<Option<DataFrame>>> = Rc::new(RefCell::new(None))}

#[derive(Debug)]
pub enum Pages {
    Choose,
    Processing,
    Model,
}

pub fn paint(window: &gtk::ApplicationWindow) {
    utils::kill_children(window);

    PAGE.with(|p| {
        DF.with(|d| match *p.borrow() {
            Pages::Choose => {
                choose::render_page(&window, Rc::clone(p), Rc::clone(d));
            }
            Pages::Processing => {
                processing::render_page(&window, Rc::clone(p), Rc::clone(d));
            }
            Pages::Model => {
                model::render_page(&window, Rc::clone(d));
            }
        })
    });
}
