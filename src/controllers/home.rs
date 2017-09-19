extern crate gtk;

use std::cell::UnsafeCell;
use std::rc::Rc;

use utils::navigator::{ Navigator, EventEmitter };
use utils::traits::{ Controller, View, Event };
use views::home::Home as HomeView;

pub struct Home {
  view: Rc<UnsafeCell<View>>
}

impl Home {
  pub fn new(events: &Rc<UnsafeCell<EventEmitter>>) -> Home {
    let mut view = HomeView::new(events);
    view.setup();
    
    Home {
      view: Rc::new(UnsafeCell::new(view))
    }
  }
}

impl Controller for Home {
  fn get_view(&self) -> &Rc<UnsafeCell<View>> {
    &self.view
  }
  
  fn on_receive_event(&self, event: Event) {
    let view = self.view.get();
    unsafe { (*view).on_receive_event(event.clone()) }
  }
}