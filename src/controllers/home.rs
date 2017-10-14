use std::cell::RefCell;
use std::rc::Rc;

use utils::traits::{ Controller, View, Event };
use utils::navigator::EventEmitter;
use views::home::Home as HomeView;

pub struct Home {
  view: Rc<RefCell<View>>
}

impl Home {
  pub fn new(events: Rc<RefCell<EventEmitter>>) -> Home {
    let view = HomeView::new(events);
    
    Home {
      view: Rc::new(RefCell::new(view))
    }
  }
}

impl Controller for Home {
  fn get_view(&self) -> Rc<RefCell<View>> {
    self.view.clone()
  }
  
  fn on_receive_event(&self, event: Event) {
    let view = self.view.clone();
    view.borrow().on_receive_event(event);
  }
}