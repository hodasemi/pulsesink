extern crate gtk;

use gtk::prelude::*;
use gtk::*;

mod command;
mod constants;
mod entry;

macro_rules! return_error {
    ($v:expr) => {
        if let Err(err) = $v {
            return Err(err);
        }
    };
}

macro_rules! print_error {
    ($v:expr) => {
        if let Err(err) = $v {
            println!("{}", err);
        }
    };
}

fn convert_tree_model<T: IsA<TreeModel>>(tree_view: &TreeView) -> Option<T> {
    tree_view.get_model().unwrap().downcast().ok()
}

fn create_column(tree_view: &TreeView) {
    let column = TreeViewColumn::new();
    let renderer = CellRendererText::new();
    column.set_title("Sink");
    column.set_resizable(true);
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", 0);
    tree_view.append_column(&column);
}

fn fill_list_store(list_store: &ListStore) -> Result<(), String> {
    let entries = match entry::get_all_sinks() {
        Ok(sinks) => sinks,
        Err(err) => return Err(err),
    };

    list_store.clear();

    for entry in entries {
        list_store.insert_with_values(None, &[0], &[entry.name()]);
    }

    Ok(())
}

fn create_sink_list(tree_view: &TreeView) -> Result<(), String> {
    let sink_data: ListStore = convert_tree_model(tree_view).unwrap();

    if let Err(err) = fill_list_store(&sink_data) {
        return Err(err);
    }

    Ok(())
}

fn load_module(sink_type: &str, name: &str, args: &[&str]) -> Result<(), String> {
    let sink_name_str = format!("sink_name={}", &name);
    let mut create_sink_args = vec!["load-module", sink_type, &sink_name_str];
    create_sink_args.extend_from_slice(args);

    return_error!(command::bash("pacmd", create_sink_args.as_slice()));

    let dev_prop_str = format!("device.description={}", &name);
    let change_name_args = ["update-sink-proplist", &name, &dev_prop_str];
    return_error!(command::bash("pacmd", &change_name_args,));

    Ok(())
}

fn main() {
    if init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = Builder::new_from_string(include_str!("../pulsesink.glade"));

    let name_chooser: Dialog = builder.get_object("NameChooser").unwrap();
    let new_sink: Button = builder.get_object("NewSink").unwrap();
    let create_sink: Button = builder.get_object("CreateSink").unwrap();
    let sink_name_field: gtk::Entry = builder.get_object("SinkName").unwrap();
    let null_sink: RadioButton = builder.get_object("NullRadio").unwrap();
    let combined_sink: RadioButton = builder.get_object("SimultaneousRadio").unwrap();
    let loopback_sink: RadioButton = builder.get_object("LoopBackRadio").unwrap();
    let cancel_sink: Button = builder.get_object("CancelSink").unwrap();
    let first_slave: ComboBoxText = builder.get_object("FirstSlaveBox").unwrap();
    let second_slave: ComboBoxText = builder.get_object("SecondSlaveBox").unwrap();
    let window: Window = builder.get_object("MainWindow").unwrap();
    let tree_view: TreeView = builder.get_object("SinkList").unwrap();

    create_column(&tree_view);
    if let Err(err) = create_sink_list(&tree_view) {
        println!("{}", err);
        return;
    }

    window.show_all();

    // close event
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    {
        let name_chooser_clone = name_chooser.clone();

        name_chooser.connect_delete_event(move |_, _| {
            name_chooser_clone.hide();
            Inhibit(false)
        });
    }

    // open dialog window
    {
        let name_chooser_clone = name_chooser.clone();
        let first_slave_clone = first_slave.clone();
        let second_slave_clone = second_slave.clone();
        let combined_sink_clone = combined_sink.clone();

        new_sink.connect_clicked(move |_| {
            let entries = match entry::get_all_sinks() {
                Ok(sinks) => sinks,
                Err(err) => {
                    println!("{}", err);
                    Vec::new()
                }
            };

            first_slave_clone.remove_all();
            second_slave_clone.remove_all();

            for entry in &entries {
                first_slave_clone.append_text(entry.name());
                second_slave_clone.append_text(entry.name());
            }

            if !entries.is_empty() {
                first_slave_clone.set_active(0);
                second_slave_clone.set_active(0);
            }

            first_slave_clone.set_sensitive(combined_sink_clone.get_active());
            second_slave_clone.set_sensitive(combined_sink_clone.get_active());

            name_chooser_clone.run();
        });
    }

    // toggle slaves
    {
        let first_slave_clone = first_slave.clone();
        let second_slave_clone = second_slave.clone();
        let combined_sink_clone = combined_sink.clone();

        combined_sink.connect_clicked(move |_| {
            first_slave_clone.set_sensitive(combined_sink_clone.get_active());
            second_slave_clone.set_sensitive(combined_sink_clone.get_active());
        });
    }

    // cancel dialog
    {
        let name_chooser_clone = name_chooser.clone();

        cancel_sink.connect_clicked(move |_| {
            name_chooser_clone.hide();
        });
    }

    // create new sink
    {
        let name_chooser_clone = name_chooser.clone();
        let first_slave_clone = first_slave.clone();
        let second_slave_clone = second_slave.clone();

        create_sink.connect_clicked(move |_| {
            let sink_name: String = sink_name_field.get_buffer().get_text();

            if sink_name.is_empty() {
                name_chooser_clone.hide();
                return;
            }

            if null_sink.get_active() {
                print_error!(load_module("module-null-sink", &sink_name, &[]));
            } else if combined_sink.get_active() {
                let first_slave_name = match first_slave_clone.get_active_text() {
                    Some(name) => name,
                    None => {
                        println!("couldn't get name for the first slave");
                        return;
                    }
                };

                let second_slave_name = match second_slave_clone.get_active_text() {
                    Some(name) => name,
                    None => {
                        println!("couldn't get name for the second slave");
                        return;
                    }
                };

                let entries = match entry::get_all_sinks() {
                    Ok(sinks) => sinks,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };

                let first_slave_id =
                    match entry::Entry::find_id_by_name(&entries, &first_slave_name) {
                        Some(id) => id,
                        None => {
                            println!("no id found for name: {}", &first_slave_name);
                            return;
                        }
                    };

                let second_slave_id =
                    match entry::Entry::find_id_by_name(&entries, &second_slave_name) {
                        Some(id) => id,
                        None => {
                            println!("no id found for name: {}", &second_slave_name);
                            return;
                        }
                    };

                print_error!(load_module(
                    "module-combine-sink",
                    &sink_name,
                    &[&format!("slaves={},{}", first_slave_id, second_slave_id)]
                ));
            } else if loopback_sink.get_active() {
                print_error!(load_module("module-loopback", &sink_name, &[]));
            } else {
                println!("radio button error");
                return;
            };

            print_error!(create_sink_list(&tree_view));

            name_chooser_clone.hide();
        });
    }

    // gtk scope required, cuz of recursion
    gtk::main();
}
