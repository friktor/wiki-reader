extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use std::cell::UnsafeCell;
use std::rc::Rc;

use navigator::{ Navigator, NavigatorEvent };
use headerbar::{ AppHeaderBar };

use self::gdk_pixbuf::{ Pixbuf };
use self::gdk::{ Screen };
use gtk::prelude::*;

use gtk::{
  Window, Button, Image, Builder, Box, StyleContext, Entry,
  Revealer, Settings, CssProvider, HeaderBar, RadioButton
};

pub struct Application {
  navigator: Rc<UnsafeCell<Navigator>>,
  headerbar: AppHeaderBar,
  settings: Settings,
  builder: Builder,
  window: Window,
}

impl Application {
  pub fn new() -> Application {
    // Setup&get containers
    let builder = Builder::new_from_resource("/org/gtk/Lurkmore/c_ui/app.xml");
    let window = builder.get_object("app_window").unwrap();
    let settings = Settings::get_default().unwrap();
    
    // Make navigator
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
    let nav_ref = self.navigator.get();
    (*nav_ref).setup();

    let container: gtk::Box = self.builder.get_object("app_container").unwrap();
    let stack = (*nav_ref).stack.get();
    
    // Add navigator block to window
    container.pack_end(&*stack, true, true, 0);

    // Listener event toogle
    let builder = self.builder.clone();
    (*nav_ref).register_listener(move |event| {
      let sidebar: Revealer = builder.get_object("sidebar").unwrap();
      let status = sidebar.get_reveal_child();

      match event {
        NavigatorEvent::ToggleSidebar => {
          sidebar.set_reveal_child(!status);
        },
        _ => {}
      }
    });
  }

  fn setup_headerbar(&self) {
    let headerbar = self.headerbar.get_content();
    self.window.set_titlebar(headerbar);
  }

  fn setup_settings(&self) {
    self.settings.set_property_gtk_enable_animations(true);
    self.settings.set_property_gtk_theme_name(Some("Arc"));

    let screen = Screen::get_default().unwrap();
    let provider = CssProvider::new();
    provider.load_from_path("./bundles/main.css").unwrap();

    StyleContext::add_provider_for_screen(
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