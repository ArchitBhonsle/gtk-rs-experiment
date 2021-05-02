extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate polars;

mod pages;
mod utils;

use gio::prelude::*;
use pages::Pages;
use std::cell::RefCell;

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

thread_local! {static PAGE: RefCell<Pages> = RefCell::new(Pages::Choose)}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .application(application)
        .title("gtk-rs-experiment")
        .default_height(800)
        .default_width(1400)
        .build();

    PAGE.with(|f| {
        let mut current_page = f.borrow_mut();
        match *current_page {
            Pages::Choose => {
                *current_page = pages::choose::processing_page(window);
            }
            Pages::Processing => {}
            Pages::Train => {}
            Pages::Test => {}
        }
    });
}
