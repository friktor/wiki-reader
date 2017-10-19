use std::collections::HashMap;
use utils::traits::Event;
use std::cell::RefCell;
use std::rc::Rc;
use utils::t_;
use gtk;

use components::wiki_switcher::WikiSwitcher;
use utils::navigator::EventEmitter;
use utils::add_class_to_widget;
use utils::wiki::WikiResource;
use fluent::MessageContext;

use gtk::ToggleButtonExt;
use gtk::ListBoxRowExt;
use gtk::HeaderBarExt;
use gtk::ListBoxExt;
use gtk::ButtonExt;
use gtk::EntryExt;
use gtk::BoxExt;

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
  search_resource: Rc<RefCell<WikiResource>>,
  i18n: Rc<RefCell<MessageContext<'a>>>,
  events: Rc<RefCell<EventEmitter>>,
  pub headerbar: gtk::HeaderBar,
  wiki_switcher: WikiSwitcher,
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
    
    let search_resource = Rc::new(RefCell::new(WikiResource::Lurkmore));
    let wiki_switcher = WikiSwitcher::new();

    AppHeaderBar {
      search_resource,
      wiki_switcher,
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
    let search_resource = self.search_resource.clone();
    let events = self.events.clone();

    self.search.connect_activate(move |event| {
      let resource = search_resource.borrow().clone();
      let value = event.get_text().unwrap();

      events.borrow_mut().push(Event::GetArticle(value, resource));
    });
  }

  fn prepare_switcher(&mut self) {    
    let search_resource = self.search_resource.clone();
    let list = self.wiki_switcher.list.clone();
    self.wiki_switcher.setup();

    list.connect_row_selected(move |_, selected| {
      let row = selected.clone().unwrap();
      let selected_index = row.get_index();
      
      let resource = match selected_index {
        0 => WikiResource::Wikipedia,
        1 => WikiResource::Lurkmore,
        _ => WikiResource::Custom
      };

      *search_resource.borrow_mut() = resource;
    });

    let actions: gtk::Box = self.builder.get_object("actions").unwrap();
    let button = self.wiki_switcher.button.clone();
    actions.pack_start(&button, false, true, 0);
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
        Event::GetArticle(name, _) => {
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

  pub fn setup(&mut self) {
    self.headerbar.set_decoration_layout(Some(""));
    
    self.prepare_switcher();
    self.subscribe_event();
    self.setup_search();
    self.setup_childs();
    self.setup_navs();
  }
}