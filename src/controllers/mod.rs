use utils::navigator::EventEmitter;
use controllers::reader::Reader;
use utils::traits::Controller;
use controllers::home::Home;

use std::cell::UnsafeCell;
use std::boxed::Box;
use std::rc::Rc;

pub mod reader;
pub mod home;

pub fn get_controllers(events: &Rc<UnsafeCell<EventEmitter>>) -> Vec<Box<Controller>> {
  let reader = Reader::new(&events);
  let home = Home::new(&events);

  let controllers: Vec<Box<Controller>> = vec![
    Box::new(reader),
    Box::new(home)
  ];

  controllers
}