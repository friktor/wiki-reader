#![feature(proc_macro, conservative_impl_trait, generators, rustc_private)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate matches;
#[macro_use] extern crate maplit;
#[macro_use] extern crate log;

extern crate env_logger;
extern crate gio;
extern crate gtk;

mod navigator;
mod headerbar;
mod reader;
mod home;
mod wiki;
mod app;

use std::error::{ Error };
use app::{ Application };
use gio::{ Resource };
use std::env;

pub fn load_resource() {
  match Resource::load("./bundles/bundle.gresource") {
    Ok(resource) => { gio::resources_register(&resource) },
    Err(error) => { println!("{:?}", Error::description(&error)) }
  }
}

fn main() {
  env_logger::init().unwrap();
  load_resource();

  println!("{}", env::current_dir().unwrap().display());

  if gtk::init().is_err() {
    println!("Failed to initialize GTK.");
    return;
  } else {
    let app = Application::new();
    app.run();
  }
}