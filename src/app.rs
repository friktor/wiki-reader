extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use components::headerbar::AppHeaderBar;
use utils::navigator::Navigator;
use utils::traits::{ Event };
use std::cell::UnsafeCell;
use std::rc::Rc;

use self::gdk::{ Screen };
use gtk::prelude::*;

pub struct Application {
  navigator: Rc<UnsafeCell<Navigator>>,
  headerbar: AppHeaderBar,
  settings: gtk::Settings,
  builder: gtk::Builder,
  window: gtk::Window,
}

impl Application {
  pub fn new() -> Application {
    // Setup&get containers
    let builder = gtk::Builder::new_from_resource("/org/gtk/Lurkmore/c_ui/app.xml");
    let window: gtk::Window = builder.get_object("app_window").unwrap();
    let settings = gtk::Settings::get_default().unwrap();

    let navigator = Rc::new(UnsafeCell::new(Navigator::new()));
    let headerbar = AppHeaderBar::new();
    headerbar.setup(&navigator);
    
    // final blocks
    Application {
      navigator,
      headerbar,
      settings,
      builder,
      window
    }
  }

  unsafe fn setup_navigator(&self) {
    let builder = self.builder.clone();
    let navigator_ref = self.navigator.get();
    (*navigator_ref).setup();
    let events = (*navigator_ref).get_events().get();

    let container: gtk::Box = self.builder.get_object("app_container").unwrap();
    let stack = (*navigator_ref).stack.get();
    container.pack_end(&*stack, true, true, 0);

    (*events).subscribe(move |event| {
      let sidebar: gtk::Revealer = builder.get_object("sidebar").unwrap();
      let status = sidebar.get_reveal_child();

      match event {
        Event::ToggleSidebar => sidebar.set_reveal_child(!status),
        _ => {}
      }
    });
  }

  fn setup_headerbar(&self) {
    let headerbar = self.headerbar.get_content().get();
    unsafe { self.window.set_titlebar(&*headerbar) }
  }

  fn setup_settings(&self) {
    self.settings.set_property_gtk_enable_animations(true);
    self.settings.set_property_gtk_theme_name(Some("Arc-Dark"));

    let screen = Screen::get_default().unwrap();
    let provider = gtk::CssProvider::new();
    provider.load_from_path("./bundles/main.css").unwrap();

    gtk::StyleContext::add_provider_for_screen(
      &screen, &provider,
      gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
  }

  fn register_quit(&self) {
    self.window.connect_delete_event(|_, _| {
      gtk::main_quit();
      Inhibit(false)
    });
  }

  pub fn run(&self) {
    unsafe { self.setup_navigator(); }
    self.setup_headerbar();
    self.setup_settings();
    self.register_quit();

    self.window.show_all();
    gtk::main();
  }
}