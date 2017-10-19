use std::cell::RefCell;
use std::rc::Rc;

use utils::wiki::{ Article, ErrorReason };
use utils::traits::{ View, Event };
use layout::tree::ArticleTreeEvent;
use utils::navigator::EventEmitter;
use utils::add_class_to_widget;
use utils::wiki::WikiResource;
use fluent::MessageContext;
use gtk;

use gtk::ScrolledWindowExt;
use gtk::ContainerExt;
use gtk::WidgetExt;
use gtk::BoxExt;

pub struct Reader<'a> {
  events: Rc<RefCell<EventEmitter>>,
  container: gtk::Box,
  content: gtk::Box,
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

    scrolled.add(&content);
    container.pack_start(&scrolled, true, true, 0);

    Reader {
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

  fn get_article(&self, name: String, resource: WikiResource) {
    match Article::get_article_by_title(name.clone(), resource.clone()) {
      Err(_) => {
        // TODO: Adding handle view if get error
      },
      Ok(article) => {        
        self.clear_content();
        
        self.content.pack_start(&article.tree.layout, false, true, 0);
        let article_name = article.title.clone();
        
        let global_events = self.events.clone();
        article.tree.events.borrow_mut().subscribe(move |event| {
          println!("event on page: {}", &article_name);

          match event {
            ArticleTreeEvent::ExternalLink(url) => {
              println!("open external link: {}", &url);
            },

            ArticleTreeEvent::WikiLink(name) => {
              println!("open wiki link: {}", &name);
              
              global_events.borrow_mut().push(Event::GetArticle(
                String::from(name),
                resource.clone()
              ));
            },
            _ => {}
          }
        });
        
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

  fn on_receive_event(&self, event: Event) {
    match event {
      Event::GetArticle(name, resource) => self.get_article(name, resource),
      _ => {}
    }
  }

  fn setup(&mut self, i18n: Rc<RefCell<MessageContext>>) {
    
  }
}