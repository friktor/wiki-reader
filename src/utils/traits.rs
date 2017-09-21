use std::cell::RefCell;
use std::rc::Rc;
use gtk;

pub trait View {
  fn on_receive_event(&self, event: Event);
  fn get_content(&self) -> gtk::Box;
  fn get_title(&self) -> String;
  fn get_name(&self) -> String;
  fn setup(&mut self);
}

pub trait Controller {
  fn on_receive_event(&self, event: Event);
  fn get_view(&self) -> Rc<RefCell<View>>;
}

#[derive(Clone)]
pub enum Event {
  GetArticle(String),
  OpenPage(String, String), // Page name, title
  ToggleSidebar,
}