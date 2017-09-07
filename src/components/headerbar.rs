extern crate gdk_pixbuf;
extern crate gtk;

use utils::navigator::{ Navigator };
use utils::traits::{ Event };
use std::cell::UnsafeCell;
use std::rc::Rc;

use self::gdk_pixbuf::{ Pixbuf };

pub struct AppHeaderBar {
  headerbar: gtk::HeaderBar,
  builder: gtk::Builder,
  search: gtk::Entry
}

impl AppHeaderBar {
  pub fn new() -> AppHeaderBar {
    let builder = gtk::Builder::new_from_resource("/org/gtk/Lurkmore/c_ui/headerbar.xml");
    let headerbar: gtk::HeaderBar = builder.get_object("app_headerbar").unwrap();
    let search: gtk::Entry = builder.get_object("search_input").unwrap();

    AppHeaderBar {
      headerbar,
      builder,
      search
    }
  }

  // connect toggles button to navigator
  unsafe fn setup_routing(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    use gtk::ToggleButtonExt;

    let pages = ["home", "reader"];
    let navigator_ref = navigator.get();
    let events = (*navigator_ref).get_events().get();

    for name in &pages {
      let page = name.clone();
      let btn_query = format!("btn-{}", &page);
      let button: gtk::RadioButton = self.builder.get_object(&btn_query[..]).unwrap();

      button.connect_toggled(move |event| {        
        if event.get_active() {
          (*events).push(Event::OpenPage(String::from(page)));
        }
      });
    }
  }

  // Set icons in ui
  fn setup_icons(&self) {
    use gtk::ImageExt;
    let icons: [&str; 4] = ["home", "reader", "menu", "search"];
    
    for name in &icons {
      let query = format!("btn-{}-image", &name);
      let element: gtk::Image = self.builder.get_object(&query).unwrap();      

      let path = format!("/org/gtk/Lurkmore/images/{}.png", &name);
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
    use gtk::ToggleButtonExt;

    let builder = self.builder.clone();
    let navigator_ref = navigator.get();
    let events = (*navigator_ref).get_events().get();
    
    // (*events).subscribe(move |event| {
    //   match event {
    //     Event::OpenPage(p) => {
    //       let page = &p[..];
    //       let btn_query = format!("btn-{}", &page);
    //       println!("change toggle to: {}", &page);

    //       if page == "home" || page == "reader" {
    //         let button: gtk::RadioButton = builder.get_object(&btn_query[..]).unwrap();
    //         button.set_active(true);
    //       }
    //     },
    //     _ => {}
    //   }
    // });
  }

  pub fn setup(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    unsafe {
      self.setup_toggle_sidebar(navigator);
      self.subscribe_event(navigator);
      self.setup_routing(navigator);
      self.setup_search(navigator);
    }

    self.setup_icons();
  }

  pub fn get_content(&self) -> &gtk::HeaderBar {
    return &self.headerbar;
  }
}