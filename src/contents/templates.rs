extern crate serde_json;
extern crate gtk;

use self::serde_json::Value;
use std::cell::UnsafeCell;
use std::boxed::Box;
use std::rc::Rc;

use gtk::StyleContextExt;
use gtk::ContainerExt;
use gtk::WidgetExt;
use gtk::BoxExt;

fn quote_template(template: Value) -> gtk::Box {
  let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
  
  let context = content.get_style_context().unwrap();
  context.add_class("quote-template");


  return content;
}

fn unhandled_template(template: Value) -> gtk::Box {
  let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
  
  let context = content.get_style_context().unwrap();
  context.add_class("quote-template");


  return content;
}

pub fn render_template(template: Value) -> gtk::Box {
  let name = template["name"].as_str().unwrap();

  match name {
    "Q" => quote_template(template.clone()),
    _ => unhandled_template(template.clone())
  }
}