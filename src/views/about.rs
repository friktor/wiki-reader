use std::cell::RefCell;
use std::rc::Rc;

use utils::navigator::EventEmitter;
use utils::traits::{ View, Event };
use utils::add_class_to_widget;
use fluent::types::FluentValue;
use fluent::MessageContext;
use gdk_pixbuf::{ Pixbuf };
use gtk;

use gtk::WidgetExt;
use gtk::LabelExt;
use gtk::ImageExt;
use gtk::BoxExt;

pub struct About<'a> {
  events: Rc<RefCell<EventEmitter>>,
  builder: gtk::Builder,
  content: gtk::Box,
  name: &'a str
}

impl <'a>About<'a> {
  pub fn new(events: Rc<RefCell<EventEmitter>>) -> About<'a> {
    let builder = gtk::Builder::new_from_resource("/org/gtk/wikireader/ui/about.xml");
    let content: gtk::Box = builder.get_object("page_about").unwrap();

    About {
      name: "about",
      builder,
      content,
      events
    }
  }

  fn setup_description(&self) {
    let header = gtk::Label::new(None);
    header.set_markup("<b>WikiReader</b> - simple reader for wiki-sites");
    header.set_valign(gtk::Align::Center);
    add_class_to_widget(&header, "header");

    let author = gtk::Label::new(None);
    author.set_markup("Powered by <a href=\"https://github.com/friktor\">Anton Shramko</a>");
    author.set_valign(gtk::Align::Center);
    add_class_to_widget(&author, "subheader");

    self.content.pack_start(&header, false, false, 0);
    self.content.pack_start(&author, false, false, 0);
  }

  fn prepare_images(&self) {
    let names = ["logo"];
    
    for name in &names {
      let id = format!("{}-image", name);
      let image: Option<gtk::Image> = self.builder.get_object(&*id);
      
      if let Some(img) = image {
        let path = format!("/org/gtk/wikireader/images/{}.png", &name);
        
        let size = match name.clone() {
          "search" => 15,
          "logo" => 100,
          _ => 15
        };

        let buffer = Pixbuf::new_from_resource_at_scale(
          &path, size.clone(), size.clone(), false
        ).unwrap();

        img.set_from_pixbuf(Some(&buffer));
      }
    }
  }
}

impl <'a>View for About<'a> {
  fn get_content(&self) -> gtk::Box {
    self.content.clone()
  }

  fn get_name(&self) -> String {
    String::from(self.name)
  }

  fn on_receive_event(&self, event: Event) {
    
  }

  fn setup(&mut self, i18n: Rc<RefCell<MessageContext>>) {
    self.prepare_images();
    self.setup_description();
  }
}