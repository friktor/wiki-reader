#![feature(proc_macro, conservative_impl_trait, generators)]
#[macro_use] extern crate maplit;

extern crate inflections;
extern crate serde_json;
extern crate gdk_pixbuf;
extern crate reqwest;
extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate url;

pub mod controllers;
pub mod components;
pub mod contents;
pub mod utils;
pub mod views;
mod app;

use gtk::GtkApplicationExt;
use gio::ApplicationExt;
use std::error::Error;
use app::Application;
use gtk::WindowExt;
use gtk::WidgetExt;
use gio::Resource;

pub fn load_resource() {
  match Resource::load("./bundles/bundle.gresource") {
    Ok(resource) => { gio::resources_register(&resource) },
    Err(error) => { println!("{:?}", Error::description(&error)) }
  }
}

fn main() {
  load_resource();

  gtk::init().expect("Failed to initialize GTK.");

  let wiki_reader_app = gtk::Application::new(Some("org.gtk.wikireader"), gio::APPLICATION_FLAGS_NONE)
    .expect("Failed to create application.");

  wiki_reader_app.connect_activate(move |wiki_reader_app| {
    if let Some(window) = wiki_reader_app.get_active_window() {
      window.present();
    } else {
      let app = Application::new();
      app.setup();

      wiki_reader_app.add_window(&app.window);
      app.window.present();
      app.window.show_all();
    }
  });

  wiki_reader_app.run(&[]);
}