use std::cell::RefCell;
use std::rc::Rc;

use utils::navigator::{ Navigator, EventEmitter };
use utils::traits::{ Controller, View, Event };
use views::reader::Reader as ReaderView;

pub struct Reader {
  view: Rc<RefCell<View>>
}

impl Reader {
  pub fn new(events: Rc<RefCell<EventEmitter>>) -> Reader {
    let view = ReaderView::new(events);
    
    Reader {
      view: Rc::new(RefCell::new(view))
    }
  }
}

impl Controller for Reader {
  fn on_receive_event(&self, event: Event) {
    let view = self.view.clone();
    view.borrow().on_receive_event(event.clone());
  }

  fn get_view(&self) -> Rc<RefCell<View>> {
    self.view.clone()
  }
}