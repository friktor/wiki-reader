extern crate serde_json;
extern crate reqwest;
extern crate url;

use self::serde_json::{ Value, from_str as json_from_str };
use std::process::{Command, Stdio};
use std::collections::HashMap;
use std::io::prelude::*;
use self::reqwest::get;
use std::error::Error;
use self::url::Url;

#[derive(Clone)]
pub enum ErrorReason {
  formatting,
  request,
  parsing,
}

#[derive(Clone)]
pub enum WikiResource {
  Wikipedia,
  Lurkmore,
  Custom
}

#[derive(Clone)]
pub struct Article {
  pub wikicode: String,
  pub content: Value,
  pub title: String,
  pub page_id: i64,
}

impl Article {
  pub fn new_from_title(title: String) -> Result<Article, ErrorReason> {
    match Article::get_article_by_title(title, WikiResource::Lurkmore) {
      Ok(mut response) => Article::normalize_response(response),
      Err(reason) => Err(reason)
    }
  }

  fn get_article_by_title(title: String, resource: WikiResource) -> Result<Value, ErrorReason> {
    let url = Article::generate_url(resource, hashmap!{
      "action" => "query",
      "titles" => &title[..],
      "prop"   => "revisions",
      "rvprop" => "content",
      "format" => "json",
    });

    let request = get(url);

    match request {
      Ok(mut response) => {
        let json = response.json();
        match json {
          Err(error) => Err(ErrorReason::parsing),
          Ok(tree) => Ok(tree)
        }
      },

      Err(error) => {
        println!("Error: {}", error.description());
        Err(ErrorReason::request)
      }
    }
  }

  fn normalize_response(response: Value) -> Result<Article, ErrorReason> {
    let pages = &response["query"]["pages"];
    
    if !pages.is_object() {
      return Err(ErrorReason::formatting)
    }

    let object = pages.as_object().unwrap();
    let negative_key = String::from("-1");

    if object.is_empty() || object.contains_key(&negative_key) {
      return Err(ErrorReason::parsing)
    }

    let mut wikicode = String::new();
    let mut title = String::new();
    let mut page_id: i64 = -1;
    
    for (key, value) in object.iter() {
      let s = value["revisions"][0]["*"].as_str().unwrap();
      let t = value["title"].as_str().unwrap();
      
      page_id = value["pageid"].as_i64().unwrap();
      wikicode.push_str(s);
      title.push_str(t);
      break;
    }

    let result = match Article::get_wikicode_ast(wikicode.clone()) {
      Err(reason) => return Err(reason),
      Ok(ast) => Article {
        content: ast,
        wikicode,
        page_id,
        title
      }
    };

    return Ok(result);
  }

  pub fn get_wikicode_ast(code: String) -> Result<Value, ErrorReason> {
    let command = format!("./wiki-parser/main.py");

    let process = match Command::new(&command[..])
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn() {
        Err(why) => panic!("couldn't spawn wiki-parser: {}", why.description()),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(code.as_bytes()) {
      Err(error) => return Err(ErrorReason::parsing),
      Ok(_) => {}
    }

    let mut dirty_ast = String::new();
    match process.stdout.unwrap().read_to_string(&mut dirty_ast) {
      Err(why) => return Err(ErrorReason::parsing),
      Ok(_) => {} 
    }

    match json_from_str(&dirty_ast[..]) {
      Err(error) => Err(ErrorReason::parsing),
      Ok(ast) => Ok(ast)
    }
  }

  fn generate_url(resource: WikiResource, params: HashMap<&str, &str>) -> Url {
    let mut params_url = String::new();
    let mut is_first = true;

    let host = match resource {
      WikiResource::Wikipedia => "https://ru.wikipedia.org/w/api.php",
      WikiResource::Lurkmore => "http://lurkmore.to/api.php",
      WikiResource::Custom => "~~~ TODO ~~~"
    };

    for (param, value) in &params {
      if is_first {
        params_url.push_str("?");
        is_first = false;
      } else {
        params_url.push_str("&");
      }

      let formatted = format!("{}={}", param, value);
      params_url.push_str(&formatted[..]);
    }

    let url = format!("{}{}", &host, &params_url);
    Url::parse(&url[..]).unwrap()
  }
}