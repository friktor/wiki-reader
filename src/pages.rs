extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use std::cell::UnsafeCell;
use std::rc::Rc;

use gtk::{ Builder, Box, Stack };
use reader::{ Reader };

pub struct Pages {
  pub reader: Rc<UnsafeCell<Reader>>,
  builder: Builder,
  pub stack: Stack,

  page_about: Box,
  page_home: Box
}

fn get_prepared_reader(builder: &Builder) -> Rc<UnsafeCell<Reader>> {
  let page_reader: Box = builder.get_object("page_reader").unwrap();
  let base = Reader::new();
  base.prepare_reader(page_reader);

  let reader = Rc::new(UnsafeCell::new(base));
  return reader;
}

impl Pages {
  pub fn new() -> Pages {
    let builder = Builder::new_from_resource("/org/gtk/Lurkmore/c_ui/pages.xml");
    let reader = get_prepared_reader(&builder);
    let stack: Stack = Stack::new();
    
    let page_about: Box = builder.get_object("page_about").unwrap();
    let page_home: Box = builder.get_object("page_home").unwrap();

    Pages {
      builder: builder,
      reader: reader,
      stack: stack,

      page_about: page_about,
      page_home: page_home,
    }
  }

  pub fn prepare_stack(&self) {
    self.stack.set_transition_type(gtk::StackTransitionType::OverRight);
    self.stack.set_transition_duration(200);
    self.stack.set_homogeneous(true);

    unsafe {
      let reader = self.reader.get();
      let content_reader = &(*reader).content;
      self.stack.add_titled(content_reader, "page_reader", "Reader");
    }

    self.stack.add_titled(&self.page_about, "page_about", "About");
    self.stack.add_titled(&self.page_home, "page_home", "Home");

    self.stack.set_visible_child_name("page_home");
  }
}