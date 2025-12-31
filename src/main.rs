use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::new(Some("com.example.lumina"), Default::default());

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Lumina"));
        window.set_default_size(800, 600);
        window.show();
    });
    app.run();
}
