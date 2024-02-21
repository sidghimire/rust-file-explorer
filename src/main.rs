use std::{ fs, i32 };

use gtk::{ prelude::*, Button, CssProvider, StyleContext };
use gtk::{ Label, Window, WindowType, Fixed, Box };
use pango::FontDescription;

fn put_label(x: i32, y: i32, text: &str, font_size: i32, font_family: &str, fixed: &Fixed) {
    let label = Label::new(Some(text));
    let mut font_desc = FontDescription::new();
    font_desc.set_size(font_size * pango::SCALE);
    font_desc.set_weight(pango::Weight::Ultraheavy);
    font_desc.set_family(font_family);
    label.override_font(&font_desc);
    let vbox = Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&label, true, true, 0);

    fixed.put(&vbox, x, y);
}



fn put_button<'a, F: Fn() + 'static>(fixed: &Fixed, x: i32, y: i32, text: &str, callback: F) {
    let button = Button::new();
    button.set_label(&text);
    button.set_size_request(150, 40);
    button.connect_clicked(move |_| callback()); // Use |_| to ignore the argument
    apply_css(
        &button,
        "
        button {
            background: #cfcfcf;
            color: #000;
            border: 0px;
            border-bottom: 1px solid #dfdfdf;
            padding: 10px; /* Adjust padding if needed */
            border-radius: 5px;
        }
        "
    );
    fixed.put(&button, x, y);
}



fn apply_css(widget: &Button, css: &str) {
    let screen = widget.get_screen().expect("Failed to get screen");
    let provider = CssProvider::new();
    CssProvider::load_from_data(&provider, css.as_bytes()).expect("Failed to Load CSS style");
    StyleContext::add_provider_for_screen(
        &screen,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    )
}

fn get_root_directories() -> Vec<(String, String, bool)> {
    let paths: fs::ReadDir = fs::read_dir("/").unwrap();
    let mut all_dir: Vec<(String, String, bool)> = vec![];
    for path in paths {
        let entry = path.unwrap();
        let file_name = entry.file_name().to_string_lossy().into_owned();
        let isDir = entry.metadata().unwrap().is_dir();
        let path = entry.path().to_string_lossy().into_owned();
        all_dir.push((file_name, path, isDir));
    }
    all_dir
}
fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("GTK Rust Text Example");
    window.set_default_size(950, 1050);
    let fixed = Fixed::new();
    window.add(&fixed);

    put_label(20, 20, "File Explorer", 30, "VarelaRound-Regular", &fixed);

    let all_root_dir: Vec<(String, String, bool)> = get_root_directories();
    for index2 in 0..all_root_dir.len() / 5 {
        // Slice the `all_root_dir` to get elements for the current row
        let row_elements = &all_root_dir[index2 * 5..(index2 + 1) * 5];

        // Iterate over each element in the current row
        for (index, (name, path, isDir)) in row_elements.iter().enumerate() {
            // Calculate position based on the row index (index2) and element index within the row
            let x_position = 20 + (index as i32) * 170;
            let y_position = 140 + 115 * (index2 as i32);



            put_button(&fixed, x_position, y_position, &name, || println!("Button Clicked"));
        
        
        
        
        }
    }
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    window.connect_destroy(|_| { gtk::main_quit() });

    gtk::main()
}
