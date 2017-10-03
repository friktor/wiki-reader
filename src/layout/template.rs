use utils::add_class_to_widget;
use serde_json::{Value, Map};
use layout::tree::Tree;
use gtk;

use gtk::BoxExt;

pub struct Template {
  pub params: Map<String, Value>,
  pub layout: gtk::Box,
  pub content: Vec<Tree>,
  pub name: String,
}

impl Template {
  pub fn setup(&self) {
    add_class_to_widget(&self.layout, "template");
    add_class_to_widget(&self.layout, &*self.name);

    match &*self.name {
      "Q" => self.quote_setup(),
      _ => {}
    }
  }

  fn quote_setup(&self) {
    for node in &self.content {
      self.layout.pack_end(&node.layout, false, false, 0);
    }
  }
}