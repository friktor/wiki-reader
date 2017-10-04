pub mod navigator;
pub mod traits;
pub mod wiki;

use std::env::current_exe;
use gtk;

pub fn add_class_to_widget<T: gtk::WidgetExt>(widget: &T, class: &str) {
  use gtk::StyleContextExt;

  let context = widget.get_style_context().unwrap();
  context.add_class(class);
}

pub fn get_resources_path() -> String {
  if cfg!(feature="debug") {
    String::from("./bundles")
  } else {
    let mut exec_path = current_exe().unwrap();
    exec_path.pop();
    exec_path.pop();
    
    let target_path = exec_path.to_str().unwrap();
    format!("{}/Resources/assets", target_path)
  }
}