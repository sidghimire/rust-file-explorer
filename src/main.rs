use gtk::prelude::*;
use gtk::{ Label, Window, WindowType, Box, ListBox, ListBoxRow, Image };
use std::fs;
use std::path::PathBuf;
use dirs;

fn main() {
    let mut currentFiles: Vec<(String, PathBuf, String)> = Vec::new();
    if let Some(home_dir) = dirs::home_dir() {
        let directory_path = home_dir;
        if let Ok(entries) = fs::read_dir(directory_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let file_name = entry.file_name().to_string_lossy().into_owned();
                    let mut char_iter = file_name.chars();
                    if let Some(first_char) = char_iter.next() {
                        if first_char != '.' {
                            if path.is_dir() {
                                currentFiles.push((
                                    file_name.clone(),
                                    path.clone(),
                                    String::from("path"),
                                ));
                            }
                            if path.is_file() {
                                currentFiles.push((
                                    file_name.clone(),
                                    path.clone(),
                                    String::from("file"),
                                ));
                            }
                        }
                    }
                }
            }
        }

        gtk::init().expect("Failed to initialize GTK");

        let data = vec!["Item 1", "Item 2", "Item 3", "Item 4"];

        let (_, list_box) = create_ui(&currentFiles);

        gtk::main();
    }
}

fn create_ui(data: &Vec<(String, PathBuf, String)>) -> (Box, ListBox) {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Rust File Explorer");
    window.set_default_size(600, 600);

    let label1 = Label::new(None);
    label1.set_markup("<span size='large'>Rust File Explorer</span>");

    let hbox = Box::new(gtk::Orientation::Horizontal, 0);
    hbox.pack_start(&label1, false, false, 10);

    let vbox = Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&hbox, false, false, 10);

    let list_box = ListBox::new();
    for item_text in data {
        if item_text.2 == String::from("path") {
            // Create a Box to hold the Image and Label
            let row_box = Box::new(gtk::Orientation::Horizontal, 0);

            // Load the icon (replace "./assets/file.png" with the actual path to your icon)
            let icon = Image::from_file("./assets/folder.png");

            // Create the label
            let label = Label::new(Some(&item_text.0));

            // Pack the icon and label into the Box
            row_box.pack_start(&icon, false, false, 5);
            row_box.pack_start(&label, false, false, 5);

            // Create a ListBoxRow and add the Box to it
            let row = ListBoxRow::new();
            row.add(&row_box);
            list_box.add(&row);
        }
    }
    vbox.pack_start(&list_box, true, true, 10);

    window.add(&vbox);

    window.connect_destroy(|_| {
        gtk::main_quit();
    });

    window.show_all();

    (vbox, list_box)
}
