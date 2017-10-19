use utils::wiki::WikiResource;
use fluent::MessageContext;
use std::cell::RefCell;
use std::rc::Rc;
use gtk;

pub trait View {
  fn setup(&mut self, i18n: Rc<RefCell<MessageContext>>);
  fn on_receive_event(&self, event: Event);
  fn get_content(&self) -> gtk::Box;
  fn get_name(&self) -> String;
}

pub trait Controller {
  fn on_receive_event(&self, event: Event);
  fn get_view(&self) -> Rc<RefCell<View>>;
}

#[derive(Clone)]
pub enum Event {
  GetArticle(String, WikiResource),
  OpenPage(String, String), // Page name, title
  ToggleSidebar,
}