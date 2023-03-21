mod imp;

use gio::{PropertyAction, SimpleAction};
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, Orientation};
glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    fn setup_actions(&self) {

        // let option1: SimpleAction = SimpleAction::new("option1", None);
        // option1.connect_activate(|action, _ | { println!("option 1 activated")});
        // self.add_action(&option1);

        // let option2: SimpleAction = SimpleAction::new("option2", None);
        // option2.connect_activate(|action, _ | { println!("option 2 activated")});
        // self.add_action(&option2);
    }

}
