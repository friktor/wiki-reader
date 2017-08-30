extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use std::cell::UnsafeCell;
use std::rc::Rc;

use navigator::{ Page, NavigatorEvent };
use gtk::{ Builder, Box, Stack };

pub struct Home<'a> {
  is_starter: bool,
  title: String,
  content: Box,
  name: &'a str
}

impl <'a>Home<'a> {
  pub fn new() -> Home<'a> {
    let builder = Builder::new_from_resource("/org/gtk/Lurkmore/c_ui/home.xml");
    let content: Box = builder.get_object("page_home").unwrap();

    Home {
      title: String::from("Home"),
      name: "page_home",
      is_starter: true,
      content
    }
  }
}

impl <'a>Page for Home<'a> {
  fn get_content(&self) -> &gtk::Box {
    &self.content
  }

  fn get_name(&self) -> &str {
    self.name
  }

  fn get_title(&self) -> &str {
    &self.title[..]
  }

  fn on_receive_event(&self, event: NavigatorEvent) {
    println!("handle event in home");
  }
}