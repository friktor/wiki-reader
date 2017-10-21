use std::cell::RefCell;
use std::rc::Rc;

use components::wiki_switcher::WikiSwitcher;
use utils::navigator::EventEmitter;
use utils::traits::{ View, Event };
use utils::add_class_to_widget;
use utils::wiki::WikiResource;
use fluent::MessageContext;
use gdk_pixbuf::{ Pixbuf };
use gtk;

use gtk::ListBoxRowExt;
use gtk::ListBoxExt;
use gtk::ButtonExt;
use gtk::ImageExt;
use gtk::EntryExt;
use gtk::BoxExt;

pub struct Home<'a> {
  search_resource: Rc<RefCell<WikiResource>>,
  events: Rc<RefCell<EventEmitter>>,
  wiki_switcher: WikiSwitcher,
  builder: gtk::Builder,
  content: gtk::Box,
  name: &'a str
}

impl <'a>Home<'a> {
  pub fn new(events: Rc<RefCell<EventEmitter>>) -> Home<'a> {
    let builder = gtk::Builder::new_from_resource("/org/wikireader/ui/home.xml");
    let content: gtk::Box = builder.get_object("page_home").unwrap();
    let wiki_switcher = WikiSwitcher::new();

    Home {
      search_resource: Rc::new(RefCell::new(WikiResource::Lurkmore)),
      wiki_switcher,
      name: "home",
      builder,
      content,
      events
    }
  }

  fn prepare_images(&self) {
    let names: [&str; 2] = ["logo", "search"];
    
    for name in &names {
      let id = format!("{}-image", name);
      let image: Option<gtk::Image> = self.builder.get_object(&*id);
      
      if let Some(img) = image {
        let path = format!("/org/wikireader/images/{}.png", &name);
        
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

  fn prepare_switcher(&mut self) {    
    let search_resource = self.search_resource.clone();
    let list = self.wiki_switcher.list.clone();
    self.wiki_switcher.setup();

    list.connect_row_selected(move |_, selected| {
      let row = selected.clone().unwrap();
      let selected_index = row.get_index();
      
      let resource = match selected_index {
        0 => WikiResource::Wikipedia,
        1 => WikiResource::Lurkmore,
        _ => WikiResource::Custom
      };

      *search_resource.borrow_mut() = resource;
    });

    // Pack label search
    let label = gtk::Label::new(
      "Use this page to search for articles and select sources to search."
    );
    
    add_class_to_widget(&label, "label-info");
    self.content.pack_start(&label, false, true, 0);

    // Pack popover switcher to end
    let button = self.wiki_switcher.button.clone();
    
    let search_button: gtk::Button = self.builder.get_object("button-search").unwrap();
    let search_box: gtk::Box = self.builder.get_object("search-box").unwrap();
    
    search_box.pack_start(&button, false, true, 0);
    search_box.pack_start(&search_button, false, true, 0);
  }

  fn prepare_search_action(&self) {
    let entry: gtk::Entry = self.builder.get_object("entry-search").unwrap();
    let button: gtk::Button = self.builder.get_object("button-search").unwrap();

    let search_resource = self.search_resource.clone();
    let events = self.events.clone();
    
    button.connect_clicked(move |_| {
      if let Some(article_name) = entry.get_text() {
        let resource = search_resource.borrow().clone();
        events.borrow_mut().push(Event::GetArticle(article_name, resource));
      }
    });
  }
}

impl <'a>View for Home<'a> {
  fn get_content(&self) -> gtk::Box {
    self.content.clone()
  }

  fn get_name(&self) -> String {
    String::from(self.name)
  }

  fn on_receive_event(&self, event: Event) {
    
  }

  fn setup(&mut self, i18n: Rc<RefCell<MessageContext>>) {
    self.prepare_switcher();
    self.prepare_images();
    self.prepare_search_action();
  }
}