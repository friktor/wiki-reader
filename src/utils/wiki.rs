extern crate serde_json;
extern crate reqwest;
extern crate url;

use self::serde_json::{ Value, from_str as json_from_str };
use std::collections::HashMap;
use self::reqwest::get;
use self::url::Url;

use std::io::prelude::*;
use std::fs::{ File, remove_file };
use std::process::Command;
use std::error::Error;
use std::path::Path;
use std::env;

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

#[derive(Clone)]
pub struct Article {
  pub content: Value,
  pub success: bool
}

fn write_json_to_cache(content: &Value, title: &str) -> String {
  use std::io::Write;

  let _cdir = env::current_dir().unwrap();
  let current_dir = _cdir.to_str().unwrap();

  let content_json = format!("{}", content);
  let file_path = format!("{}/cache/{}.json", &current_dir, &title);

  let fcp = file_path.clone();
  let path = Path::new(&fcp);
  
  let mut file = match File::create(&path) {
    Err(why) => panic!("couldn't create {}: {}", path.display(), why.description()),
    Ok(file) => file,
  };

  println!("write cache file to: {}, is exists - {}", 
    &path.display(),
    &path.exists()
  );

  match file.write_all(content_json.as_bytes()) {
    Err(why) => panic!("error in write file content: {}", &why.description()),
    Ok(()) => println!("success create file")
  };

  return file_path;
}

pub fn get_article_ast(content: Value, title: &str) -> Result<Value, ()> {
  let file_path = write_json_to_cache(&content, &title);
  
  let _cdir = env::current_dir().unwrap();
  let current_dir = _cdir.to_str().unwrap();

  let command = Command::new("./cli")
    .current_dir(format!("{}/cli-parser", &current_dir))
    .arg(&file_path)
    .output();

  let result = match command {
    Ok(out) => {
      let dirty_output = String::from_utf8_lossy(&out.stdout);
      let output = format!("{}", dirty_output);

      match json_from_str(&output[..]) {
        Ok(json) => Ok(json),
        Err(error) => {
          println!("error parsing json: {}", error.description());
          Err(())
        }
      }
    },

    Err(error) => {
      println!("error execute command: {}", error.description());
      Err(())
    }
  };

  let path = Path::new(&file_path);
  match remove_file(&path) {
    Err(why) => panic!("couldn't remove file {}: {}", &why.description(), &path.display()),
    Ok(()) => println!("file success removed")
  }

  return result;
}

pub fn get_article_by_name(name: String) -> Article {
  let title = &name[..];

  let params: HashMap<&str, &str> = hashmap!{
    "action" => "query",
    "titles" => title,
    "prop"   => "revisions",
    "rvprop" => "content",
    "format" => "json",
  };

  let formatted = format_request(params);
  let r = get(formatted);

  if r.is_ok() {
    let mut response = r.unwrap();
    let url = response.url().clone();

    if response.status().is_success() {
      println!("{}", url.into_string());
      let res_content: Value = response.json().unwrap();
      // TODO: handle error in this parsing scope
      let content = get_article_ast(res_content, title).unwrap();
      
      Article {
        content: content,
        success: true
      }
    } else {
      Article { success: false, content: Value::Null }
    }
  } else {
    Article { success: false, content: Value::Null }
  }
}