extern crate gtk;

use gtk::StyleContextExt;
use gtk::ContainerExt;
use gtk::WidgetExt;
use gtk::BoxExt;

use contents::templates::render_template;

pub fn get_node() -> gtk::Box {
  let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
  
  let context = content.get_style_context().unwrap();
  context.add_class("paragraph");

  content
}