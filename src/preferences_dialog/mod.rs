mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gio, glib};
use glib::Object;


glib::wrapper! {
    pub struct RonajoPreferencesDialog(ObjectSubclass<imp::RonajoPreferencesDialog>)
    @extends adw::PreferencesDialog, adw::Dialog, gtk::Widget, glib::InitiallyUnowned,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl RonajoPreferencesDialog {
    pub fn new() -> Self {
        Object::builder()
            .build()
    }
}

