extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use self::gdk_pixbuf::{ Pixbuf };
use self::gdk::{ Screen };
use gtk::prelude::*;

use gtk::{
  Button, Image, Builder, Box, StyleContext,
  Revealer, Settings, CssProvider,
};

pub struct Application {
  headerbar: gtk::HeaderBar,
  settings: gtk::Settings,
  builder: gtk::Builder,
  window: gtk::Window
}

impl Application {
  pub fn new() -> Application {
    let interface = include_str!("./interface.xml");
    let builder = Builder::new_from_string(interface);

    let headerbar = builder.get_object("app_headerbar").unwrap();
    let window = builder.get_object("app_window").unwrap();
    let settings = Settings::get_default().unwrap();

    Application {
      headerbar: headerbar,
      settings: settings,
      builder: builder,
      window: window
    }
  }

  fn setup_settings(&self) {
    self.settings.set_property_gtk_enable_animations(true);
    self.settings.set_property_gtk_theme_name(Some("Arc"));

    let screen = Screen::get_default().unwrap();
    let style = include_str!("app.css");
    
    let provider = CssProvider::new();
    provider.load_from_data(style).unwrap();

    StyleContext::add_provider_for_screen(
      &screen, &provider,
      gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
  }

  fn setup_headerbar(&self) {
    let image: Image = self.builder.get_object("app_header_logo").unwrap();
    match Pixbuf::new_from_file_at_size("./images/logo.png", 39, 36) {
      Ok(buffer) => { image.set_from_pixbuf(&buffer) },
      Err(error) => { }
    }
  }

  fn setup_sidebar(&self) {
    let button: Button = self.builder.get_object("settings_button").unwrap();
    let sidebar: Revealer = self.builder.get_object("sidebar").unwrap();

    button.connect_clicked(move |_| {
      let is_opened = sidebar.get_reveal_child();
      sidebar.set_reveal_child(!is_opened);
    });
  }

  fn register_quit(&self) {
    self.window.connect_delete_event(|_, _| {
      gtk::main_quit();
      Inhibit(false)
    });
  }

  pub fn prepare_and_run(&self) {
    self.setup_settings();
    self.setup_headerbar();
    self.setup_sidebar();
    self.register_quit();

    self.window.show_all();
    gtk::main();
  }
}
