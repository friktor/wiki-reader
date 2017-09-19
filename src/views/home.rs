extern crate gdk_pixbuf;
extern crate gtk;

use std::cell::UnsafeCell;
use std::rc::Rc;

use utils::navigator::{ Navigator, EventEmitter };
use components::wiki_switcher::WikiSwitcher;
use utils::traits::{ View, Event };
use self::gdk_pixbuf::{ Pixbuf };
use utils::add_class_to_widget;
use utils::wiki::WikiResource;

pub struct Home<'a> {
  search_resource: Rc<UnsafeCell<WikiResource>>,
  events: Rc<UnsafeCell<EventEmitter>>,
  wiki_switcher: WikiSwitcher,
  builder: gtk::Builder,
  content: gtk::Box,
  title: String,
  name: &'a str
}

impl <'a>Home<'a> {
  pub fn new(events: &Rc<UnsafeCell<EventEmitter>>) -> Home<'a> {
    let builder = gtk::Builder::new_from_resource("/org/gtk/wikireader/c_ui/home.xml");
    let content: gtk::Box = builder.get_object("page_home").unwrap();
    let wiki_switcher = WikiSwitcher::new();

    Home {
      search_resource: Rc::new(UnsafeCell::new(WikiResource::Lurkmore)),
      title: String::from("Home"),
      events: events.clone(),
      wiki_switcher,
      name: "home",
      builder,
      content
    }
  }

  fn prepare_images(&self) {
    use gtk::ImageExt;
    let names: [&str; 2] = ["logo", "search"];
    
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

  unsafe fn prepare_switcher(&mut self) {
    use gtk::ListBoxRowExt;
    use gtk::ListBoxExt;
    use gtk::BoxExt;
    
    let search_resource = self.search_resource.get();
    let list = self.wiki_switcher.list.clone();
    self.wiki_switcher.setup();

    list.borrow_mut().connect_row_selected(move |list, selected| {
      let row = selected.clone().unwrap();
      let selected_index = row.get_index();
      
      let resource = match selected_index {
        0 => WikiResource::Wikipedia,
        1 => WikiResource::Lurkmore,
        _ => WikiResource::Custom
      };

      // *search_resource = resource;
    });

    // Pack label search
    let label = gtk::Label::new(
      "Use this page to search for articles and select sources to search."
    );
    
    add_class_to_widget(&label, "label-info");
    self.content.pack_start(&label, false, true, 0);

    // Pack popover switcher to end
    let button = self.wiki_switcher.button.clone();
    self.content.pack_end(&*button.borrow(), false, false, 0);
  }

  unsafe fn prepare_search_action(&self) {
    use gtk::ButtonExt;
    use gtk::EntryExt;

    let entry: gtk::Entry = self.builder.get_object("entry-search").unwrap();
    let button: gtk::Button = self.builder.get_object("button-search").unwrap();

    let events = self.events.get();
    button.connect_clicked(move |_| {
      if let Some(article_name) = entry.get_text() {
        (*events).push(Event::GetArticle(article_name));
      }
    });
  }
}

impl <'a>View for Home<'a> {
  fn get_content(&self) -> &gtk::Box {
    &self.content
  }

  fn get_name(&self) -> &str {
    self.name
  }

  fn get_title(&self) -> &str {
    &self.title[..]
  }

  fn on_receive_event(&self, event: Event) {
    
  }

  fn setup(&mut self) {
    self.prepare_images();

    unsafe {
      self.prepare_switcher();
      self.prepare_search_action();
    }
  }
}