use crate::main::*;

fn main() {
  println!("gello");
  
  let a = Fedron(5);
  let b = Fedron(10);

  println!("{}", a.0+b.0);

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
use std::mem::take;

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
    pub fn new_single_loop() -> Board<'a> {
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

    pub fn new_nested_loop() -> Board<'a> {
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

      let io = Spot::Planet (Property {
        name: String::from("Io"),
        monopoly: Monopoly::Jupiter,
        rent_table: [
          Option::Some((Fedron(100), Hydron(5))),
          Option::Some((Fedron(500), Hydron(10))),
        ],
      });

      let venus = Spot::Planet (Property {
        name: String::from("Venus"),
        monopoly: Monopoly::Venus,
        rent_table: [
          Option::Some((Fedron(100), Hydron(5))),
          Option::Some((Fedron(500), Hydron(10))),
        ],
      });

      let mut empty_space: Vec<Spot> = Vec::new();
      for i in 0..3 {
        empty_space.push(Spot::EmptySpace { name: format!("empty_space_{:?}", i)});
      };

      let mut gravity_wells: Vec<Spot> = Vec::new();
      for i in 0..3 {
        gravity_wells.push(Spot::GravityWell { name: format!("gravity_well_{:?}", i)});
      };

      let venus_node = BoardNode::PassThrough {
        spot: venus,
        next: Box::new(BoardNode::Tail),
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
        continue_orbit: (&BoardNode::E2_Tail), //TODO need to make this IO
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
          BoardNode::Tail => current_node = &self.board_path.start,
          BoardNode::E2_Tail => {
            // link up e2 to Io
            let mut io = &self.board_path.start;
            {
              let mut current_node = &self.board_path.start;
              loop {
                let spot = current_node.spot();
                if spot.to_string() == "Io" {
                  io = current_node;
                  break;
                };
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
            current_node = io;
          },
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
        BoardNode::Tail => current_node = &self.board_path.start,
        BoardNode::E2_Tail => {
          // link up e2 to Io
          let mut io = &self.board_path.start;
          {
            let mut current_node = &self.board_path.start;
            loop {
              let spot = current_node.spot();
              if spot.to_string() == "Io" {
                io = current_node;
                break;
              };
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
          current_node = io;
        },
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
    Fork { spot: Spot, escape_orbit: Box<BoardNode<'a>>, continue_orbit: &'a BoardNode<'a> },
    Merge { spot: Spot, next: Box<BoardNode<'a>> },
    Tail,
    E2_Tail,
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
  use crate::Board;
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
    fn nested_loop_board() {
      let board = Board::new_nested_loop();

      let player_1 = &mut board.new_player();
 //     assert_eq!(player_1.current_spot().to_string(), "Earth");

      let player_1 = board.move_player(player_1, 1);
 //     assert_eq!(player_1.current_spot().to_string(), "Moon");

      let player_1 = board.move_player(player_1, 1);
 //     assert_eq!(player_1.current_spot().to_string(), "Io");

      let player_1 = board.move_player(player_1, 3);
  //    assert_eq!(player_1.current_spot().to_string(), "empty_space_2");

      let player_1 = board.move_player(player_1, 1);
      assert_eq!(player_1.current_spot().to_string(), "Io");

      let player_1 = board.move_player(player_1, 6);
      assert_eq!(player_1.current_spot().to_string(), "empty_space_1");

      let player_1 = board.move_player(player_1, 5);
      assert_eq!(player_1.current_spot().to_string(), "Venus");

      let player_1 = board.move_player(player_1, 6);
      assert_eq!(player_1.current_spot().to_string(), "empty_space_2");
    }
}
