use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box};
use std::process::Command;

fn main() {
    let app = Application::new(Some("com.example.ow_2_multi"), Default::default());
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("OW 2 Multi");
        window.set_default_size(300, 200);

        let vbox = Box::new(gtk::Orientation::Vertical, 5);
        let label = Label::new(Some("Welcome to OW 2 Multi"));
        let start_button = Button::with_label("Start Overwatch 2");

        start_button.connect_clicked(|_| {
            Command::new("C:\\Path\\To\\Overwatch2.exe").spawn().expect("Failed to start Overwatch 2");
        });

        vbox.pack_start(&label, true, true, 0);
        vbox.pack_start(&start_button, true, true, 0);
        window.add(&vbox);
        window.show_all();
    });

    app.run();
}