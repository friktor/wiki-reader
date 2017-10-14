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
  if cfg!(feature="release") {
    let mut exec_path = current_exe().unwrap();
    exec_path.pop();
    exec_path.pop();
    
    let target_path = exec_path.to_str().unwrap();
    format!("{}/Resources/assets", target_path)
  } else {
    String::from("./assets")
  }
}

pub fn get_parser_path() -> String {
  if cfg!(feature="release") {
    let mut exec_path = current_exe().unwrap();
    exec_path.pop();
    exec_path.pop();
    
    let target_path = exec_path.to_str().unwrap();
    format!(".{}/Resources/wiki-parser/main.py", target_path)
  } else {
    String::from("./wiki-parser/main.py")
  }
}