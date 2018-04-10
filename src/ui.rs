use std::io::{self, Write};

use game::player::Command;
use game::player::Player;

#[derive(Debug)]
enum Error {
  Parse,
  Quit,
}

pub fn game_loop(mut player: Player) {
  println!("\nWelcome new adventurer!");
  loop {
    // Print a user input prompt.
    println!(
      "\nYou find yourself in {}.\n{}\n",
      player.get_room_name(),
      player.get_room_contents()
    );
    player.handle_room_events();
    if player.is_dead() {
      println!(
        "\nYou try in vain to shovel more wall chicken into \
         your mouth, but you've been impaled by too many spikes or Wumpi :("
      );
      println!("You Lose!");
      break;
    }
    print!(
      "\n{}Exits are: {}.\n\nWhat wouldst thou do?\n> ",
      player,
      player.location.borrow().neighbors_string()
    );
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
      Err(err) => {
        panic!("error: {}", err);
      }
      Ok(0) => {
        break;
      }
      Ok(_) => match parse_line(&buf) {
        Err(Error::Parse) => println!("I do not know how to {}!", buf.trim()),
        Err(Error::Quit) => break,
        Ok(cmd) => {
          if let Err(_) = player.act(cmd) {
            println!("I don't know how to {}!", buf.trim());
          }
        }
      },
    }
  }
  println!("Score: {}", player.gold * 1000);
}

fn parse_line(buf: &String) -> Result<Command, Error> {
  use game::player::Command::*;

  let tokens = buf.trim().split_whitespace();
  let mut tokens = tokens.map(|t| String::from(t).to_lowercase());

  let cmd = tokens.next().ok_or(Error::Parse)?;
  match cmd.as_ref() {
    "go" => {
      let room = tokens.next().ok_or(Error::Parse)?;
      Ok(Go(room))
    }
    "use" => {
      let curio = tokens.next().ok_or(Error::Parse)?;
      Ok(Use(curio))
    }
    "shoot" => {
      let room = tokens.next().ok_or(Error::Parse)?;
      Ok(Shoot(room))
    }
    "quit" => {
      println!("Bye for now!");
      Err(Error::Quit)
    }
    _ => Err(Error::Parse),
  }
}
