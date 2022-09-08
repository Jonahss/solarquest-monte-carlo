use crate::main::*;

fn main() {
  println!("gello");
  
  let a = Fedron(5);
  let b = Fedron(10);

  println!("{}", a.0+b.0);

  // todo: maybe Landable or Spot should be a trait and I should have a specific type for each kind of spot, rather than them being variants of an enum 
}

mod main {
  use crate::main::Spot::*;
  use std::collections::HashMap;
  use std::fmt;
  use std::mem::take;

  #[derive(PartialEq, Eq, Hash, Debug)]
  pub enum SolarID {
    Earth,
    Moon,
    JupiterSpaceDock,
    Io,
    Venus,
    Ganymede,
    EmptySpace0,
    EmptySpace1,
    EmptySpace2,
    GravityWell0,
    GravityWell1,
    GravityWell2,
  }
  
  pub struct Board<'a> {
    board_path: BoardPath<'a>,
    players: Vec<PlayerCursor<'a>>,
  }

  impl <'a> Board<'a> {
    pub fn new_single_loop() -> Board<'a> {
      let earth = Spot::Planet (Property {
        name: SolarID::Earth,
        monopoly: Monopoly::Earth,
      });

      let moon = Spot::Planet (Property {
        name: SolarID::Moon,
        monopoly: Monopoly::Earth,
      });
      // todo: constructor for Property, fill rent_table with None

      let moon_node = BoardNode::PassThrough {
        spot: moon,
        next: Box::new(BoardNode::Link(SolarID::Earth)),
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

    pub fn new_nested_loop() -> Board<'a> {
      let earth = Spot::Planet (Property {
        name: SolarID::Earth,
        monopoly: Monopoly::Earth,
      });

      let moon = Spot::Planet (Property {
        name: SolarID::Moon,
        monopoly: Monopoly::Earth,
      });
      // todo: constructor for Property, fill rent_table with None

      let io = Spot::Planet (Property {
        name: SolarID::Io,
        monopoly: Monopoly::Jupiter,
      });

      let venus = Spot::Planet (Property {
        name: SolarID::Venus,
        monopoly: Monopoly::Venus,
      });

      // TODO: use `map`?
      let mut empty_space: Vec<Spot> = Vec::new();
      for i in vec![SolarID::EmptySpace0, SolarID::EmptySpace1, SolarID::EmptySpace2] {
        empty_space.push(Spot::EmptySpace { name: i });
      };

      let mut gravity_wells: Vec<Spot> = Vec::new();
      for i in vec![SolarID::EmptySpace0, SolarID::EmptySpace1, SolarID::EmptySpace2] {
        gravity_wells.push(Spot::GravityWell { name: i });
      };

      let venus_node = BoardNode::PassThrough {
        spot: venus,
        next: Box::new(BoardNode::Link(SolarID::Earth)),
      };
      
      let gw2_node = BoardNode::PassThrough {
        spot: gravity_wells.pop().unwrap(),
        next: Box::new(venus_node) 
      };

      let gw1_node = BoardNode::PassThrough {
        spot: gravity_wells.pop().unwrap(),
        next: Box::new(gw2_node) 
      };

      let gw0_node = BoardNode::PassThrough {
        spot: gravity_wells.pop().unwrap(),
        next: Box::new(gw1_node) 
      };

      let e2_node = BoardNode::Fork {
        spot: empty_space.pop().unwrap(),
        escape_orbit: Box::new(gw0_node),
        continue_orbit: (&BoardNode::Link(SolarID::Io)), //TODO need to make this IO
      };

      let e1_node = BoardNode::PassThrough {
        spot: empty_space.pop().unwrap(),
        next: Box::new(e2_node) 
      };

      let e0_node = BoardNode::PassThrough {
        spot: empty_space.pop().unwrap(),
        next: Box::new(e1_node) 
      };

      let io_node = BoardNode::Merge {
        spot: io,
        next: Box::new(e0_node) 
      };

      let moon_node = BoardNode::PassThrough {
        spot: moon,
        next: Box::new(io_node) 
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

    // Warning, this method searches by traversing the board, but only takes the escape_orbit branch of forks.
    // So some nodes can never be found this way.
    // TODO fix this
    fn find_node (&self, query: &SolarID) -> &BoardNode {
      let mut current_node = &self.board_path.start;
      loop {
        let spot = current_node.spot();
        match spot {
          Spot::EmptySpace { name } |
          Spot::GravityWell { name } |
          Spot::FederationStation { name, .. } => {
            if name == query {
              return current_node;
            };
          },
          Spot::Planet(prop) |
          Spot::Moon(prop) |
          Spot::SpaceDock(prop) |
          Spot::ResearchLab(prop) |
          Spot::Earth(prop) => {
            if &prop.name == query {
              return current_node;
            }
          }
        }
        match current_node {
          BoardNode::PassThrough { next, ..} |
          BoardNode::Merge { next, .. } => {
            current_node = next;
          },
          BoardNode::Fork { escape_orbit, .. } => current_node = &escape_orbit,
          _ => (),
        }
      };
    }

    // WARNING: this method searches by traversing the board, but only takes the escape_orbit branch of forks.
    // So some nodes can never be found this way.
    // TODO fix this
    pub fn find_spot(&self, query: &SolarID) -> &Spot {
      self.find_node(query).spot()
    }

    pub fn move_player (&'a self, player: &'a mut PlayerCursor<'a>, amount: u16) -> &'a mut PlayerCursor {
      let mut movement_remaining = amount;
      let mut current_node = player.current_board_node;
      let mut last_fork = Option::None;
      let mut moves_since_last_fork = 0;
      let mut take_fork = false;
      let mut last_non_gravity_well = player.current_board_node;
      
      while movement_remaining > 0 {
        // set last_non_gravity_well
        match current_node {
          BoardNode::PassThrough { spot, ..} |
          BoardNode::Fork { spot, .. } |
          BoardNode::Merge { spot, .. } => {
            match spot {
              Spot::GravityWell { .. } => (),
              _ => last_non_gravity_well = current_node,
            };
          },
          _ => (),
        };

        // main movement loop
        match current_node {
          BoardNode::PassThrough { next, .. } => {
            current_node = next;
            movement_remaining -= 1;
            if let Some(_) = last_fork {
              moves_since_last_fork += 1;
            };
          },
          BoardNode::Fork { escape_orbit, continue_orbit, .. } => {
            // if we haven't been instructed to take the fork, we'll follow the escape orbit
            if !take_fork {
              last_fork = Option::Some(current_node);
              current_node = &escape_orbit;
              movement_remaining -= 1;
              moves_since_last_fork = 1;
            } else {
              // we tried to escape and failed, continue our orbit
              current_node = continue_orbit;
              movement_remaining -= 1;
            }
          },
          BoardNode::Merge { next, .. } => {
            current_node = next;
            movement_remaining -= 1;
            if let Some(_) = last_fork {
              moves_since_last_fork += 1;
            };
          },
          BoardNode::Link(name) => current_node = &self.find_node(name),
        }

        // rewind if we land on a gravity well
        if movement_remaining == 0 {
          match current_node {
            BoardNode::PassThrough { spot, ..} |
            BoardNode::Fork { spot, .. } |
            BoardNode::Merge { spot, .. } => {
              if let Spot::GravityWell { .. } = spot {
                // if we passed a fork, take the other leg. If not, go to the last non-gravity-well
                match last_fork {
                  Option::Some(fork_node) => {
                    take_fork = true;
                    current_node = fork_node;
                    movement_remaining = moves_since_last_fork;
                    moves_since_last_fork = 0;
                    last_fork = Option::None;
                  },
                  Option::None => {
                    current_node = last_non_gravity_well;
                  },
                };
              };
            },
            _ => (),
          };
        };
      };

      // loop to first node, if we end on the Tail
      match current_node {
        BoardNode::Link(name) => current_node = &self.find_node(name),
        _ => (),
      };

      player.current_board_node = current_node;

      player
    }
  }

  struct BoardPath<'a> {
    start: BoardNode<'a>,
  }

  #[derive(Debug)]
  enum BoardNode<'a> {
    PassThrough { spot: Spot, next: Box<BoardNode<'a>> },
    Fork { spot: Spot, escape_orbit: Box<BoardNode<'a>>, continue_orbit: &'a BoardNode<'a> },
    Merge { spot: Spot, next: Box<BoardNode<'a>> },
    Link(SolarID), // shadow node to link up a cyle, tail to head. Tuple value refers to the Head to link to, but we link manually in the `Board::move()` method 
  }

  impl <'a> BoardNode<'a> {
    fn spot(&self) -> &Spot {
      match self {
        BoardNode::PassThrough { spot, .. } => spot,
        BoardNode::Fork { spot, .. } => spot,
        BoardNode::Merge { spot, .. } => spot,
        _ => panic!("Player piece is lost in space!"),
      }
    }
  }

  pub struct PlayerCursor<'a> {
    current_board_node: &'a BoardNode<'a>,
  }

  impl <'a> PlayerCursor<'a> {
    pub fn current_spot(&self) -> &'a Spot {
      &self.current_board_node.spot()
      // match self.current_board_node {
      //   BoardNode::PassThrough { spot, .. } => spot,
      //   BoardNode::Fork { spot, .. } => spot,
      //   BoardNode::Merge { spot, .. } => spot,
      //   _ => ("Player piece is lost in space!"), 
      // }
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
    pub name: SolarID,
    pub monopoly: Monopoly,
    //pub rent_table: [Option<(Fedron, Hydron)>; 2],
  }

  impl Property {
    // fn calculate_rent (&self, holdings: Vec<&Property>) -> Fedron {
    //   let num_properties_in_monopoly = holdings.iter().filter(|property| property.monopoly == self.monopoly).count();
    //   self.rent_table[num_properties_in_monopoly]
    //     .unwrap_or_else(|| panic!("Tried to access rent for level {} on {:?} but there are not that many, you are a fraud!", num_properties_in_monopoly, self.monopoly))
    //     .0
    // }
  }

  #[derive(Debug, Clone, Copy)]
  pub struct Fedron(pub u32);
  #[derive(Debug, Clone, Copy)]
  pub struct Hydron(pub u32);

  #[derive(Debug)]
  pub enum Spot {
    EmptySpace {name: SolarID},
    GravityWell {name: SolarID},
    FederationStation {name: SolarID, reward: Fedron},
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
      write!(f, "{:?}", spot_name)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::Board;
  use crate::SolarID;
    #[test]
    fn single_loop_board() {
      let board = Board::new_single_loop();

      let player_1 = &mut board.new_player();
      assert_eq!(player_1.current_spot().to_string(), "Earth");

      let player_1 = board.move_player(player_1, 1);
      assert_eq!(player_1.current_spot().to_string(), "Moon");

      let player_1 = board.move_player(player_1, 1);
      assert_eq!(player_1.current_spot().to_string(), "Earth");

      let player_1 = board.move_player(player_1, 8);
      assert_eq!(player_1.current_spot().to_string(), "Earth");

      let player_1 = board.move_player(player_1, 3);
      assert_eq!(player_1.current_spot().to_string(), "Moon");
    }

    #[test]
    fn find_node() {
      let board = Board::new_nested_loop();
      let io = board.find_spot(&SolarID::Io);
    }

    #[test]
    fn nested_loop_board() {
      let board = Board::new_nested_loop();

      let player_1 = &mut board.new_player();
      assert_eq!(player_1.current_spot().to_string(), "Earth");

      let player_1 = board.move_player(player_1, 1);
      assert_eq!(player_1.current_spot().to_string(), "Moon");

      let player_1 = board.move_player(player_1, 1);
      assert_eq!(player_1.current_spot().to_string(), "Io");

      let player_1 = board.move_player(player_1, 3);
      assert_eq!(player_1.current_spot().to_string(), "EmptySpace2");

      let player_1 = board.move_player(player_1, 1);
      assert_eq!(player_1.current_spot().to_string(), "Io");

      let player_1 = board.move_player(player_1, 6);
      assert_eq!(player_1.current_spot().to_string(), "EmptySpace1");

      let player_1 = board.move_player(player_1, 5);
      assert_eq!(player_1.current_spot().to_string(), "Venus");

      let player_1 = board.move_player(player_1, 6);
      assert_eq!(player_1.current_spot().to_string(), "EmptySpace2");

      let player_1 = board.move_player(player_1, 7);
      assert_eq!(player_1.current_spot().to_string(), "Io");

      let player_1 = board.move_player(player_1, 6);
      assert_eq!(player_1.current_spot().to_string(), "EmptySpace1");

      let player_1 = board.move_player(player_1, 12);
      assert_eq!(player_1.current_spot().to_string(), "Io");
    }
}
