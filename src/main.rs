use crate::main::*;

fn main() {
  println!("gello");
  
  let a = Fedron(5);
  let b = Fedron(10);

  println!("{}", a.0+b.0);

  let mercury = planet!(SolarID::Mercury, Monopoly::Mercury);
  println!("{:?}", mercury);

  //let board = Board::new_full_board();

  // todo: maybe Landable or Spot should be a trait and I should have a specific type for each kind of spot, rather than them being variants of an enum 
}

#[macro_export]
macro_rules! planet {
    ( $name:expr, $monopoly:expr ) => {
      Spot::Planet (Property {
        name: $name,
        monopoly: $monopoly,
      });
    };
}

#[macro_export]
macro_rules! moon {
    ( $name:expr, $monopoly:expr ) => {
      Spot::Moon (Property {
        name: $name,
        monopoly: $monopoly,
      });
    };
}

#[macro_export]
macro_rules! spaceDock {
    ( $name:expr, $monopoly:expr ) => {
      Spot::SpaceDock (Property {
        name: $name,
        monopoly: $monopoly,
      });
    };
}

#[macro_export]
macro_rules! researchLab {
    ( $name:expr, $monopoly:expr ) => {
      Spot::ResearchLab (Property {
        name: $name,
        monopoly: $monopoly,
      });
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
    Mercury,
    Venus,
    Moon,
    Pluto,
    Charon,
    Triton,
    Proteus,
    Nereid,
    Galatea,
    Despina,
    Thalassa,
    Larissa,
    Naiad,
    Enceladus,
    Mimas,
    Iapetus,
    Phoebe,
    Tethys,
    Hyperion,
    Rhea,
    Janus,
    Dione,
    Titan,
    Europa,
    Thebe,
    Himalia,
    Metis,
    Io,
    Amalthea,
    Sinope,
    Callisto,
    Adrastea,
    Ganymede,
    Elara,
    Miranda,
    Ariel,
    Umbriel,
    Portia,
    Oberon,
    Titania,
    Phobos,
    Mars,
    Deimos,
    JupiterResearchLab,
    SaturnResearchLab,
    EarthResearchLab,
    VenusResearchLab,
    NeptuneResearchLab,
    UranusResearchLab,
    JupiterSpaceDock,
    SaturnSpaceDock,
    NeptuneSpaceDock,
    SolarSpaceDock,
    UranusSpaceDock,
    EmptySpace0,
    EmptySpace1,
    EmptySpace2,
    GravityWell0,
    GravityWell1,
    GravityWell2,
    GravityWell3,
    GravityWell4,
    GravityWell5,
    GravityWell6,
    GravityWell7,
    GravityWell8,
    GravityWell9,
    GravityWell10,
    GravityWell11,
    GravityWell12,
    GravityWell13,
    GravityWell14,
    GravityWell15,
  }
  
  pub struct Board<'a> {
    board_path: BoardPath,
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

      let mut empty_space: Vec<Spot> = vec![SolarID::EmptySpace0, SolarID::EmptySpace1, SolarID::EmptySpace2]
        .into_iter()
        .map(move |id| Spot::EmptySpace { name: id })
        .collect();
      
      let mut gravity_wells: Vec<Spot> = vec![SolarID::GravityWell0, SolarID::GravityWell1, SolarID::GravityWell2]
      .into_iter()
      .map(move |id| Spot::GravityWell { name: id })
      .collect();

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
        continue_orbit: Box::new(BoardNode::Link(SolarID::Io)),
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

    // pub fn new_full_board() -> Board<'a> {

    //   let mercury = planet!(SolarID::Mercury, Monopoly::Mercury);
    //   let venus = planet!(SolarID::Venus, Monopoly::Venus);
    //   let moon = moon!(SolarID::Moon, Monopoly::Earth);
    //   let pluto = planet!(SolarID::Pluto, Monopoly::Pluto);
    //   let charon = moon!(SolarID::Charon, Monopoly::Pluto);
    //   let triton = moon!(SolarID::Triton, Monopoly::Neptune);
    //   let proteus = moon!(SolarID::Proteus, Monopoly::Neptune);
    //   let nereid = moon!(SolarID::Nereid, Monopoly::Neptune);
    //   let galatea = moon!(SolarID::Galatea, Monopoly::Neptune);
    //   let despina = moon!(SolarID::Despina, Monopoly::Neptune);
    //   let thalassa = moon!(SolarID::Thalassa, Monopoly::Neptune);
    //   let larissa = moon!(SolarID::Larissa, Monopoly::Neptune);
    //   let naiad = moon!(SolarID::Naiad, Monopoly::Neptune);
    //   let enceladus = moon!(SolarID::Enceladus, Monopoly::Saturn);
    //   let mimas = moon!(SolarID::Mimas, Monopoly::Saturn);
    //   let iapetus = moon!(SolarID::Iapetus, Monopoly::Saturn);
    //   let phoebe = moon!(SolarID::Phoebe, Monopoly::Saturn);
    //   let tethys = moon!(SolarID::Tethys, Monopoly::Saturn);
    //   let hyperion = moon!(SolarID::Hyperion, Monopoly::Saturn);
    //   let rhea = moon!(SolarID::Rhea, Monopoly::Saturn);
    //   let janus = moon!(SolarID::Janus, Monopoly::Saturn);
    //   let dione = moon!(SolarID::Dione, Monopoly::Saturn);
    //   let titan = moon!(SolarID::Titan, Monopoly::Saturn);
    //   let europa = moon!(SolarID::Europa, Monopoly::Jupiter);
    //   let thebe = moon!(SolarID::Thebe, Monopoly::Jupiter);
    //   let himalia = moon!(SolarID::Himalia, Monopoly::Jupiter);
    //   let metis = moon!(SolarID::Metis, Monopoly::Jupiter);
    //   let io = moon!(SolarID::Io, Monopoly::Jupiter);
    //   let amalthea = moon!(SolarID::Amalthea, Monopoly::Jupiter);
    //   let sinope = moon!(SolarID::Sinope, Monopoly::Jupiter);
    //   let callisto = moon!(SolarID::Callisto, Monopoly::Jupiter);
    //   let adrastea = moon!(SolarID::Adrastea, Monopoly::Jupiter);
    //   let ganymede = moon!(SolarID::Ganymede, Monopoly::Jupiter);
    //   let elara = moon!(SolarID::Elara, Monopoly::Jupiter);
    //   let miranda = moon!(SolarID::Miranda, Monopoly::Uranus);
    //   let ariel = moon!(SolarID::Ariel, Monopoly::Uranus);
    //   let umbriel = moon!(SolarID::Umbriel, Monopoly::Uranus);
    //   let portia = moon!(SolarID::Portia, Monopoly::Uranus);
    //   let oberon = moon!(SolarID::Oberon, Monopoly::Uranus);
    //   let titania = moon!(SolarID::Titania, Monopoly::Uranus);
    //   let phobos = moon!(SolarID::Phobos, Monopoly::Mars);
    //   let mars = planet!(SolarID::Mars, Monopoly::Mars);
    //   let deimos = planet!(SolarID::Deimos, Monopoly::Mars);
    //   let jupiter_research_lab = researchLab!(SolarID::JupiterResearchLab, Monopoly::ResearchLab);
    //   let saturn_research_lab = researchLab!(SolarID::SaturnResearchLab, Monopoly::ResearchLab);
    //   let earth_research_lab = researchLab!(SolarID::EarthResearchLab, Monopoly::ResearchLab);
    //   let venus_research_lab = researchLab!(SolarID::VenusResearchLab, Monopoly::ResearchLab);
    //   let neptune_research_lab = researchLab!(SolarID::NeptuneResearchLab, Monopoly::ResearchLab);
    //   let uranus_research_lab = researchLab!(SolarID::UranusResearchLab, Monopoly::ResearchLab);
    //   let jupiter_space_dock = spaceDock!(SolarID::JupiterSpaceDock, Monopoly::SpaceDock);
    //   let saturn_space_dock = spaceDock!(SolarID::SaturnSpaceDock, Monopoly::SpaceDock);
    //   let neptune_space_dock = spaceDock!(SolarID::NeptuneSpaceDock, Monopoly::SpaceDock);
    //   let solar_space_dock = spaceDock!(SolarID::SolarSpaceDock, Monopoly::SpaceDock);
    //   let uranus_space_dock = spaceDock!(SolarID::UranusSpaceDock, Monopoly::SpaceDock);

    //   let mut empty_space: Vec<Spot> = vec![
    //       SolarID::EmptySpace0,
    //       SolarID::EmptySpace1,
    //       SolarID::EmptySpace2
    //     ]
    //     .into_iter()
    //     .map(move |id| Spot::EmptySpace { name: id })
    //     .collect();
      
    //   let mut gravity_wells: Vec<Spot> = vec![
    //     SolarID::GravityWell0,
    //     SolarID::GravityWell1,
    //     SolarID::GravityWell2,
    //     SolarID::GravityWell3,
    //     SolarID::GravityWell4,
    //     SolarID::GravityWell5,
    //     SolarID::GravityWell6,
    //     SolarID::GravityWell7,
    //     SolarID::GravityWell8,
    //     SolarID::GravityWell9,
    //     SolarID::GravityWell10,
    //     SolarID::GravityWell11,
    //     SolarID::GravityWell12,
    //     SolarID::GravityWell13,
    //     SolarID::GravityWell14,
    //     SolarID::GravityWell15,
    //     ]
    //     .into_iter()
    //     .map(move |id| Spot::GravityWell { name: id })
    //     .collect();


    //   let earth_research_lab = BoardNode::PassThrough {
    //     spot: earth_research_lab,
    //     next: Box::new(BoardNode::Link(SolarID::Earth)),
    //   };
    //   let gravity_well = BoardNode::PassThrough {
    //     spot: gravity_wells.pop().unwrap(),
    //     next: Box::new(earth_research_lab),
    //   };
    //   let gravity_well = BoardNode::PassThrough {
    //     spot: gravity_wells.pop().unwrap(),
    //     next: Box::new(gravity_well),
    //   };
    //   let gravity_well = BoardNode::PassThrough {
    //     spot: gravity_wells.pop().unwrap(),
    //     next: Box::new(gravity_well),
    //   };
      
    //   let naiad = BoardNode::PassThrough {
    //     spot: naiad,
    //     next: Box::new(BoardNode::Link(SolarID::Nereid)),
    //   };

    //   let galatea = BoardNode::Fork {
    //     spot: galatea,
    //     escape_orbit: Box::new(gravity_well),
    //     continue_orbit: Box::new(naiad),
    //   };



    //   let adrastea = BoardNode::PassThrough {
    //     spot: adrastea,
    //     next: Box::new(BoardNode::Link(SolarID::Io)),
    //   };
    //   let amalthea = BoardNode::PassThrough {
    //     spot: amalthea,
    //     next: Box::new(adrastea),
    //   };
    //   let europa = BoardNode::PassThrough {
    //     spot: europa,
    //     next: Box::new(amalthea),
    //   };
    //   let himalia = BoardNode::PassThrough {
    //     spot: himalia,
    //     next: Box::new(europa),
    //   };
    //   let jupiter_research_lab = BoardNode::PassThrough {
    //     spot: jupiter_research_lab,
    //     next: Box::new(himalia),
    //   };
    //   // let ganymede = BoardNode::Fork {
    //   //   spot: ganymede,
    //   //   escape_orbit:
    //   //   sinope,
    //   //   continue_orbit: Box::new(jupiter_research_lab)
    //   // };


    


    //   let earth = Spot::Planet (Property {
    //     name: SolarID::Earth,
    //     monopoly: Monopoly::Earth,
    //   });

    //   let moon = Spot::Planet (Property {
    //     name: SolarID::Moon,
    //     monopoly: Monopoly::Earth,
    //   });
    //   // todo: constructor for Property, fill rent_table with None

    //   let io = Spot::Planet (Property {
    //     name: SolarID::Io,
    //     monopoly: Monopoly::Jupiter,
    //   });

    //   let venus = Spot::Planet (Property {
    //     name: SolarID::Venus,
    //     monopoly: Monopoly::Venus,
    //   });

      

    //   let venus_node = BoardNode::PassThrough {
    //     spot: venus,
    //     next: Box::new(BoardNode::Link(SolarID::Earth)),
    //   };
      
    //   let gw2_node = BoardNode::PassThrough {
    //     spot: gravity_wells.pop().unwrap(),
    //     next: Box::new(venus_node) 
    //   };

    //   let gw1_node = BoardNode::PassThrough {
    //     spot: gravity_wells.pop().unwrap(),
    //     next: Box::new(gw2_node) 
    //   };

    //   let gw0_node = BoardNode::PassThrough {
    //     spot: gravity_wells.pop().unwrap(),
    //     next: Box::new(gw1_node) 
    //   };

    //   let e2_node = BoardNode::Fork {
    //     spot: empty_space.pop().unwrap(),
    //     escape_orbit: Box::new(gw0_node),
    //     continue_orbit: (&BoardNode::Link(SolarID::Io)),
    //   };

    //   let e1_node = BoardNode::PassThrough {
    //     spot: empty_space.pop().unwrap(),
    //     next: Box::new(e2_node) 
    //   };

    //   let e0_node = BoardNode::PassThrough {
    //     spot: empty_space.pop().unwrap(),
    //     next: Box::new(e1_node) 
    //   };

    //   let io_node = BoardNode::Merge {
    //     spot: io,
    //     next: Box::new(e0_node) 
    //   };

    //   let moon_node = BoardNode::PassThrough {
    //     spot: moon,
    //     next: Box::new(io_node) 
    //   };
      
    //   let earth_node = BoardNode::PassThrough {
    //     spot: earth,
    //     next: Box::new(moon_node),
    //   };

    //   let board_path = BoardPath {
    //     start: earth_node
    //   };

    //   Board {
    //     board_path,
    //     players: vec![],
    //   }
    // }

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

  struct BoardPath {
    start: BoardNode,
  }

  #[derive(Debug)]
  enum BoardNode {
    PassThrough { spot: Spot, next: Box<BoardNode> },
    Fork { spot: Spot, escape_orbit: Box<BoardNode>, continue_orbit: Box<BoardNode> },
    Merge { spot: Spot, next: Box<BoardNode> },
    Link(SolarID), // shadow node to link up a cyle, tail to head. Tuple value refers to the Head to link to, but we link manually in the `Board::move()` method 
  }

  impl <'a> BoardNode {
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
    current_board_node: &'a BoardNode,
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
