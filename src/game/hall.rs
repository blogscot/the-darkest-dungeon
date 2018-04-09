use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::room::Room;

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
  pub fn other(&self, _room: &Room) -> Rc<RefCell<Room>> {
    // TODO: Implement
    unimplemented!();
  }
}

impl fmt::Debug for Hall {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let hall_left = &self.left.borrow().name;
    let hall_right = &self.right.borrow().name;
    write!(f, "[left: '{}' right: '{}']", hall_left, hall_right)
  }
}
