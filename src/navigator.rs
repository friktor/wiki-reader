extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use std::cell::{ UnsafeCell, RefCell };
use std::ops::FnMut;
use std::rc::Rc;

use reader::{ Reader };
use home::{ Home };

use gtk::{ Builder, Stack };
use gtk::StackExt;

pub trait Page {
  fn get_content(&self) -> &gtk::Box;
  fn get_title(&self) -> &str;
  fn get_name(&self) -> &str;
}

pub struct Navigator {
  listeners: Vec<Rc<RefCell<FnMut(NavigatorEvent)>>>,
  pages: Vec<Box<Page>>,
  pub stack: Stack,
}

#[derive(Clone)]
pub enum NavigatorEvent {
  ToggleSidebar
}

impl Navigator {
  pub fn new() -> Navigator {
    // Stack with options
    let stack: Stack = Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::OverRight);
    stack.set_transition_duration(200);
    stack.set_homogeneous(true);

    // Page blocks
    let reader = Reader::new();
    let home = Home::new();

    let pages: Vec<Box<Page>> = vec![
      Box::new(reader),
      Box::new(home),
    ];

    Navigator {
      listeners: Vec::new(),
      pages,
      stack
    }
  }

  pub fn register_listener<F: FnMut(NavigatorEvent)+'static>(&mut self, listener: F) {
    let cell = Rc::new(RefCell::new(listener));
    self.listeners.push(cell); 
  }

  pub fn push_event(&mut self, event: NavigatorEvent) {
    for listener in self.listeners.iter() {
      let mut closure = listener.borrow_mut();
      let _e = event.clone();
      (&mut *closure)(_e);
    }
  }

  pub fn setup(&self) {
    for page in &self.pages {
      let content = page.get_content();
      let title = page.get_title();
      let name = page.get_name();

      self.stack.add_titled(content, name, title);
    }

    self.stack.set_visible_child_name("page_home");
  }

  pub fn open(&self, page: String) {
    self.stack.set_visible_child_name(&page[..]);
  }
}