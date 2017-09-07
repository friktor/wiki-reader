extern crate gtk;

use std::cell::UnsafeCell;
use std::rc::Rc;

use utils::navigator::{ Navigator, EventEmitter };
use utils::traits::{ Controller, View, Event };
use views::reader::Reader as ReaderView;

pub struct Reader {
  view: Rc<UnsafeCell<View>>
}

impl Reader {
  pub fn new(events: &Rc<UnsafeCell<EventEmitter>>) -> Reader {
    let view = ReaderView::new(events);
    
    Reader {
      view: Rc::new(UnsafeCell::new(view))
    }
  }
}

impl Controller for Reader {
  fn on_receive_event(&self, event: Event) {
    let view = self.view.get();
    unsafe { (*view).on_receive_event(event.clone()) }
  }

  fn get_view(&self) -> &Rc<UnsafeCell<View>> {
    &self.view
  }
}