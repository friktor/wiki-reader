extern crate serde_json;
extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use std::cell::UnsafeCell;
use std::rc::Rc;

use navigator::{ Page, NavigatorEvent, NavigatorStateMachine };
use gtk::{ Builder, Box, Label, Stack, Revealer };
use wiki::{ get_article_by_name, Article };
use self::serde_json::Value;
use gtk::BoxExt;

pub struct Reader<'a> {
  events: Rc<UnsafeCell<NavigatorStateMachine>>,
  is_starter: bool,
  title: String,
  content: Box,
  name: &'a str
}

impl <'a>Reader<'a> {
  pub fn new(events: &Rc<UnsafeCell<NavigatorStateMachine>>) -> Reader<'a> {
    use gtk::StyleContextExt;
    use gtk::WidgetExt;

    let content = Box::new(gtk::Orientation::Vertical, 0);
    let context = content.get_style_context().unwrap();
    context.add_class("content-page");

    let _events = events.clone();

    Reader {
      title: String::from("Reader"),
      name: "page_reader",
      is_starter: true,
      events: _events,
      content,
    }
  }
}

impl <'a>Reader<'a> {
  fn clear_content(&self) {
    use gtk::ContainerExt;

    let childs = self.content.get_children();
    for child in childs {
      self.content.remove(&child);
    }
  }

  fn get_article(&self, name: String) {
    use gtk::WidgetExt;

    let article = get_article_by_name(name.clone());
    let nodes = self.get_nodes(article.clone(), name.clone());
    println!("search for: {} \n {}", &name, &article.content);
    
    self.clear_content();
    self.content.pack_start(&nodes, false, true, 0);
    self.content.show_all();
    
    let events = self.events.get();
    let page = NavigatorEvent::OpenPage(String::from("page_reader"));
    unsafe { (*events).push_event(page) }
  }

  fn disambiguation_content(&self, container: &Box, content: &Value) {
    
  }

  fn page_content(&self, container: &Box, content: &Value) {

  }

  fn get_nodes(&self, article: Article, title: String) -> Box {
    let container = Box::new(gtk::Orientation::Vertical, 0);
    let res_type = article.content["type"].as_str().unwrap();

    let article_title = Label::new(&title[..]);
    container.pack_start(&article_title, false, true, 0);

    match res_type {
      "disambiguation" => self.disambiguation_content(&container, &article.content),
      "page" => self.page_content(&container, &article.content),
      _ => {}
    }

    container
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
      _ => {}
    }
  }
}