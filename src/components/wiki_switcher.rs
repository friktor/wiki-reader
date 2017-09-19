extern crate inflections;
extern crate gdk_pixbuf;
extern crate gtk;

use self::inflections::case::to_lower_case;
use self::gdk_pixbuf::{ Pixbuf };
use utils::add_class_to_widget;
use utils::wiki::WikiResource;

use std::cell::{ RefCell };
use std::ops::FnMut;
use std::rc::Rc;

pub struct WikiSwitcher {
  pub button: Rc<RefCell<gtk::Button>>,
  pub list: Rc<RefCell<gtk::ListBox>>,
  popover: Rc<RefCell<gtk::Popover>>,
}

impl WikiSwitcher {
  pub fn new() -> WikiSwitcher {
    use gtk::ContainerExt;
    use gtk::PopoverExt;

    let button = gtk::Button::new_with_label("Lurkmore");
    add_class_to_widget(&button, "button-wiki-switcher");
    
    let popover = gtk::Popover::new(Some(&button));
    popover.set_position(gtk::PositionType::Top);

    let list = gtk::ListBox::new();
    popover.add(&list);

    WikiSwitcher {
      popover: Rc::new(RefCell::new(popover)),
      button: Rc::new(RefCell::new(button)),
      list: Rc::new(RefCell::new(list)),
    }
  }

  fn prepare_visible(&self) {
    use gtk::ButtonExt;
    use gtk::WidgetExt;

    let popover = self.popover.clone();
    let button = self.button.clone();

    button.borrow().connect_clicked(move |event| {
      let menu = popover.borrow();
      if menu.get_visible() {
        menu.hide();
      } else {
        menu.show_all();
      }
    });
  }

  fn prepare_list(&self) {
    use gtk::ListBoxRowExt;
    use gtk::ListBoxExt;
    use gtk::ButtonExt;
    use gtk::BoxExt;

    let resources = ["Wikipedia", "Lurkmore"];
    let button = self.button.clone();
    let list = self.list.clone();

    for (i, resource) in resources.iter().enumerate() {
      let row = gtk::Box::new(gtk::Orientation::Horizontal, 0);
      add_class_to_widget(&row, "popover-row");
      
      let path = format!(
        "/org/gtk/wikireader/images/{}-logo.png",
        to_lower_case(resource.clone())
      );

      let buffer = Pixbuf::new_from_resource_at_scale(&path, 25, 25, false).unwrap();
      
      let image = gtk::Image::new_from_pixbuf(Some(&buffer));
      add_class_to_widget(&image, "icon");

      let label = gtk::Label::new(resource.clone());
      add_class_to_widget(&label, "label");

      row.pack_start(&image, false, false, 0);
      row.pack_start(&label, false, false, 0);

      let index = i as i32;
      list.borrow().insert(&row, index);
    }

    // Set Lurkmore default row
    if let Some(row) = list.borrow().get_row_at_index(1) {
      list.borrow().select_row(Some(&row));
    }

    // Connect to selected for change button label
    list.borrow().connect_row_selected(move |list, selected| {
      let row = selected.clone().unwrap();
      let selected_index = row.get_index();

      match selected_index {
        0 => button.borrow().set_label("Wikipedia"),
        1 => button.borrow().set_label("Lurkmore"),
        _ => {}
      }
    });
  }

  pub fn setup(&self) {
    self.prepare_visible();
    self.prepare_list();
  }
}