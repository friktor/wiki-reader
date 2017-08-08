extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use self::gdk_pixbuf::{ Pixbuf };
use std::error::{ Error };
use gtk::prelude::*;

use gtk::{
  Window, WindowPosition, WindowType,
  Button, HeaderBar, Box, Image
};

pub struct Application {
  headerbar: gtk::HeaderBar,
  window: gtk::Window,
}

impl Application {
  pub fn new() -> Application {
    fn window() -> Window {
      let window = Window::new(WindowType::Toplevel);
      window.set_position(WindowPosition::Center);
      window.set_size_request(700, 600);
      window.set_title("Lurkmore");
      window
    }

    fn headerbar() -> HeaderBar {
      let headerbar = HeaderBar::new();
      // headerbar.set_subtitle(Some("Клиент для вики"));
      // headerbar.set_title(Some("Lurkmore"));
      headerbar
    }

    Application {
      headerbar: headerbar(),
      window: window()
    }
  }

  fn setup_headerbar(&self) {
    let leftbox = Box::new(gtk::Orientation::Horizontal, 0);
    leftbox.set_baseline_position(gtk::BaselinePosition::Center);
    
    match Pixbuf::new_from_file_at_size("./images/logo.png", 36, 36) {
      Err(error) => { println!("Error: {}", Error::description(&error)) },
      Ok(logo) => {
        let image = Image::new_from_pixbuf(Some(&logo));
        leftbox.pack_start(&image, false, false, 3);
      }
    }

    let button = Button::new_with_label("Настройки");
    button.connect_clicked(|_| {
      println!("Click epta!");
    });

    leftbox.pack_end(&button, false, false, 3);
    self.headerbar.set_custom_title(&leftbox);

    self.window.set_titlebar(Some(&self.headerbar));
  }

  fn register_quit(&self) {
    self.window.connect_delete_event(|_, _| {
      gtk::main_quit();
      Inhibit(false)
    });
  }

  pub fn prepare_and_run(&self) {
    self.setup_headerbar();
    self.register_quit();

    self.window.show_all();
    gtk::main();
  }
}
