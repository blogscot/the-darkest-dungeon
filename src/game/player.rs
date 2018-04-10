use std;
use std::fmt;

use super::curio::Curio;
use super::room::Location;

const MAX_HP: i32 = 25;

#[derive(Debug)]
pub enum Command {
  Go(String),
  Shoot(String),
  Use(String),
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
  fn find_room(&self, room: String) -> Result<Location, ()> {
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
      let location = self.clone().location;
      let curios = &location.borrow().contents;
      for curio in curios {
        self.use_curio(curio.clone());
      }
    }
    &self.location.borrow_mut().clear_contents();
  }

  pub fn get_room_name(&mut self) -> String {
    let room_name = &self.location.borrow().name;
    room_name.clone()
  }

  pub fn get_room_contents(&mut self) -> String {
    let contents = &self.location.borrow().get_contents();
    contents.clone()
  }

  pub fn is_dead(&mut self) -> bool {
    self.hp <= 0
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
