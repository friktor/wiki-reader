extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use self::gdk_pixbuf::{ Pixbuf };
use self::gdk::{ Screen };
use gtk::prelude::*;

use gtk::{
  Button, Image, Builder, Box, StyleContext,
  Revealer, Settings, CssProvider,
};

pub struct Pages {
  builder: Builder,
  reader: Box,
  about: Box,
  home: Box
}

impl Pages {
  pub fn new() -> Pages {
    let interface = include_str!("./interface.xml");
    let builder = Builder::new_from_string(interface);
    
    let reader: Box = builder.get_object("page_reader").unwrap();
    let about: Box = builder.get_object("page_about").unwrap();
    let home: Box = builder.get_object("page_home").unwrap();

    Pages {
      builder: builder,
      reader: reader,
      about: about,
      home: home,
    }
  }
}