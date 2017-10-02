use utils::navigator::EventEmitter;
use utils::traits::Event;
use gdk_pixbuf::Pixbuf;
use std::cell::RefCell;
use std::rc::Rc;
use gtk;

use gtk::HeaderBarExt;
use gtk::ButtonExt;
use gtk::ImageExt;
use gtk::EntryExt;

pub struct AppHeaderBar {
  events: Rc<RefCell<EventEmitter>>,
  pub headerbar: gtk::HeaderBar,
  builder: gtk::Builder,
  search: gtk::Entry
}

impl AppHeaderBar {
  pub fn new(events: Rc<RefCell<EventEmitter>>) -> AppHeaderBar {
    let builder = gtk::Builder::new_from_resource("/org/gtk/wikireader/ui/headerbar.xml");
    let headerbar: gtk::HeaderBar = builder.get_object("app_headerbar").unwrap();
    let search: gtk::Entry = builder.get_object("search_input").unwrap();

    AppHeaderBar {
      headerbar,
      builder,
      events,
      search
    }
  }

  // Set icons in ui
  fn setup_childs(&self) {
    let actions: gtk::Box = self.builder.get_object("actions").unwrap();
    let tabs: gtk::Box = self.builder.get_object("tabs").unwrap();

    self.headerbar.pack_start(&tabs);
    self.headerbar.pack_end(&actions);
  }

  fn setup_search(&self) {
    let events = self.events.clone();

    self.search.connect_activate(move |event| {
      let value = event.get_text().unwrap();
      events.borrow_mut().push(Event::GetArticle(value));
    });
  }

  fn subscribe_event(&self) {    
    let headerbar = self.headerbar.clone();
    headerbar.set_title(Some("Home"));
    
    let events = self.events.clone();
    events.borrow_mut().subscribe(move |event| {
      match event {
        Event::GetArticle(name) => {
          headerbar.set_title(Some(&*name));
        },
        Event::OpenPage(_, title) => {
          headerbar.set_title(&*title);
        }
        _ => {}
      }
    });
  }

  pub fn setup(&self) {
    self.subscribe_event();
    self.setup_search();
    self.setup_childs();
  }
}