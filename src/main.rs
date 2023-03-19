mod window;

use gtk::prelude::*;
use gtk::{gio, Application};
use window::Window;


// comment for version 1.0

const APP_ID: &str = "org.gtk_rs.single-button";

fn main() {
    // Register and include resources
    gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}


