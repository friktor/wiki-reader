extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use self::gdk_pixbuf::{ Pixbuf };
use self::gdk::{ Screen };
use pages::{ Pages };
use gtk::prelude::*;

use gtk::{
  Window, Button, Image, Builder, Box, StyleContext,
  Revealer, Settings, CssProvider, HeaderBar, RadioButton
};

pub struct Application {
  headerbar: HeaderBar,
  settings: Settings,
  builder: Builder,
  window: Window,
  pages: Pages
}

impl Application {
  pub fn new() -> Application {
    let builder = Builder::new_from_string(include_str!("./ui/app.xml"));

    // Default components
    let headerbar = builder.get_object("app_headerbar").unwrap();
    let window = builder.get_object("app_window").unwrap();
    let settings = Settings::get_default().unwrap();
    
    // Content containers
    let pages = Pages::new();
    pages.prepare_stack();

    Application {
      headerbar: headerbar,
      settings: settings,
      builder: builder,
      window: window,
      pages: pages
    }
  }

  fn setup_settings(&self) {
    self.settings.set_property_gtk_enable_animations(true);
    self.settings.set_property_gtk_theme_name(Some("Arc"));

    let screen = Screen::get_default().unwrap();
    let style = include_str!("./ui/styles.css");
    
    let provider = CssProvider::new();
    provider.load_from_data(style).unwrap();

    StyleContext::add_provider_for_screen(
      &screen, &provider,
      gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
  }

  fn setup_sidebar(&self) {
    // Setup show sidebar reveal
    let button: Button = self.builder.get_object("settings_button").unwrap();
    let sidebar: Revealer = self.builder.get_object("sidebar").unwrap();

    button.connect_clicked(move |_| {
      let is_opened = sidebar.get_reveal_child();
      sidebar.set_reveal_child(!is_opened);
    });

    // Preparation images
    let logo_element: Image = self.builder.get_object("sidebar-logo-image").unwrap();
    let image = Pixbuf::new_from_resource_at_scale("/org/gtk/Lurkmore/images/logo.png", 100, 100, false).unwrap();
    logo_element.set_from_pixbuf(Some(&image));
  }

  fn setup_content(&self) {
    let container: Box = self.builder.get_object("app_container").unwrap();
    let content = self.pages.get_content();
    container.pack_end(content, true, true, 0);
  }

  fn setup_headerbar(&self) {
    let icons: [&str; 4] = ["home", "reader", "menu", "search"];
    for name in &icons {
      let query = format!("btn-{}-image", &name);
      let element: Image = self.builder.get_object(&query).unwrap();      

      let path = format!("/org/gtk/Lurkmore/images/{}.png", &name);
      let image = Pixbuf::new_from_resource_at_scale(&path, 20, 20, false).unwrap();
      element.set_from_pixbuf(Some(&image));
    }

    // Radio handlers
    let pages: [&str; 2] = ["home", "reader"];

    for page in &pages {
      let query = format!("btn-{}", &page);
      let element: RadioButton = self.builder.get_object(&query).unwrap();
      
      let content_pages = self.pages.get_content().to_owned();
      let current_page = page.clone();

      element.connect_toggled(move |event| {
        let is_active = event.get_active();
        if is_active {
          let page_query = format!("page_{}", &current_page);
          content_pages.set_visible_child_name(&page_query);
        }
      });
    }
  }

  fn register_quit(&self) {
    self.window.connect_delete_event(|_, _| {
      gtk::main_quit();
      Inhibit(false)
    });
  }

  pub fn prepare_and_run(&self) {
    self.setup_headerbar();
    self.setup_settings();
    self.setup_content();
    self.setup_sidebar();
    self.register_quit();

    self.window.show_all();
    gtk::main();
  }
}
