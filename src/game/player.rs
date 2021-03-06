use std;
use std::fmt;

use super::curio::Curio;
use super::room::Location;

const MAX_HP: i32 = 25;

#[derive(Debug)]
pub enum Command {
  Go(String),
  Help,
  Shoot(String),
}

#[derive(Clone, Debug)]
pub struct Player {
  pub location: Location,
  pub hp: i32,
  pub gold: i32,
  won: bool,
}

impl Player {
  pub fn new(location: Location) -> Player {
    Player {
      location,
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
      Command::Help => {
        println!("Your only commands are: Go, and Shoot. Sad, I know.");
        Ok(())
      }
      Command::Go(room_name) => self
        .find_room(room_name.as_str())
        .map(|location| self.location = location),
      Command::Shoot(room_name) => self.find_room(room_name.as_str()).map(|location| {
        let (wumpus_is_dead, message) = location.borrow_mut().shoot_wumpus();
        println!("{}", message);
        if wumpus_is_dead {
          self.won = true;
        }
      }),
    }
  }

  /// Find one of the neighbors of the current room based on its name. Case insensitive.
  fn find_room(&self, room: &str) -> Result<Location, ()> {
    let halls = &self.location.borrow().halls;

    let mut room_found: Vec<Location> = halls
      .into_iter()
      .map(|hall| hall.right.clone())
      .filter(|adjacent_room| {
        let adjacent_room_name = &adjacent_room.borrow().name;
        adjacent_room_name.to_lowercase() == room.to_lowercase()
      })
      .collect();
    room_found.pop().ok_or(())
  }

  pub fn handle_room_events(&mut self) {
    {
      // Use all rooms curios
      let location = self.clone().location;
      let curios = &location.borrow().contents;
      for curio in curios {
        self.use_curio(curio.clone());
      }
      // Is Wumpus present?
      if location.borrow().wumpus {
        println!("As you enter the room the Wumpus attacks you without warning.");
        self.hp = 0;
      }
    }
    self.location.borrow_mut().clear_contents();
  }

  pub fn get_room_name(&mut self) -> String {
    let room_name = &self.location.borrow().name;
    room_name.clone()
  }

  pub fn get_room_contents(&mut self) -> String {
    let contents = &self.location.borrow().get_contents();
    contents.clone()
  }

  pub fn is_dead(&self) -> bool {
    self.hp <= 0
  }

  pub fn has_won(&self) -> bool {
    self.won
  }
}

/**/
impl fmt::Display for Player {
  /**/
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    /**/
    write!(f, "You have {} HP and {} gold.\n", self.hp, self.gold)
    /**/
  }
  /**/
}
