use gtk::prelude::*;
use gtk::*;
use return_error;

pub mod callbacks;

pub fn setup_gui(builder: &Builder) -> Result<(), String> {
    return_error!(callbacks::create_callbacks(builder));

    Ok(())
}
