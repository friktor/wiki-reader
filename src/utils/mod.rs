pub mod navigator;
pub mod traits;
pub mod wiki;

use fluent::types::FluentValue;
use std::collections::HashMap;
use fluent::MessageContext;
use fluent_locale::Locale;
use std::io::prelude::*;
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;
use std::env;
use gtk;

pub fn add_class_to_widget<T: gtk::WidgetExt>(widget: &T, class: &str) {
  use gtk::StyleContextExt;

  let context = widget.get_style_context().unwrap();
  context.add_class(class);
}

pub fn get_resources_path() -> String {
  if cfg!(feature="release") {
    let mut exec_path = env::current_exe().unwrap();
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
    let mut exec_path = env::current_exe().unwrap();
    exec_path.pop();
    exec_path.pop();
    
    let target_path = exec_path.to_str().unwrap();
    format!(".{}/Resources/wiki-parser/main.py", target_path)
  } else {
    String::from("./wiki-parser/main.py")
  }
}

pub fn get_i18n_locale() -> (Locale, String) {
  let mut locale = match env::var("LANG") {
    Ok(code) => {
      let codes: Vec<&str> = code.split(".").collect();
      Locale::from(String::from(codes[0].clone()))
    },

    Err(_) => {
      Locale::from("en_US")
    }
  };

  let _lc = locale.clone();
  let language = _lc.get_language();
  if language != "ru" || language != "en" {
    locale = Locale::from("en_US");
  }
  
  let locale_path = format!("{}/i18n/{}.fluent", get_resources_path(), &language);
  // println!("language: {}, path: {}", &language, &locale_path);

  let mut locale_file = File::open(&*locale_path).expect("i18n file not found");
  
  let mut i18n_text = String::new();
  locale_file.read_to_string(&mut i18n_text)
    .expect("Error with read i18n file");

  // println!("locales:\n {}", &i18n_text);
  let _locale = locale.clone();
  (_locale, i18n_text)
}

pub fn t_(
  params: Option<&HashMap<&str, FluentValue>>,
  i18n: Rc<RefCell<MessageContext>>,
  key: &str, 
) -> String {
  let c = i18n.borrow();

  let message = c.get_message(key).unwrap();
  let result = c.format(message, params);
  let value = result.unwrap();

  // println!("i18n({}: {})", &key, &value);
  value
}