#![feature(proc_macro, conservative_impl_trait, generators)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate matches;
#[macro_use] extern crate maplit;

extern crate gio;
extern crate gtk;

pub mod controllers;
pub mod components;
pub mod utils;
pub mod views;
mod app;

use std::error::{ Error };
use app::{ Application };
use gio::{ Resource };

pub fn load_resource() {
  match Resource::load("./bundles/bundle.gresource") {
    Ok(resource) => { gio::resources_register(&resource) },
    Err(error) => { println!("{:?}", Error::description(&error)) }
  }
}

fn main() {
  load_resource();

  gtk::init().expect("Failed to initialize GTK.");
  let app = Application::new();
  app.run();
}