pub mod navigator;
pub mod traits;
pub mod wiki;
use gtk;

pub fn add_class_to_widget<T: gtk::WidgetExt>(widget: &T, class: &str) {
  use gtk::StyleContextExt;

  let context = widget.get_style_context().unwrap();
  context.add_class(class);
}