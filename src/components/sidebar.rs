extern crate inflections;
extern crate gdk_pixbuf;
extern crate gtk;

use self::inflections::case::to_lower_case;
use utils::navigator::{ Navigator };
use self::gdk_pixbuf::{ Pixbuf };
use utils::add_class_to_widget;
use utils::traits::{ Event };
use std::cell::UnsafeCell;
use std::rc::Rc;

pub struct AppSidebar {
  container: Rc<UnsafeCell<gtk::Revealer>>,
  content: Rc<UnsafeCell<gtk::Box>>
}

impl AppSidebar {
  fn get_menu_button(name: &str) -> gtk::Button {
    use gtk::ContainerExt;
    use gtk::BoxExt;

    let text = to_lower_case(&name);

    let button = gtk::Button::new();
    add_class_to_widget(&button, "menu-button-root");

    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.set_baseline_position(gtk::BaselinePosition::Center);
    add_class_to_widget(&content, "button");

    let icon_path = format!("/org/gtk/wikireader/images/{}.png", &*text);
    let pixbuf = Pixbuf::new_from_resource_at_scale(&*icon_path, 35, 35, false).unwrap();

    let icon = gtk::Image::new_from_pixbuf(Some(&pixbuf));
    add_class_to_widget(&icon, "icon");

    let label = gtk::Label::new(Some(name));
    add_class_to_widget(&label, "label");

    content.pack_start(&icon, false, false, 0);
    content.pack_end(&label, false, false, 0);
    button.add(&content);

    return button;
  }

  fn get_sidebar_content() -> gtk::Box {
    use gtk::BoxExt;

    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    add_class_to_widget(&content, "sidebar");

    let header = AppSidebar::get_sidebar_header();
    content.pack_start(&header, false, true, 0);

    return content;
  }

  fn get_sidebar_header() -> gtk::Box {
    use gtk::BoxExt;

    let header = gtk::Box::new(gtk::Orientation::Vertical, 0);
    header.set_baseline_position(gtk::BaselinePosition::Center);
    add_class_to_widget(&header, "sidebar-header");

    let pixbuf = Pixbuf::new_from_resource_at_scale(
      "/org/gtk/wikireader/images/logo.png",
      60, 60, false
    ).unwrap();

    let header_image = gtk::Image::new_from_pixbuf(Some(&pixbuf));
    header.pack_start(&header_image, false, false, 0);

    return header;
  }

  fn get_sidebar() -> gtk::Revealer {
    use gtk::RevealerExt;
    let sidebar = gtk::Revealer::new();

    sidebar.set_transition_type(gtk::RevealerTransitionType::SlideLeft);
    sidebar.set_transition_duration(250);
    sidebar.set_reveal_child(false);
    add_class_to_widget(&sidebar, "sidebar-root");

    return sidebar;
  }

  pub fn new() -> AppSidebar {
    use gtk::ContainerExt;

    let sidebar = AppSidebar::get_sidebar();
    let content = AppSidebar::get_sidebar_content();
    sidebar.add(&content);

    AppSidebar {
      container: Rc::new(UnsafeCell::new(sidebar)),
      content: Rc::new(UnsafeCell::new(content)),
    }
  }

  unsafe fn setup_buttons(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    use gtk::StyleContextExt;
    use gtk::WidgetExt;
    use gtk::BoxExt;

    let items: [&str; 3] = [ "Home", "Settings", "About"];
    let navigator_ref = navigator.get();
    let content = self.content.get();

    let events_ref = (*navigator_ref).get_events();
    let events = events_ref.get();

    let list = gtk::Box::new(gtk::Orientation::Vertical, 0);
    add_class_to_widget(&list, "menu-list");

    for name in &items {
      let button = AppSidebar::get_menu_button(&name);
      list.pack_start(&button, false, false, 0);
    }

    (*content).pack_start(&list, false, false, 0);
  }

  unsafe fn subscribe_event(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    use gtk::RevealerExt;

    let navigator_ref = navigator.get();
    let sidebar = self.container.get();

    let events_ref = (*navigator_ref).get_events();
    let events = events_ref.get();

    (*events).subscribe(move |event| {
      match event {
        Event::ToggleSidebar => {
          let status = (*sidebar).get_reveal_child();
          (*sidebar).set_reveal_child(!status);
        },
        _ => {}
      }
    });
  }

  pub fn setup(&self, navigator: &Rc<UnsafeCell<Navigator>>) {
    unsafe {
      self.setup_buttons(navigator);
      self.subscribe_event(navigator);
    }
  }

  pub fn get_content(&self) -> &Rc<UnsafeCell<gtk::Revealer>> {
    return &self.container;
  }
}