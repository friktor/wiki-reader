use utils::navigator::EventEmitter;
use std::collections::HashMap;
use fluent::MessageContext;
use utils::traits::Event;
use std::cell::RefCell;
use std::rc::Rc;
use utils::t_;
use gtk;

use gtk::ToggleButtonExt;
use gtk::HeaderBarExt;
use gtk::ButtonExt;
use gtk::EntryExt;

fn get_localized_titles(i18n: Rc<RefCell<MessageContext>>) -> HashMap<String, String> {
  let mut titles: HashMap<String, String> = HashMap::new();

  for name in &["home", "about", "reader"] {
    let title_key = &*format!("page-title-{}", &name);
    let title = t_(None, i18n.clone(), title_key);
    titles.insert(String::from(name.clone()), title);
  }

  titles
}

pub struct AppHeaderBar<'a> {
  i18n: Rc<RefCell<MessageContext<'a>>>,
  events: Rc<RefCell<EventEmitter>>,
  pub headerbar: gtk::HeaderBar,
  builder: gtk::Builder,
  search: gtk::Entry
}

impl <'a>AppHeaderBar<'a> {
  pub fn new(
    events: Rc<RefCell<EventEmitter>>,
    i18n: Rc<RefCell<MessageContext>>
  ) -> AppHeaderBar {
    let builder = gtk::Builder::new_from_resource("/org/gtk/wikireader/ui/headerbar.xml");
    let headerbar: gtk::HeaderBar = builder.get_object("app_headerbar").unwrap();
    let search: gtk::Entry = builder.get_object("search_input").unwrap();

    AppHeaderBar {
      headerbar,
      builder,
      events,
      search,
      i18n
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
    let titles = Rc::new(RefCell::new(get_localized_titles(self.i18n.clone())));

    if let Some(_title) = titles.borrow().get(&String::from("home")) {
      let title = _title.clone();
      headerbar.set_title(Some(&*title));
    }

    let builder = self.builder.clone();
    let events = self.events.clone();
    let _titles = titles.clone();

    // TODO: need adding changing tabs by OpenPage nad GetArticle event,
    // without borrowed panic 
    events.borrow_mut().subscribe(move |event| {
      match event {
        Event::GetArticle(name) => {
          headerbar.set_title(Some(&*name));
        },

        Event::OpenPage(page_name, _) => {
          // Change page title
          match _titles.borrow().get(&page_name) {
            Some(title) => headerbar.set_title(&*title.clone()),
            None => headerbar.set_title(&*page_name)
          }
        }
        _ => {}
      }
    });
  }

  fn setup_navs(&self) {
    let pages = vec![ "home", "reader", "about" ];
    let titles = get_localized_titles(self.i18n.clone());

    for name in &pages {
      let events = self.events.clone();
      let titles = titles.clone();
      let name = name.clone();
      
      let button_query = format!("btn-tab-{}", &name);
      let button: gtk::RadioButton = self.builder.get_object(&*button_query).unwrap();
      
      let label_key = String::from(name.clone());
      let label = titles.get(&label_key).unwrap();
      button.set_label(&*label.clone());

      button.connect_toggled(move |event| {        
        if event.get_active() {
          events.borrow_mut().push(Event::OpenPage(
            String::from(name.clone()),
            String::from("")
          ));
        }
      });
    }
  }

  pub fn setup(&self) {
    self.headerbar.set_decoration_layout(Some(""));
    
    self.subscribe_event();
    self.setup_search();
    self.setup_childs();
    self.setup_navs();
  }
}