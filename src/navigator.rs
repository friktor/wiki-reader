extern crate gdk_pixbuf;
extern crate gtk;
extern crate gdk;

use std::cell::{ UnsafeCell, RefCell };
use std::ops::FnMut;
use std::rc::Rc;

use reader::{ Reader };
use home::{ Home };

use gtk::{ Builder, Stack };
use gtk::StackExt;

pub trait Page {
  fn on_receive_event(&self, event: NavigatorEvent);
  fn get_content(&self) -> &gtk::Box;
  fn get_title(&self) -> &str;
  fn get_name(&self) -> &str;
}

#[derive(Clone)]
pub enum NavigatorEvent {
  GetArticle(String),
  OpenPage(String),
  ToggleSidebar,
}

pub struct NavigatorStateMachine {
  listeners: Vec<Rc<RefCell<FnMut(NavigatorEvent)>>>,
}

impl NavigatorStateMachine {
  fn new() -> NavigatorStateMachine {
    NavigatorStateMachine { listeners: Vec::new() }
  }

  pub fn register_listener<F: FnMut(NavigatorEvent)+'static>(&mut self, listener: F) {
    let cell = Rc::new(RefCell::new(listener));
    self.listeners.push(cell); 
  }

  pub fn push_event(&mut self, event: NavigatorEvent) {
    for listener in self.listeners.iter() {
      let mut closure = listener.borrow_mut();
      let _e = event.clone();
      (&mut *closure)(_e);
    }
  }
}

pub struct Navigator {
  events: Rc<UnsafeCell<NavigatorStateMachine>>,
  pub stack: Rc<UnsafeCell<Stack>>,
  pages: Vec<Box<Page>>,
}

impl Navigator {
  pub fn new() -> Navigator {
    let events = Rc::new(UnsafeCell::new(NavigatorStateMachine::new()));

    // Stack with options
    let stack: Stack = Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::OverRight);
    stack.set_transition_duration(200);
    stack.set_homogeneous(true);

    // Page blocks
    let reader = Reader::new(&events);
    let home = Home::new(&events);

    let pages: Vec<Box<Page>> = vec![
      Box::new(reader),
      Box::new(home),
    ];

    Navigator {
      stack: Rc::new(UnsafeCell::new(stack)),
      events,
      pages,
    }
  }

  pub fn register_listener<F: FnMut(NavigatorEvent)+'static>(&mut self, listener: F) {
    let events = self.events.get();
    unsafe { (*events).register_listener(listener) }
  }

  pub fn push_event(&mut self, event: NavigatorEvent) {
    let events = self.events.get();

    unsafe { (*events).push_event(event.clone()) }

    for page in &self.pages {
      let _e = event.clone();
      page.on_receive_event(_e);
    }
  }

  pub fn register_navigator_listener(&self) {
    let events = self.events.get();
    let stack = self.stack.get();
    
    unsafe {
      (*events).register_listener(move |event| {
        match event {
          NavigatorEvent::OpenPage(name) => {
            (*stack).set_visible_child_name(&name[..])
          },
          _ => {}
        }        
      })
    }
  }

  pub fn setup(&self) {
    let stack = self.stack.get();

    for page in &self.pages {
      let content = page.get_content();
      let title = page.get_title();
      let name = page.get_name();

      unsafe { (*stack).add_titled(content, name, title) }
    }

    unsafe { (*stack).set_visible_child_name("home") }
    self.register_navigator_listener();
  }

  pub fn open(&self, page: String) {
    let stack = self.stack.get();
    unsafe { (*stack).set_visible_child_name(&page[..]) }
  }
}