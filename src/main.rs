extern crate gio;
extern crate gtk;
mod pages;
mod app;

use std::error::{ Error };
use app::{ Application };
use gio::{ Resource };

pub fn load_resource() {
  match Resource::load("./src/resources/resource.gresource") {
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
    app.prepare_and_run();
  }
}