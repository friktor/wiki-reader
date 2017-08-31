extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;
extern crate gio;

use navigator::{ Navigator, NavigatorEvent };
use std::collections::HashMap;
use std::cell::UnsafeCell;
use std::rc::Rc;

use gtk::{ Image, Entry, HeaderBar, RadioButton, Builder, Button };
use self::gdk_pixbuf::{ Pixbuf };
use gtk::ImageExt;

pub struct AppHeaderBar {
  menu: HashMap<String, RadioButton>,
  headerbar: HeaderBar,
  builder: Builder,
  search: Entry
}

impl AppHeaderBar {
  pub fn new() -> AppHeaderBar {
    let builder = Builder::new_from_resource("/org/gtk/Lurkmore/c_ui/headerbar.xml");
    let headerbar: HeaderBar = builder.get_object("app_headerbar").unwrap();
    let search: Entry = builder.get_object("search_input").unwrap();

    let menu: HashMap<String, RadioButton> = hashmap!{
      String::from("reader") => builder.get_object("btn-reader").unwrap(),
      String::from("home") => builder.get_object("btn-home").unwrap(),
    };

    AppHeaderBar {
      headerbar,
      builder,
      search,
      menu
    }
  }

  // connect toggles button to navigator
  fn setup_routing(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    use gtk::ToggleButtonExt;

    for (name, button) in &self.menu {
      let current_page = name.clone();
      let nav = navigator.get();

      button.connect_toggled(move |event| {
        let is_active = event.get_active();
        let page_name = &current_page[..];
        if is_active {
          let page_query = format!("page_{}", page_name);
          unsafe { (*nav).open(page_query); }
        }
      });
    }
  }

  // Set icons in ui
  fn setup_icons(&self) {
    let icons: [&str; 4] = ["home", "reader", "menu", "search"];
    for name in &icons {
      let query = format!("btn-{}-image", &name);
      let element: Image = self.builder.get_object(&query).unwrap();      

      let path = format!("/org/gtk/Lurkmore/images/{}.png", &name);
      let image = Pixbuf::new_from_resource_at_scale(&path, 20, 20, false).unwrap();
      element.set_from_pixbuf(Some(&image));
    }
  }

  fn setup_toggle_sidebar(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    use gtk::ButtonExt;

    let button: Button = self.builder.get_object("toggle_sidebar").unwrap();
    let nav = navigator.get();

    // Send toggle sidebar event
    button.connect_clicked(move |event| {
      unsafe {
        (*nav).push_event(NavigatorEvent::ToggleSidebar);
      }
    });
  }

  fn setup_search(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    use gtk::EntryExt;
    let nav = navigator.get();

    self.search.connect_activate(move |event| {
      let value = event.get_text().unwrap();
      unsafe {
        (*nav).push_event(NavigatorEvent::GetArticle(value))
      }
    });
  }

  pub fn setup(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    self.setup_toggle_sidebar(navigator);
    self.setup_routing(navigator);
    self.setup_search(navigator);
    self.setup_icons();
  }

  pub fn get_content(&self) -> &HeaderBar {
    return &self.headerbar;
  }
}