extern crate serde_json;
extern crate gtk;

use std::cell::UnsafeCell;
use std::rc::Rc;

use utils::navigator::{ Navigator, EventEmitter };
use utils::wiki::{ Article, ErrorReason };
use utils::traits::{ View, Event };
use self::serde_json::Value;

pub struct Reader<'a> {
  events: Rc<UnsafeCell<EventEmitter>>,
  container: gtk::Box,
  content: gtk::Box,
  title: String,
  name: &'a str
}

impl <'a>Reader<'a> {
  pub fn new(events: &Rc<UnsafeCell<EventEmitter>>) -> Reader<'a> {
    use gtk::ScrolledWindowExt;
    use gtk::StyleContextExt;
    use gtk::ContainerExt;
    use gtk::WidgetExt;
    use gtk::BoxExt;

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let context = container.get_style_context().unwrap();
    context.add_class("reader-page");

    let scrolled = gtk::ScrolledWindow::new(None, None);
    scrolled.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);

    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    // let content_context = content.get_style_context().unwrap();
    // content_context.add_class("too-big-box");

    scrolled.add(&content);

    container.pack_start(&scrolled, true, true, 0);

    Reader {
      title: String::from("Reader"),
      events: events.clone(),
      name: "reader",
      container,
      content
    }
  }

  fn clear_content(&self) {
    use gtk::ContainerExt;

    let childs = self.content.get_children();
    for child in childs {
      self.content.remove(&child);
    }
  }

  fn get_article(&self, name: String) {
    use gtk::WidgetExt;
    use gtk::BoxExt;

    // Next need add error handle
    match Article::new_from_title(name.clone()) {
      Err(_) => {},
      Ok(article) => {
        let nodes = self.get_nodes(article, name.clone());
        
        self.clear_content();
        self.content.pack_start(&nodes, false, true, 0);
        self.content.show_all();
      }
    }
  }

  fn page_content(&self, container: &gtk::Box, content: &Value) {
    
  }

  fn get_nodes(&self, article: Article, title: String) -> gtk::Box {
    use gtk::StyleContextExt;
    use gtk::WidgetExt;
    use gtk::BoxExt;

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    println!("{}", &article.content);

    let article_title = gtk::Label::new(&title[..]);
    let title_context = article_title.get_style_context().unwrap();
    title_context.add_class("page-title");
    
    container.pack_start(&article_title, false, true, 0);
    self.page_content(&container, &article.content);
    container
  }
}

impl <'a>View for Reader<'a> {
  fn get_content(&self) -> &gtk::Box {
    &self.container
  }

  fn get_name(&self) -> &str {
    self.name
  }

  fn get_title(&self) -> &str {
    &self.title[..]
  }

  fn on_receive_event(&self, event: Event) {
    match event {
      Event::GetArticle(name) => self.get_article(name),
      _ => {}
    }
  }
}