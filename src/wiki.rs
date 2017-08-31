extern crate serde_json;
extern crate reqwest;
extern crate serde;
extern crate url;

use self::serde_json::{ Value, Error };
use std::collections::HashMap;
use self::reqwest::get;
use self::url::Url;

fn format_request(params: HashMap<&str, &str>) -> Url {
  let mut req_url = String::from("http://lurkmore.to/api.php");
  let mut is_first = true;

  for (param, value) in &params {
    if is_first {
      req_url.push_str("?");
      is_first = false;
    } else {
      req_url.push_str("&");
    }

    let formatted = format!("{}={}", param, value);
    req_url.push_str(&formatted[..]);
  }

  Url::parse(&req_url[..]).unwrap()
}

pub struct Article {
  content: Value,
  success: bool
}

pub fn get_article_by_name(name: String) -> Article {
  let params: HashMap<&str, &str> = hashmap!{
    "action" => "query",
    "titles" => &name[..],
    "props" => "revisions",
    "rvprop" => "content",
    "format" => "json",
  };

  let formatted = format_request(params);
  let mut r = reqwest::get(formatted);
  if r.is_ok() {
    let mut response = r.unwrap();

    if response.status().is_success() {
      let content: Value = response.json().unwrap();
      
      Article {
        content: content,
        success: true,
      }
    } else {
      Article { success: false, content: Value::Null }
    }
  } else {
    Article { success: false, content: Value::Null }
  }
}