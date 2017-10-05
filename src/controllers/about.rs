use std::cell::RefCell;
use std::rc::Rc;

use utils::traits::{ Controller, View, Event };
use utils::navigator::EventEmitter;
use views::about::About as AboutView;

pub struct About {
  view: Rc<RefCell<View>>
}

impl About {
  pub fn new(events: Rc<RefCell<EventEmitter>>) -> About {
    let mut view = AboutView::new(events);
    view.setup();
    
    About {
      view: Rc::new(RefCell::new(view))
    }
  }
}

impl Controller for About {
  fn get_view(&self) -> Rc<RefCell<View>> {
    self.view.clone()
  }
  
  fn on_receive_event(&self, event: Event) {
    let view = self.view.clone();
    view.borrow().on_receive_event(event);
  }
}