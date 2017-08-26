#![feature(conservative_impl_trait)]
#[macro_use] extern crate maplit;

extern crate gio;
extern crate gtk;

mod navigator;
mod headerbar;
mod reader;
mod home;
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
  if gtk::init().is_err() {
    println!("Failed to initialize GTK.");
    return;
  } else {
    let app = Application::new();
    app.run();
  }
}