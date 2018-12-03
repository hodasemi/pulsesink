use pa::context::introspect::Introspector;
use util::entry::Entry;

pub fn get_all_sinks() -> Result<Vec<Entry>, String> {
    //let sinks = Introspector::get_sink_info_list();

    Ok(Vec::new())
}

pub fn load_module(sink_type: &str, name: &str, args: &[&str]) -> Result<(), String> {
    Ok(())
}
