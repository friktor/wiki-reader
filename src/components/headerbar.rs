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
  fn setup_icons(&self) {
    let icons: [&str; 2] = ["menu", "search"]; // "home", "reader", 
    
    for name in &icons {
      let query = format!("btn-{}-image", &name);
      let element: gtk::Image = self.builder.get_object(&query).unwrap();      

      let path = format!("/org/gtk/wikireader/images/{}.png", &name);
      let image = Pixbuf::new_from_resource_at_scale(&path, 20, 20, false).unwrap();
      element.set_from_pixbuf(Some(&image));
    }
  }

  fn setup_toggle_sidebar(&self) {
    let button: gtk::Button = self.builder.get_object("toggle_sidebar").unwrap();
    let events = self.events.clone();

    button.connect_clicked(move |_| {
      events.borrow_mut().push(Event::ToggleSidebar);
    });
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
    self.setup_toggle_sidebar();
    self.subscribe_event();
    self.setup_search();
    self.setup_icons();
  }
}