extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use self::gdk_pixbuf::{ Pixbuf };
use self::gdk::{ Screen };
use gtk::prelude::*;

use gtk::{
  Button, Image, Builder, Box, StyleContext,
  Revealer, Settings, CssProvider, Stack
};

pub struct Pages {
  builder: Builder,
  stack: Stack,

  reader: Box,
  about: Box,
  home: Box
}

impl Pages {
  pub fn new() -> Pages {
    let builder = Builder::new_from_resource("/org/gtk/Lurkmore/ui/pages.xml");
    let stack: Stack = Stack::new();
    
    let reader: Box = builder.get_object("page_reader").unwrap();
    let about: Box = builder.get_object("page_about").unwrap();
    let home: Box = builder.get_object("page_home").unwrap();

    Pages {
      builder: builder,
      stack: stack,

      reader: reader,
      about: about,
      home: home,
    }
  }

  pub fn prepare_stack(&self) {
    self.stack.set_transition_type(gtk::StackTransitionType::OverRight);
    self.stack.set_transition_duration(450);
    self.stack.set_hhomogeneous(true);

    self.stack.add_titled(&self.reader, "page_reader", "Reader");
    self.stack.add_titled(&self.about, "page_about", "About");
    self.stack.add_titled(&self.home, "page_home", "Home");
  }

  pub fn get_content(&self) -> &Stack {
    return &self.stack;
  }
}