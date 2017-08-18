extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use gtk::{ Box };
use gtk::BoxExt;

pub struct Reader {
  page_name: String,
  is_starter: bool,
  content: Box
}

impl Reader {
  pub fn new() -> Reader {
    let content = Box::new(gtk::Orientation::Vertical, 0);

    Reader {
      page_name: String::new(),
      is_starter: true,
      content: content
    }
  }

  pub fn prepare_reader(&self, starter_page: Box) {
    self.content.pack_start(&starter_page, true, true, 0);
  }

  pub fn get_content(&self) -> &Box {
    return &self.content;
  }
}