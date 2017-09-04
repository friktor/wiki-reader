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
  headerbar: HeaderBar,
  builder: Builder,
  search: Entry
}

impl AppHeaderBar {
  pub fn new() -> AppHeaderBar {
    let builder = Builder::new_from_resource("/org/gtk/Lurkmore/c_ui/headerbar.xml");
    let headerbar: HeaderBar = builder.get_object("app_headerbar").unwrap();
    let search: Entry = builder.get_object("search_input").unwrap();

    AppHeaderBar {
      headerbar,
      builder,
      search
    }
  }

  // connect toggles button to navigator
  fn setup_routing(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    use gtk::ToggleButtonExt;
    let pages = ["home", "reader"];
    let nav = navigator.get();

    for name in &pages {
      let page = name.clone();
      let btn_query = format!("btn-{}", &page);
      let button: RadioButton = self.builder.get_object(&btn_query[..]).unwrap();

      button.connect_toggled(move |event| {        
        if event.get_active() {unsafe {
          (*nav).open(String::from(page));
        }}
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

  pub fn subscribe_event(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    use gtk::ToggleButtonExt;
    let builder = self.builder.clone();
    let nav = navigator.get();
    
    unsafe {(*nav).register_listener(move |event| {
      println!("receive event from headerbar");

      match event {
        NavigatorEvent::OpenPage(p) => {
          let page = &p[..];
          let btn_query = format!("btn-{}", &page);
          println!("change toggle to: {}", &page);

          if page == "home" || page == "reader" {
            let button: RadioButton = builder.get_object(&btn_query[..]).unwrap();
            button.set_active(true);
          }
        },
        _ => {}
      }
    })}
  }

  pub fn setup(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    self.setup_toggle_sidebar(navigator);
    self.subscribe_event(navigator);
    self.setup_routing(navigator);
    self.setup_search(navigator);
    self.setup_icons();
  }

  pub fn get_content(&self) -> &HeaderBar {
    return &self.headerbar;
  }
}