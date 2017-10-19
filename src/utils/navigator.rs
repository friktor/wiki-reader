use utils::traits::{ Controller, Event };
use controllers::get_controllers;

use fluent::MessageContext;
use std::cell::RefCell;
use std::ops::FnMut;
use std::rc::Rc;
use gtk;

use gtk::StackExt;

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
  pub pages: Rc<RefCell<Vec<Box<Controller>>>>,
  pub events: Rc<RefCell<EventEmitter>>,
  pub stack: gtk::Stack,
}

impl Navigator {
  pub fn new() -> Navigator {
    let events = Rc::new(RefCell::new(EventEmitter::new()));

    // Stack with options
    let content_stack: gtk::Stack = gtk::Stack::new();
    content_stack.set_transition_type(gtk::StackTransitionType::OverRight);
    content_stack.set_transition_duration(200);
    content_stack.set_homogeneous(true);

    let controllers = get_controllers(events.clone());
    let pages = Rc::new(RefCell::new(controllers));
    let stack = content_stack;

    Navigator {
      events,
      stack,
      pages,
    }
  }

  pub fn get_events(&self) -> Rc<RefCell<EventEmitter>> {
    self.events.clone()
  }

  pub fn subscribe_events(&self) {
    let events = self.events.clone();
    let stack = self.stack.clone();
    let pages = self.pages.clone();

    events.borrow_mut().subscribe(move |event| {
      match event {
        Event::OpenPage(name, _) => stack.set_visible_child_name(&*name),
        Event::GetArticle(_, _) => stack.set_visible_child_name("reader"),
        _ => {}
      } 
    });

    events.borrow_mut().subscribe(move |event| {
      for page in pages.borrow().iter() {
        page.on_receive_event(event.clone());
      }
    });
  }

  pub fn setup(&self, i18n: Rc<RefCell<MessageContext>>) {
    let stack = self.stack.clone();
    let pages = self.pages.clone();
    
    for page in pages.borrow().iter() {
      let view = page.get_view();
      view.borrow_mut().setup(i18n.clone());

      let content = view.borrow().get_content();
      let name = view.borrow().get_name();
      let title = name.clone();

      stack.add_titled(&content, &*name, &*title);
    }

    stack.set_visible_child_name("home");
    self.subscribe_events();
  }
}