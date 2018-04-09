use std;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::curio::Curio;
use super::room::Room;

const MAX_HP: i32 = 25;

#[derive(Debug)]
pub enum Command {
  Go(String),
  Shoot(String),
  Use(String),
}

#[derive(Debug)]
pub struct Player {
  pub location: Rc<RefCell<Room>>,
  pub hp: i32,
  pub gold: i32,
  won: bool,
}

impl Player {
  pub fn new(location: Rc<RefCell<Room>>) -> Player {
    Player {
      location: location,
      hp: MAX_HP,
      gold: 0,
      won: false,
    }
  }

  pub fn use_curio(&mut self, curio: Curio) {
    match curio {
      Curio::Chest(gold) => {
        println!("You open the chest and gain {} gold.", gold);
        self.gold += gold;
      }
      Curio::SpikeTrap(dmg) => {
        println!("You take {} damage from the spikes.", dmg);
        self.hp -= dmg;
      }
      Curio::Food(heal) => {
        println!(
          "You shove a wall chicken into your gob and heal {} HP.",
          heal
        );
        self.hp = std::cmp::min(MAX_HP, self.hp + heal);
      }
      Curio::IronMaiden(sub, dmg) => {
        println!("Dude I love Iron Maiden! This one's pointy, though.");
        println!("You cut yourself on the spikes inside for {} damage.", dmg);
        self.hp -= dmg;
        println!("You open the iron maiden and...");
        self.use_curio(*sub);
      }
      Curio::FallenAdventurer(sub) => {
        println!("You pilfer the corpse and...");
        self.use_curio(*sub);
      }
    }
  }

  /// Execute the given command on the player and board state.
  pub fn act(&mut self, cmd: Command) -> Result<(), ()> {
    match cmd {
      Command::Go(room_name) => {
        if let Ok(new_room) = self.find_room(room_name) {
          self.location = new_room;
          println!("{:?}", self);
          return Ok(());
        }
        Err(())
      }
      Command::Use(curio) => {
        println!("You reach for {}, but something holds you back.", curio);
        Ok(())
      }
      Command::Shoot(room_name) => {
        println!("You shoot into the {}", room_name);
        Ok(())
      }
    }
  }

  /// Find one of the neighbors of the current room based on its name. Case insensitive.
  fn find_room(&self, room: String) -> Result<Rc<RefCell<Room>>, ()> {
    let halls = &self.location.borrow().halls;

    let mut room_found: Vec<Rc<RefCell<Room>>> = halls
      .into_iter()
      .map(|hall| hall.right.clone())
      .filter(|adjacent_room| {
        let adjacent_room_name = &adjacent_room.borrow().name;
        adjacent_room_name.to_lowercase() == room.to_lowercase()
      })
      .collect();
    room_found.pop().ok_or(())
  }
}

/**/
impl fmt::Display for Player {
  /**/
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    /**/
    write!(
      f,
      "You find yourself in {}.\nYou have {} HP and {} gold.",
      /**/ self.location.borrow().name,
      self.hp,
      self.gold
    )
    /**/
  }
  /**/
}
