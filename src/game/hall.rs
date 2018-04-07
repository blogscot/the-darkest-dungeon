use std::cell::RefCell;
use std::rc::Rc;

use super::room::Room;

#[derive(Debug)]
pub struct Hall {
  pub left: Rc<RefCell<Room>>,
  pub right: Rc<RefCell<Room>>,
}

impl Hall {
  pub fn new() -> Self {
    Hall {
      left: Rc::new(RefCell::new(Room::new())),
      right: Rc::new(RefCell::new(Room::new())),
    }
  }

  /// Given a Room `room`, find the room at the other end of Hall `self`.
  pub fn other(&self, room: &Room) -> Rc<RefCell<Room>> {
    // TODO: Implement
    unimplemented!();
  }
}
