extern crate gtk;

use std::cell::UnsafeCell;
use std::rc::Rc;

use utils::navigator::{ Navigator, EventEmitter };
use utils::traits::{ View, Event };

pub struct Home<'a> {
  events: Rc<UnsafeCell<EventEmitter>>,
  content: gtk::Box,
  title: String,
  name: &'a str
}

impl <'a>Home<'a> {
  pub fn new(events: &Rc<UnsafeCell<EventEmitter>>) -> Home<'a> {
    let builder = gtk::Builder::new_from_resource("/org/gtk/wikireader/c_ui/home.xml");
    let content: gtk::Box = builder.get_object("page_home").unwrap();

    Home {
      title: String::from("Home"),
      events: events.clone(),
      name: "home",
      content
    }
  }
}

impl <'a>View for Home<'a> {
  fn get_content(&self) -> &gtk::Box {
    &self.content
  }

  fn get_name(&self) -> &str {
    self.name
  }

  fn get_title(&self) -> &str {
    &self.title[..]
  }

  fn on_receive_event(&self, event: Event) {

  }
}