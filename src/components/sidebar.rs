use inflections::case::to_lower_case;
use utils::navigator::EventEmitter;
use utils::add_class_to_widget;
use utils::traits::Event;
use gdk_pixbuf::Pixbuf;
use std::cell::RefCell;
use std::rc::Rc;
use gtk;

use gtk::ContainerExt;
use gtk::RevealerExt;
use gtk::BoxExt;

pub struct AppSidebar {
  events: Rc<RefCell<EventEmitter>>,
  pub container: gtk::Revealer,
  pub content: gtk::Box
}

impl AppSidebar {
  pub fn new(events: Rc<RefCell<EventEmitter>>) -> AppSidebar {
    let sidebar = AppSidebar::get_sidebar();
    let content = AppSidebar::get_sidebar_content();
    sidebar.add(&content);

    AppSidebar {
      container: sidebar,
      content,
      events,
    }
  }

  fn get_menu_button(name: &str) -> gtk::Button {
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
    let sidebar = gtk::Revealer::new();

    sidebar.set_transition_type(gtk::RevealerTransitionType::SlideLeft);
    sidebar.set_transition_duration(250);
    sidebar.set_reveal_child(false);
    add_class_to_widget(&sidebar, "sidebar-root");

    return sidebar;
  }

  fn setup_buttons(&self) {
    let items: [&str; 3] = [ "Home", "Settings", "About"];
    let content = self.content.clone();
    let events = self.events.clone();

    let list = gtk::Box::new(gtk::Orientation::Vertical, 0);
    add_class_to_widget(&list, "menu-list");

    for name in &items {
      let button = AppSidebar::get_menu_button(&name);
      list.pack_start(&button, false, false, 0);
    }

    content.pack_start(&list, false, false, 0);
  }

  fn subscribe_event(&self) {
    let sidebar = self.container.clone();
    let events = self.events.clone();

    events.borrow_mut().subscribe(move |event| {
      match event {
        Event::ToggleSidebar => {
          let status = sidebar.get_reveal_child();
          sidebar.set_reveal_child(!status);
        },
        _ => {}
      }
    });
  }

  pub fn setup(&self) {
    self.setup_buttons();
    self.subscribe_event();
  }
}