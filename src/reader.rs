extern crate serde_json;
extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use std::cell::UnsafeCell;
use std::rc::Rc;

use wiki::{ get_article_by_name, Article };
use navigator::{ Page, NavigatorEvent };
use self::serde_json::{ Value };
use gtk::{ Builder, Box, Stack };

pub struct Reader<'a> {
  is_starter: bool,
  title: String,
  content: Box,
  name: &'a str
}

impl <'a>Reader<'a> {
  pub fn new() -> Reader<'a> {
    let content = Box::new(gtk::Orientation::Vertical, 0);

    Reader {
      title: String::from("Reader"),
      name: "page_reader",
      is_starter: true,
      content
    }
  }
}

impl <'a>Reader<'a> {
  fn get_article(&self, name: String) {
    let result = get_article_by_name(name.clone());
    
    println!("search for: {}", &name);
    println!("content: {}", &result.content);
  }
}

impl <'a>Page for Reader<'a> {
  fn get_content(&self) -> &gtk::Box {
    &self.content
  }

  fn get_name(&self) -> &str {
    self.name
  }

  fn get_title(&self) -> &str {
    &self.title[..]
  }

  fn on_receive_event(&self, event: NavigatorEvent) {
    match event {
      NavigatorEvent::GetArticle(name) => self.get_article(name),
      NavigatorEvent::ToggleSidebar => {},
    }
  }
}