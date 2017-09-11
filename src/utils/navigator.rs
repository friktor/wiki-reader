extern crate gtk;

use utils::traits::{ View, Controller, Event };
use controllers::get_controllers;

use std::cell::{ UnsafeCell, RefCell };
use std::ops::FnMut;
use std::rc::Rc;

pub struct EventEmitter {
  listeners: Vec<Rc<RefCell<FnMut(Event)>>>
}

impl EventEmitter {
  pub fn new() -> EventEmitter {
    EventEmitter { listeners: Vec::new() }
  }

  pub fn subscribe<F: FnMut(Event)+'static>(&mut self, listener: F) {
    let cell = Rc::new(RefCell::new(listener));
    self.listeners.push(cell); 
  }

  pub fn push(&mut self, event: Event) {
    for listener in self.listeners.iter() {
      let mut closure = listener.borrow_mut();
      let _e = event.clone();
      (&mut *closure)(_e);
    }
  }
}

pub struct Navigator {
  pub pages: Rc<UnsafeCell<Vec<Box<Controller>>>>,
  pub events: Rc<UnsafeCell<EventEmitter>>,
  pub stack: Rc<UnsafeCell<gtk::Stack>>,
}

impl Navigator {
  pub fn new() -> Navigator {
    use gtk::StackExt;

    let events = Rc::new(UnsafeCell::new(EventEmitter::new()));

    // Stack with options
    let content_stack: gtk::Stack = gtk::Stack::new();
    content_stack.set_transition_type(gtk::StackTransitionType::OverRight);
    content_stack.set_transition_duration(200);
    content_stack.set_homogeneous(true);

    let controllers = get_controllers(&events);
    let pages: Rc<UnsafeCell<Vec<Box<Controller>>>> = Rc::new(UnsafeCell::new(controllers));
    let stack = Rc::new(UnsafeCell::new(content_stack));

    Navigator {
      events,
      stack,
      pages,
    }
  }

  pub fn get_events(&self) -> &Rc<UnsafeCell<EventEmitter>> {
    &self.events
  }

  pub unsafe fn subscribe_events(&self) {
    use gtk::StackExt;

    let events = self.events.get();
    let stack = self.stack.get();
    let pages = self.pages.get();

    (*events).subscribe(move |event| {
      match event {
        Event::OpenPage(name) => (*stack).set_visible_child_name(&name[..]),
        Event::GetArticle(_) => (*stack).set_visible_child_name("reader"),
        _ => {}
      } 
    });

    (*events).subscribe(move |event| {
      for page in &(*pages) {
        let cloned_event = event.clone();
        page.on_receive_event(cloned_event);
      }
    });
  }

  pub unsafe fn get_page_title(&self, name: &str) -> Option<&str> {
    let pages = self.pages.get();

    for page in (*pages).iter() {
      let ref_view = page.get_view();
      let view = ref_view.get();
      let page_name = (*view).get_name();

      if name == page_name {
        let page_title = (*view).get_title();
        return Some(page_title);
      }
    }

    None
  }

  pub unsafe fn setup(&self) {
    use gtk::StackExt;

    let stack = self.stack.get();
    let pages = self.pages.get();
    
    for page in &(*pages) {
      let view = page.get_view().get();

      let content = (*view).get_content();
      let title = (*view).get_title();
      let name = (*view).get_name();

      (*stack).add_titled(content, name, title);
    }

    (*stack).set_visible_child_name("home");
    self.subscribe_events();
  }
}