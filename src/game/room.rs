use std::cell::RefCell;
use std::rc::Rc;

use super::curio::Curio;
use super::hall::Hall;

pub type Location = Rc<RefCell<Room>>;

#[derive(Debug, Default)]
pub struct Room {
  pub name: String,
  pub contents: Vec<Curio>,
  pub halls: Vec<Rc<Hall>>,
  pub wumpus: bool,
}

impl PartialEq for Room {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

impl Eq for Room {}

impl Room {
  pub fn new() -> Self {
    Room {
      name: "".to_string(),
      contents: Vec::new(),
      halls: Vec::new(),
      wumpus: false,
    }
  }

  pub fn neighbors_string(&self) -> String {
    let mut neighbors = self
      .halls
      .iter()
      .map(|hall| hall.right.borrow().name.clone())
      .collect::<Vec<String>>()
      .join(", ");

    if neighbors.is_empty() {
      neighbors = "None".to_string();
    }
    neighbors
  }

  pub fn get_contents(&self) -> String {
    let contents = &self.contents;
    let mut items = contents
      .into_iter()
      .map(|curio| format!("{}", curio))
      .collect::<Vec<String>>()
      .join(", ");

    if items.is_empty() {
      items = "nothing".to_string();
    }
    format!("The room contains: {}.", items)
  }

  pub fn clear_contents(&mut self) {
    self.contents = Vec::new();
  }

  pub fn shoot_wumpus(&mut self) -> (bool, String) {
    let mut message = format!("You shoot into {}...", self.name);
    let mut wumpus_is_dead = false;
    if self.wumpus {
      self.wumpus = false;
      wumpus_is_dead = true;
      message += "\nYou hear a deathly scream! You have killed the Wumpus!";
    } else {
      message += "\nYour aim is true but nothing seems to happen.";
    }
    (wumpus_is_dead, message)
  }
}
