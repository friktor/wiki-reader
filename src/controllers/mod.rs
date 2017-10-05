use utils::navigator::EventEmitter;
use controllers::reader::Reader;
use utils::traits::Controller;
use controllers::about::About;
use controllers::home::Home;

use std::cell::RefCell;
use std::boxed::Box;
use std::rc::Rc;

pub mod reader;
pub mod about;
pub mod home;

pub fn get_controllers(events: Rc<RefCell<EventEmitter>>) -> Vec<Box<Controller>> {
  let reader = Reader::new(events.clone());
  let about = About::new(events.clone());
  let home = Home::new(events.clone());

  let controllers: Vec<Box<Controller>> = vec![
    Box::new(reader),
    Box::new(about),
    Box::new(home),
  ];

  controllers
}