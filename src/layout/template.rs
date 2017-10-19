use utils::add_class_to_widget;
use serde_json::{Value, Map};
use layout::tree::Tree;
use gtk;

use gtk::WidgetExt;
use gtk::BoxExt;

pub struct Template {
  pub params: Map<String, Value>,
  pub content: Vec<Tree>,
  pub layout: gtk::Box,
  pub name: String,
}

impl Template {
  pub fn setup(&self) {
    add_class_to_widget(&self.layout, "template");

    match &*self.name {
      "Q" | "Цитата" => self.quote_setup(),
      _ => {}
    }
  }

  pub fn is_inline(&self) -> bool {
    match &*self.name {
      "Q" | "Цитата" => false,
      _ => true
    }
  }

  fn quote_setup(&self) {
    add_class_to_widget(&self.layout, "Quote");
    self.layout.set_halign(gtk::Align::End);

    for (index, node) in self.content.iter().enumerate() {
      if index == 1 { add_class_to_widget(&node.layout, "separated") }
      self.layout.pack_start(&node.layout, true, false, 0);
    }
  }
}