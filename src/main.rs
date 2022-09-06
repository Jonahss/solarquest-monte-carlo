use crate::main::*;

fn main() {
  println!("gello");
  
  let a = Fedron(5);
  let b = Fedron(10);

  println!("{}", a.0+b.0);

  let board = Board::new();

  //println!("{:?}", board.get_spot(&SolarID::Earth));

  let player_1 = &mut board.new_player();
  println!("Player 1 starts at spot: {}", player_1.current_spot());

  let player_1 = board.move_player(player_1, 1);
  println!("Player 1 at spot: {}", player_1.current_spot());

  let player_1 = board.move_player(player_1, 1);
  println!("Player 1 at spot: {}", player_1.current_spot());

  let player_1 = board.move_player(player_1, 8);
  println!("Player 1 at spot: {}", player_1.current_spot());

  let player_1 = board.move_player(player_1, 3);
  println!("Player 1 at spot: {}", player_1.current_spot());

  
  // todo: maybe Landable or Spot should be a trait and I should have a specific type for each kind of spot, rather than them being variants of an enum 
  let mut empty_space: Vec<Spot> = Vec::new();
  for i in [0..5] {
    empty_space.push(Spot::EmptySpace { name: format!("empty_space_{:?}", i)});
  };
  
}

mod main {
  use crate::main::Spot::*;
  use std::collections::HashMap;
  use std::fmt;

  #[derive(PartialEq, Eq, Hash, Debug)]
  pub enum SolarID {
    Earth,
    Moon,
    JupiterSpaceDock,
    IO,
    Ganymede,
    EmptySpace0,
    GravityWell0,
  }
  
  pub struct Board<'a> {
    board_path: BoardPath<'a>,
    players: Vec<PlayerCursor<'a>>,
  }

  impl <'a> Board<'a> {
    pub fn new() -> Board<'a> {
      let earth = Spot::Planet (Property {
        name: String::from("Earth"),
        monopoly: Monopoly::Earth,
        rent_table: [
          Option::Some((Fedron(100), Hydron(5))),
          Option::Some((Fedron(500), Hydron(10))),
        ],
      });

      let moon = Spot::Planet (Property {
        name: String::from("Moon"),
        monopoly: Monopoly::Earth,
        rent_table: [
          Option::Some((Fedron(100), Hydron(5))),
          Option::Some((Fedron(500), Hydron(10))),
        ],
      });
      // todo: constructor for Property, fill rent_table with None

      let moon_node = BoardNode::PassThrough {
        spot: moon,
        next: Box::new(BoardNode::Tail),
      };
      let earth_node = BoardNode::PassThrough {
        spot: earth,
        next: Box::new(moon_node),
      };

      let board_path = BoardPath {
        start: earth_node
      };

      Board {
        board_path,
        players: vec![],
      }
    }

    pub fn new_player(&'a self) -> PlayerCursor<'a> {
      PlayerCursor { current_board_node: &self.board_path.start }
    }

    // TODO
    // pub fn get_spot<'b> (&'b mut self, name: &SolarID) -> &Spot {
    //   let spot = self.spots.get(name);
    //   match spot {
    //     Some(spot) => spot,
    //     None => {
    //       panic!("No spot was created for SolarID {:?}", name);
    //     }
    //   }
    // }

    pub fn move_player (&'a self, player: &'a mut PlayerCursor<'a>, amount: u16) -> &'a mut PlayerCursor {
      let mut movement_remaining = amount;
      let mut current_node = player.current_board_node;
      while movement_remaining > 0 {
        match current_node {
          BoardNode::PassThrough { next, .. } => {
            current_node = next;
            movement_remaining -= 1;
          },
          BoardNode::Fork { .. } => panic!("todo"),
          BoardNode::Merge { .. } => panic!("todo"),
          BoardNode::Tail => current_node = &self.board_path.start,
        }
      };

      match current_node {
        BoardNode::Tail => current_node = &self.board_path.start,
        _ => (),
      };

      player.current_board_node = current_node;

      player
    }
  }

  struct BoardPath<'a> {
    start: BoardNode<'a>,
  }

  enum BoardNode<'a> {
    PassThrough { spot: Spot, next: Box<BoardNode<'a>> },
    Fork { spot: Spot, escape_orbit: Box<BoardNode<
          'a>>, continue_orbit: &'a BoardNode<'a> },
    Merge { spot: Spot, next: Box<BoardNode<'a>> },
    Tail,
  }

  impl <'a> BoardNode<'a> {
    fn spot(&self) -> &Spot {
      match self {
        BoardNode::PassThrough { spot, .. } => spot,
        BoardNode::Fork { spot, .. } => spot,
        BoardNode::Merge { spot, .. } => spot,
        BoardNode::Tail => panic!("Player piece is lost in space!"),
      }
    }
  }

  pub struct PlayerCursor<'a> {
    current_board_node: &'a BoardNode<'a>,
  }

  impl <'a> PlayerCursor<'a> {
    pub fn current_spot(&self) -> &'a Spot {
      match self.current_board_node {
        BoardNode::PassThrough { spot, .. } => spot,
        BoardNode::Fork { spot, .. } => spot,
        BoardNode::Merge { spot, .. } => spot,
        BoardNode::Tail => panic!("Player piece is lost in space!"), 
      }
    }
  }

  #[derive(PartialEq, Debug)]
  pub enum Monopoly {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto, // Game predates Pluto being reclassified!
    SpaceDock,
    ResearchLab,
  }

  #[derive(Debug)]
  pub struct Property {
    pub name: String,
    pub monopoly: Monopoly,
    pub rent_table: [Option<(Fedron, Hydron)>; 2],
  }

  impl Property {
    fn calculate_rent (&self, holdings: Vec<&Property>) -> Fedron {
      let num_properties_in_monopoly = holdings.iter().filter(|property| property.monopoly == self.monopoly).count();
      self.rent_table[num_properties_in_monopoly]
        .unwrap_or_else(|| panic!("Tried to access rent for level {} on {:?} but there are not that many, you are a fraud!", num_properties_in_monopoly, self.monopoly))
        .0
    }
  }

  #[derive(Debug, Clone, Copy)]
  pub struct Fedron(pub u32);
  #[derive(Debug, Clone, Copy)]
  pub struct Hydron(pub u32);

  #[derive(Debug)]
  pub enum Spot {
    EmptySpace {name: String},
    GravityWell {name: String},
    FederationStation {name: String, reward: Fedron},
    Planet(Property),
    Moon(Property),
    SpaceDock(Property),
    ResearchLab(Property),
    Earth(Property), // todo: use it
  }
  
  impl Spot {
    fn landable(&self) -> bool {
      match self {
        GravityWell {..} => false,
        _ => true,
      }
    }
  }
  impl fmt::Display for Spot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let spot_name = match &self {
        EmptySpace { name } => name,
        GravityWell { name } => name,
        FederationStation { name, .. } => name,
        Planet(prop) => &prop.name,
        Moon(prop) => &prop.name,
        SpaceDock(prop) => &prop.name,
        ResearchLab(prop) => &prop.name,
        Earth(prop) => &prop.name,
      };
      write!(f, "{}", spot_name)
    }
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}