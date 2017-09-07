extern crate gtk;

use std::cell::{ UnsafeCell, RefCell };
use std::rc::Rc;

pub trait View {
  fn on_receive_event(&self, event: Event);
  fn get_content(&self) -> &gtk::Box;
  fn get_title(&self) -> &str;
  fn get_name(&self) -> &str;
}

pub trait Controller {
  fn on_receive_event(&self, event: Event);
  fn get_view(&self) -> &Rc<UnsafeCell<View>>;
}

#[derive(Clone)]
pub enum Event {
  GetArticle(String),
  OpenPage(String),
  ToggleSidebar,
}