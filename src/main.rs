extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate polars;

mod pages;
mod utils;

use gio::prelude::*;
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
    let window = gtk::ApplicationWindowBuilder::new()
        .application(application)
        .title("gtk-rs-experiment")
        .default_height(800)
        .default_width(1400)
        .build();

    pages::paint(&window);
}
