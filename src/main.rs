extern crate gtk;

use gtk::prelude::*;
use gtk::*;

mod command;
mod constants;
mod entry;

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

fn main() {
    if init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = Builder::new_from_file("pulsesink.glade");

    let window: Window = builder.get_object("MainWindow").unwrap();
    let tree_view: TreeView = builder.get_object("SinkList").unwrap();

    create_column(&tree_view);
    if let Err(err) = create_sink_list(&tree_view) {
        println!("{}", err);
        return;
    }

    window.show_all();

    // open dialog window
    let name_chooser: Dialog = builder.get_object("NameChooser").unwrap();
    let new_sink: Button = builder.get_object("NewSink").unwrap();
    let drop_down: ComboBox = builder.get_object("CombinedDropDown").unwrap();

    {
        let name_chooser_clone = name_chooser.clone();

        new_sink.connect_clicked(move |_| {
            let data = ListStore::new(&[Type::String]);

            if let Err(err) = fill_list_store(&data) {
                println!("{}", err);
            }

            drop_down.set_model(Some(&data));

            name_chooser_clone.run();
        });
    }

    // cancel dialog
    let cancel_sink: Button = builder.get_object("CancelSink").unwrap();

    {
        let name_chooser_clone = name_chooser.clone();

        cancel_sink.connect_clicked(move |_| {
            name_chooser_clone.hide();
        });
    }

    // create new sink
    let create_sink: Button = builder.get_object("CreateSink").unwrap();
    let sink_name_field: gtk::Entry = builder.get_object("SinkName").unwrap();
    let combined_box: CheckButton = builder.get_object("CombinedCheckBox").unwrap();

    {
        let name_chooser_clone = name_chooser.clone();

        create_sink.connect_clicked(move |_| {
            let sink_name: String = sink_name_field.get_buffer().get_text();

            if sink_name.is_empty() {
                name_chooser_clone.hide();
                return;
            }

            if let Err(err) = command::bash(
                "pacmd",
                &[
                    "load-module",
                    "module-null-sink",
                    &format!("sink_name={}", &sink_name),
                ],
            ) {
                println!("{}", err);
            }

            if let Err(err) = command::bash(
                "pacmd",
                &[
                    "update-sink-proplist",
                    &sink_name,
                    &format!("device.description={}", &sink_name),
                ],
            ) {
                println!("{}", err);
            }

            if let Err(err) = create_sink_list(&tree_view) {
                println!("{}", err);
            }

            name_chooser_clone.hide();
        });
    }

    // gtk scope required, cuz of recursion
    gtk::main();
}
