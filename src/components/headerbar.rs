extern crate gdk_pixbuf;
extern crate gtk;

use utils::navigator::{ Navigator };
use utils::traits::{ Event };
use std::cell::UnsafeCell;
use std::rc::Rc;

use self::gdk_pixbuf::{ Pixbuf };

pub struct AppHeaderBar {
  headerbar: Rc<UnsafeCell<gtk::HeaderBar>>,
  builder: gtk::Builder,
  search: gtk::Entry
}

impl AppHeaderBar {
  pub fn new() -> AppHeaderBar {
    let builder = gtk::Builder::new_from_resource("/org/gtk/wikireader/c_ui/headerbar.xml");
    let headerbar: gtk::HeaderBar = builder.get_object("app_headerbar").unwrap();
    let search: gtk::Entry = builder.get_object("search_input").unwrap();

    AppHeaderBar {
      headerbar: Rc::new(UnsafeCell::new(headerbar)),
      builder,
      search
    }
  }

  // Set icons in ui
  fn setup_icons(&self) {
    use gtk::ImageExt;
    let icons: [&str; 2] = ["menu", "search"]; // "home", "reader", 
    
    for name in &icons {
      let query = format!("btn-{}-image", &name);
      let element: gtk::Image = self.builder.get_object(&query).unwrap();      

      let path = format!("/org/gtk/wikireader/images/{}.png", &name);
      let image = Pixbuf::new_from_resource_at_scale(&path, 20, 20, false).unwrap();
      element.set_from_pixbuf(Some(&image));
    }
  }

  unsafe fn setup_toggle_sidebar(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    use gtk::ButtonExt;

    let button: gtk::Button = self.builder.get_object("toggle_sidebar").unwrap();
    let navigator_ref = navigator.get();
    let events = (*navigator_ref).get_events().get();

    button.connect_clicked(move |event| {
      (*events).push(Event::ToggleSidebar);
    });
  }

  unsafe fn setup_search(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    use gtk::EntryExt;
    
    let navigator_ref = navigator.get();
    let events = (*navigator_ref).get_events().get();

    self.search.connect_activate(move |event| {
      let value = event.get_text().unwrap();
      (*events).push(Event::GetArticle(value));
    });
  }

  unsafe fn subscribe_event(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    use gtk::HeaderBarExt;
    
    let headerbar = self.headerbar.get();
    let navigator_ref = navigator.get();

    // Default title
    (*headerbar).set_title(Some("Home"));

    let events_ref = (*navigator_ref).get_events();
    let events = events_ref.get();

    (*events).subscribe(move |event| {
      match event {
        Event::GetArticle(name) => {
          let title = (*navigator_ref).get_page_title("reader");
          (*headerbar).set_subtitle(Some(&name[..]));
          (*headerbar).set_title(title);
        },

        Event::OpenPage(name) => {
          let title = (*navigator_ref).get_page_title(&name[..]);
          (*headerbar).set_subtitle(None);
          (*headerbar).set_title(title);
        }
        _ => {}
      }
    });
  }

  pub fn setup(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    unsafe {
      self.setup_toggle_sidebar(navigator);
      self.subscribe_event(navigator);
      self.setup_search(navigator);
    }

    self.setup_icons();
  }

  pub fn get_content(&self) -> &Rc<UnsafeCell<gtk::HeaderBar>> {
    return &self.headerbar;
  }
}