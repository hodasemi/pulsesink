extern crate gtk;
extern crate libpulse_binding as pa;

use gtk::prelude::*;
use gtk::*;

mod constants;

mod gui;
mod pulse;
mod util;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = Builder::new_from_string(include_str!("../pulsesink.glade"));

    print_error_return!(gui::setup_gui(&builder));

    // gtk scope required, cuz of recursion
    gtk::main();
}
