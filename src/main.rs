#![feature(proc_macro, conservative_impl_trait, generators, underscore_lifetimes)]
#[macro_use] extern crate maplit;

extern crate fluent_locale;
extern crate inflections;
extern crate serde_json;
extern crate gdk_pixbuf;
extern crate tokio_core;
extern crate tokio_io;
extern crate futures;
extern crate reqwest;
extern crate fluent;
extern crate hyper;
extern crate pango;
extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate url;

pub mod controllers;
pub mod components;
pub mod layout;
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

use utils::get_resources_path;

pub fn load_resource() {
  let assets_path = format!("{}/assets.gresource", get_resources_path());

  match Resource::load(&*assets_path) {
    Ok(resource) => { gio::resources_register(&resource) },
    Err(error) => { println!("{:?}", Error::description(&error)) }
  }
}

fn main() {
  load_resource();

  gtk::init().expect("Failed to initialize GTK.");

  let wiki_reader_app = gtk::Application::new(
    Some("org.wikireader"),
    gio::APPLICATION_FLAGS_NONE
  ).expect("Failed to create application.");

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