use utils::add_class_to_widget;
use serde_json::{Value, Map};
use layout::tree::Tree;
use gtk;

use gtk::WidgetExt;
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
      "Q" | "Цитата" => self.quote_setup(),
      _ => {}
    }
  }

  fn quote_setup(&self) {
    self.layout.set_halign(gtk::Align::End);

    for (index, node) in self.content.iter().enumerate() {
      if index == 1 { add_class_to_widget(&node.layout, "separated") }
      self.layout.pack_start(&node.layout, true, false, 0);
    }
  }
}