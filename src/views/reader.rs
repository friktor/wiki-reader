use std::cell::RefCell;
use std::rc::Rc;

use utils::wiki::{ Article, ErrorReason };
use utils::traits::{ View, Event };
use utils::navigator::EventEmitter;
use utils::add_class_to_widget;
use serde_json::Value;
use gtk;

use gtk::ScrolledWindowExt;
use gtk::ContainerExt;
use gtk::WidgetExt;
use gtk::BoxExt;

pub struct Reader<'a> {
  events: Rc<RefCell<EventEmitter>>,
  container: gtk::Box,
  content: gtk::Box,
  title: String,
  name: &'a str
}

impl <'a>Reader<'a> {
  pub fn new(events: Rc<RefCell<EventEmitter>>) -> Reader<'a> {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    add_class_to_widget(&container, "reader");
    add_class_to_widget(&container, "page");

    let scrolled = gtk::ScrolledWindow::new(None, None);
    scrolled.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);

    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    // add_class_to_widget(&content, "too-big-box");

    scrolled.add(&content);
    container.pack_start(&scrolled, true, true, 0);

    Reader {
      title: String::from("Reader"),
      name: "reader",
      container,
      content,
      events
    }
  }

  fn clear_content(&self) {
    let childs = self.content.get_children();
    for child in childs {
      self.content.remove(&child);
    }
  }

  fn get_article(&self, name: String) {
    // Next need add error handle
    match Article::new_from_title(name.clone()) {
      Err(_) => {
        // TODO: Adding handle view if get error
      },
      Ok(article) => {        
        self.clear_content();
        self.content.pack_start(&article.layout, false, true, 0);
        self.content.show_all();
      }
    }
  }
}

impl <'a>View for Reader<'a> {
  fn get_content(&self) -> gtk::Box {
    self.container.clone()
  }

  fn get_name(&self) -> String {
    String::from(self.name)
  }

  fn get_title(&self) -> String {
    self.title.clone()
  }

  fn on_receive_event(&self, event: Event) {
    match event {
      Event::GetArticle(name) => self.get_article(name),
      _ => {}
    }
  }

  fn setup(&mut self) {
    
  }
}