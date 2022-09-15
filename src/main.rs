use std::{collections::HashMap, convert::TryInto};

use crate::main::*;
use bracket_random::prelude::RandomNumberGenerator;

fn main() {
  let mut rng = RandomNumberGenerator::new();
  let mut thirteen_count = 0;

  let board = Board::new_full_board();

  let mut land_rate: HashMap<SolarID, u128> = HashMap::new();
  let all_solar_ids: Vec<SolarID> = vec!(
    SolarID::Earth,
    SolarID::Mercury,
    SolarID::Venus,
    SolarID::Moon,
    SolarID::Pluto,
    SolarID::Charon,
    SolarID::Triton,
    SolarID::Proteus,
    SolarID::Nereid,
    SolarID::Galatea,
    SolarID::Despina,
    SolarID::Thalassa,
    SolarID::Larissa,
    SolarID::Naiad,
    SolarID::Enceladus,
    SolarID::Mimas,
    SolarID::Iapetus,
    SolarID::Phoebe,
    SolarID::Tethys,
    SolarID::Hyperion,
    SolarID::Rhea,
    SolarID::Janus,
    SolarID::Dione,
    SolarID::Titan,
    SolarID::Europa,
    SolarID::Thebe,
    SolarID::Himalia,
    SolarID::Metis,
    SolarID::Io,
    SolarID::Amalthea,
    SolarID::Sinope,
    SolarID::Callisto,
    SolarID::Adrastea,
    SolarID::Ganymede,
    SolarID::Elara,
    SolarID::Miranda,
    SolarID::Ariel,
    SolarID::Umbriel,
    SolarID::Portia,
    SolarID::Oberon,
    SolarID::Titania,
    SolarID::Phobos,
    SolarID::Mars,
    SolarID::Deimos,
    SolarID::JupiterResearchLab,
    SolarID::SaturnResearchLab,
    SolarID::EarthResearchLab,
    SolarID::VenusResearchLab,
    SolarID::NeptuneResearchLab,
    SolarID::UranusResearchLab,
    SolarID::JupiterSpaceDock,
    SolarID::SaturnSpaceDock,
    SolarID::NeptuneSpaceDock,
    SolarID::SolarSpaceDock,
    SolarID::UranusSpaceDock,
    SolarID::FederationStationI,
    SolarID::FederationStationII,
    SolarID::FederationStationIII,
    SolarID::FederationStationIV,
    SolarID::FederationStationV,
    SolarID::FederationStationVI,
    SolarID::FederationStationVII,
    SolarID::FederationStationVIII,
    SolarID::FederationStationIX,
  );
  all_solar_ids.iter().for_each(|id| { land_rate.insert(*id, 0); });

  let num_players = 100_000;
  let rounds = 40; // about 20 rounds to get around the board once
  let mut players: Vec<PlayerCursor> = (0..num_players).map(|_| board.new_player()).collect();
  
  for turn in 0..rounds {
    for (player_num, player) in players.iter_mut().enumerate() {
      let roll = (rng.roll_str("1d6").unwrap(), rng.roll_str("1d6").unwrap());
      if roll.0 == roll.1 {
        // If you roll doubles, you can skip an oponents spot, but want to land on a property if it is unavailable.
        // for the sake of this analysis, I think we prefer to just treat it as a landing, since we're looking
        // for the most popular spots
      }
      if (roll.0 == 1 && roll.1 == 3) || (roll.0 == 3 && roll.1 == 1) {
        println!("thirteen!"); // TODO
        thirteen_count += 1;
        // It's a lot of work to code in what happens on a thirteen! The cards all do different things.
        // leaving it out for now
      }

      let roll = (roll.0 as u16 + roll.1 as u16).try_into().unwrap();
      board.move_player(player, roll);
      println!("player {} rolled {} on turn {} and landed on {}", player_num, roll, turn, player.current_spot_id);
      if player.current_spot_id.to_string().contains("EmptySpace") {
        // skip empty spaces, we don't really care about how often you land on them
        continue;
      }
      let count = land_rate.entry(player.current_spot_id).or_insert(0);
      *count += 1;
    }
  }

  let mut land_rate: Vec<(&SolarID, &u128)> = land_rate.iter().collect();
  land_rate.sort_unstable_by_key(|pair| pair.1);
  land_rate.reverse();
  println!("Simulated {} rounds for {} players. It takes about 20 rounds to get around the board once", rounds, num_players);
  land_rate.iter().for_each(|(id, count)| println!("{},{}", id, count));
  println!("thirteen was rolled {} times", thirteen_count);
}

#[macro_export]
macro_rules! planet {
    ( $name:expr, $monopoly:expr ) => {
      Spot::Planet (Property {
        name: $name,
        monopoly: $monopoly,
      })
    };
}

#[macro_export]
macro_rules! moon {
    ( $name:expr, $monopoly:expr ) => {
      Spot::Moon (Property {
        name: $name,
        monopoly: $monopoly,
      })
    };
}

#[macro_export]
macro_rules! spaceDock {
    ( $name:expr, $monopoly:expr ) => {
      Spot::SpaceDock (Property {
        name: $name,
        monopoly: $monopoly,
      })
    };
}

#[macro_export]
macro_rules! researchLab {
    ( $name:expr, $monopoly:expr ) => {
      Spot::ResearchLab (Property {
        name: $name,
        monopoly: $monopoly,
      })
    };
}

#[macro_export]
macro_rules! federationStation {
    ( $name:expr, $reward:expr ) => {
      Spot::FederationStation {
        name: $name,
        reward: $reward,
      }
    };
}

#[macro_export]
macro_rules! passthrough {
    ( $name:expr=>$to:expr ) => {
      BoardNode::PassThrough {
        spot: $name,
        next: Box::new($to),
      }
    };
}

#[macro_export]
macro_rules! merge {
    ( $name:expr=>$to:expr ) => {
      BoardNode::Merge {
        spot: $name,
        next: Box::new($to),
      }
    };
}

mod main {
  use crate::main::Spot::*;
  use std::fmt::{self, Debug};
  use std::error::Error;
  use std::fmt::Display;

  #[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
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
    FederationStationI,
    FederationStationII,
    FederationStationIII,
    FederationStationIV,
    FederationStationV,
    FederationStationVI,
    FederationStationVII,
    FederationStationVIII,
    FederationStationIX,
    EmptySpace0,
    EmptySpace1,
    EmptySpace2,
    EmptySpace3,
    EmptySpace4,
    EmptySpace5,
    EmptySpace6,
    EmptySpace7,
    EmptySpace8,
    EmptySpace9,
    EmptySpace10,
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
  
  impl fmt::Display for SolarID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
  
  pub struct Board {
    board_path: BoardPath,
  }

  impl <'a> Board {
    #[cfg(test)]
    pub fn new_single_loop() -> Board {
      let earth = Spot::Planet (Property {
        name: SolarID::Earth,
        monopoly: Monopoly::Earth,
      });

      let moon = Spot::Planet (Property {
        name: SolarID::Moon,
        monopoly: Monopoly::Earth,
      });
      // todo: constructor for Property, fill rent_table with None

      let prev = passthrough!(moon=>BoardNode::Link(SolarID::Earth));
      let earth_node = passthrough!(earth=>prev);
 
      let board_path = BoardPath {
        start: earth_node
      };

      Board {
        board_path,
      }
    }
    #[cfg(test)]
    pub fn new_nested_loop() -> Board {
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
      }
    }

    pub fn new_full_board() -> Board {

      let earth = Spot::Earth { name: SolarID::Earth, reward: Fedron(1000), passing_reward: Fedron(500) };
      let mercury = planet!(SolarID::Mercury, Monopoly::Mercury);
      let venus = planet!(SolarID::Venus, Monopoly::Venus);
      let moon = moon!(SolarID::Moon, Monopoly::Earth);
      let pluto = planet!(SolarID::Pluto, Monopoly::Pluto);
      let charon = moon!(SolarID::Charon, Monopoly::Pluto);
      let triton = moon!(SolarID::Triton, Monopoly::Neptune);
      let proteus = moon!(SolarID::Proteus, Monopoly::Neptune);
      let nereid = moon!(SolarID::Nereid, Monopoly::Neptune);
      let galatea = moon!(SolarID::Galatea, Monopoly::Neptune);
      let despina = moon!(SolarID::Despina, Monopoly::Neptune);
      let thalassa = moon!(SolarID::Thalassa, Monopoly::Neptune);
      let larissa = moon!(SolarID::Larissa, Monopoly::Neptune);
      let naiad = moon!(SolarID::Naiad, Monopoly::Neptune);
      let enceladus = moon!(SolarID::Enceladus, Monopoly::Saturn);
      let mimas = moon!(SolarID::Mimas, Monopoly::Saturn);
      let iapetus = moon!(SolarID::Iapetus, Monopoly::Saturn);
      let phoebe = moon!(SolarID::Phoebe, Monopoly::Saturn);
      let tethys = moon!(SolarID::Tethys, Monopoly::Saturn);
      let hyperion = moon!(SolarID::Hyperion, Monopoly::Saturn);
      let rhea = moon!(SolarID::Rhea, Monopoly::Saturn);
      let janus = moon!(SolarID::Janus, Monopoly::Saturn);
      let dione = moon!(SolarID::Dione, Monopoly::Saturn);
      let titan = moon!(SolarID::Titan, Monopoly::Saturn);
      let europa = moon!(SolarID::Europa, Monopoly::Jupiter);
      let thebe = moon!(SolarID::Thebe, Monopoly::Jupiter);
      let himalia = moon!(SolarID::Himalia, Monopoly::Jupiter);
      let metis = moon!(SolarID::Metis, Monopoly::Jupiter);
      let io = moon!(SolarID::Io, Monopoly::Jupiter);
      let amalthea = moon!(SolarID::Amalthea, Monopoly::Jupiter);
      let sinope = moon!(SolarID::Sinope, Monopoly::Jupiter);
      let callisto = moon!(SolarID::Callisto, Monopoly::Jupiter);
      let adrastea = moon!(SolarID::Adrastea, Monopoly::Jupiter);
      let ganymede = moon!(SolarID::Ganymede, Monopoly::Jupiter);
      let elara = moon!(SolarID::Elara, Monopoly::Jupiter);
      let miranda = moon!(SolarID::Miranda, Monopoly::Uranus);
      let ariel = moon!(SolarID::Ariel, Monopoly::Uranus);
      let umbriel = moon!(SolarID::Umbriel, Monopoly::Uranus);
      let portia = moon!(SolarID::Portia, Monopoly::Uranus);
      let oberon = moon!(SolarID::Oberon, Monopoly::Uranus);
      let titania = moon!(SolarID::Titania, Monopoly::Uranus);
      let phobos = moon!(SolarID::Phobos, Monopoly::Mars);
      let mars = planet!(SolarID::Mars, Monopoly::Mars);
      let deimos = planet!(SolarID::Deimos, Monopoly::Mars);
      let jupiter_research_lab = researchLab!(SolarID::JupiterResearchLab, Monopoly::ResearchLab);
      let saturn_research_lab = researchLab!(SolarID::SaturnResearchLab, Monopoly::ResearchLab);
      let earth_research_lab = researchLab!(SolarID::EarthResearchLab, Monopoly::ResearchLab);
      let venus_research_lab = researchLab!(SolarID::VenusResearchLab, Monopoly::ResearchLab);
      let neptune_research_lab = researchLab!(SolarID::NeptuneResearchLab, Monopoly::ResearchLab);
      let uranus_research_lab = researchLab!(SolarID::UranusResearchLab, Monopoly::ResearchLab);
      let jupiter_space_dock = spaceDock!(SolarID::JupiterSpaceDock, Monopoly::SpaceDock);
      let saturn_space_dock = spaceDock!(SolarID::SaturnSpaceDock, Monopoly::SpaceDock);
      let neptune_space_dock = spaceDock!(SolarID::NeptuneSpaceDock, Monopoly::SpaceDock);
      let solar_space_dock = spaceDock!(SolarID::SolarSpaceDock, Monopoly::SpaceDock);
      let uranus_space_dock = spaceDock!(SolarID::UranusSpaceDock, Monopoly::SpaceDock);
      let federation_station_i = federationStation!(SolarID::FederationStationI, Fedron(450));
      let federation_station_ii = federationStation!(SolarID::FederationStationII, Fedron(800));
      let federation_station_iii = federationStation!(SolarID::FederationStationIII, Fedron(600));
      let federation_station_iv = federationStation!(SolarID::FederationStationIV, Fedron(500));
      let federation_station_v = federationStation!(SolarID::FederationStationV, Fedron(1000));
      let federation_station_vi = federationStation!(SolarID::FederationStationVI, Fedron(400));
      let federation_station_vii = federationStation!(SolarID::FederationStationVII, Fedron(700));
      let federation_station_viii = federationStation!(SolarID::FederationStationVIII, Fedron(500));
      let federation_station_ix = federationStation!(SolarID::FederationStationIX, Fedron(300));

      let mut empty_space: Vec<Spot> = vec![
          SolarID::EmptySpace0,
          SolarID::EmptySpace1,
          SolarID::EmptySpace2,
          SolarID::EmptySpace3,
          SolarID::EmptySpace4,
          SolarID::EmptySpace5,
          SolarID::EmptySpace6,
          SolarID::EmptySpace7,
          SolarID::EmptySpace8,
          SolarID::EmptySpace9,
          SolarID::EmptySpace10,
        ]
        .into_iter()
        .map(move |id| Spot::EmptySpace { name: id })
        .collect();
      
      let mut next_empty_space_passthrough_node = move |next:BoardNode| {
        BoardNode::PassThrough {
          spot: empty_space.pop().unwrap(),
          next: Box::new(next),
        }
      };
      
      let mut gravity_wells: Vec<Spot> = vec![
        SolarID::GravityWell0,
        SolarID::GravityWell1,
        SolarID::GravityWell2,
        SolarID::GravityWell3,
        SolarID::GravityWell4,
        SolarID::GravityWell5,
        SolarID::GravityWell6,
        SolarID::GravityWell7,
        SolarID::GravityWell8,
        SolarID::GravityWell9,
        SolarID::GravityWell10,
        SolarID::GravityWell11,
        SolarID::GravityWell12,
        SolarID::GravityWell13,
        SolarID::GravityWell14,
        SolarID::GravityWell15,
        ]
        .into_iter()
        .map(move |id| Spot::GravityWell { name: id })
        .collect();

      let mut next_gravity_well_passthrough_node = move |next:BoardNode| {
        BoardNode::PassThrough {
          spot: gravity_wells.pop().unwrap(),
          next: Box::new(next),
        }
      };

      let earth_research_lab = BoardNode::PassThrough {
        spot: earth_research_lab,
        next: Box::new(BoardNode::Link(SolarID::Earth)),
      };
      let prev = next_gravity_well_passthrough_node(earth_research_lab);
      let prev = next_gravity_well_passthrough_node(prev);
      let gravity_well = next_gravity_well_passthrough_node(prev);
      
      let naiad = BoardNode::PassThrough {
        spot: naiad,
        next: Box::new(BoardNode::Link(SolarID::Nereid)),
      };

      let galatea = BoardNode::Fork {
        spot: galatea,
        escape_orbit: Box::new(gravity_well),
        continue_orbit: Box::new(naiad),
      };

      let federation_station_ix = BoardNode::PassThrough {
        spot: federation_station_ix,
        next: Box::new(galatea),
      };
      let triton = BoardNode::PassThrough {
        spot: triton,
        next: Box::new(federation_station_ix),
      };
      let despina = BoardNode::PassThrough {
        spot: despina,
        next: Box::new(triton),
      };
      let prev = passthrough!(neptune_research_lab=>despina);
      let prev = passthrough!(thalassa=>prev);
      let prev = passthrough!(neptune_space_dock=>prev);
      let prev = passthrough!(larissa=>prev);
      let prev = merge!(nereid=>prev);
      let prev = passthrough!(proteus=>prev);
      let prev = passthrough!(federation_station_viii=>prev);
      let prev = next_gravity_well_passthrough_node(prev);
      let prev = next_gravity_well_passthrough_node(prev);
      let prev = next_gravity_well_passthrough_node(prev);
      let gravity_well = next_gravity_well_passthrough_node(prev);

      let prev = passthrough!(federation_station_vii=>BoardNode::Link(SolarID::Mimas));
      let titan = passthrough!(titan=>prev);

      let prev = BoardNode::Fork {
        spot: hyperion,
        escape_orbit: Box::new(gravity_well),
        continue_orbit: Box::new(titan),
      };
      let prev = passthrough!(saturn_research_lab=>prev);
      let prev = passthrough!(enceladus=>prev);
      let prev = passthrough!(janus=>prev);
      let prev = passthrough!(federation_station_vi=>prev);
      let prev = passthrough!(tethys=>prev);
      let prev = passthrough!(rhea=>prev);
      let prev = passthrough!(iapetus=>prev);
      let prev = next_empty_space_passthrough_node(prev);
      let prev = passthrough!(dione=>prev);
      let prev = passthrough!(saturn_space_dock=>prev);
      let prev = merge!(mimas=>prev);

      let prev = passthrough!(phoebe=>prev);
      let prev = next_gravity_well_passthrough_node(prev);
      let prev = passthrough!(venus=>prev);
      let prev = passthrough!(venus_research_lab=>prev);
      let prev = next_empty_space_passthrough_node(prev);
      let prev = passthrough!(deimos=>prev);
      let prev = passthrough!(mars=>prev);
      let prev = passthrough!(phobos=>prev);
      let prev = next_empty_space_passthrough_node(prev);
      let prev = next_empty_space_passthrough_node(prev);
      let prev = passthrough!(federation_station_v=>prev);
      let prev = next_empty_space_passthrough_node(prev);
      let prev = passthrough!(charon=>prev);
      let prev = passthrough!(pluto=>prev);
      let prev = next_gravity_well_passthrough_node(prev);
      let prev = next_gravity_well_passthrough_node(prev);
      let gravity_well = next_gravity_well_passthrough_node(prev);

      let prev = passthrough!(titania=>BoardNode::Link(SolarID::Miranda));
      let federation_station_iv = passthrough!(federation_station_iv=>prev);

      let prev = BoardNode::Fork {
        spot: ariel,
        escape_orbit: Box::new(gravity_well),
        continue_orbit: Box::new(federation_station_iv),
      };
      let prev = next_empty_space_passthrough_node(prev);
      let prev = passthrough!(uranus_research_lab=>prev);
      let prev = passthrough!(portia=>prev);
      let prev = passthrough!(oberon=>prev);
      let prev = passthrough!(umbriel=>prev);
      let prev = passthrough!(uranus_space_dock=>prev);
      let prev = merge!(miranda=>prev);

      let prev = next_empty_space_passthrough_node(prev);
      let prev = passthrough!(federation_station_iii=>prev);
      let prev = next_gravity_well_passthrough_node(prev);
      let prev = next_gravity_well_passthrough_node(prev);
      let prev = next_gravity_well_passthrough_node(prev);
      let prev = next_gravity_well_passthrough_node(prev);

      let sinope = passthrough!(sinope=>prev);

      let prev = passthrough!(adrastea=>BoardNode::Link(SolarID::Io));
      let prev = passthrough!(amalthea=>prev);
      let prev = passthrough!(europa=>prev);
      let prev = passthrough!(himalia=>prev);
      let jupiter_research_lab = passthrough!(jupiter_research_lab=>prev);

      let prev = BoardNode::Fork {
        spot: ganymede,
        escape_orbit: Box::new(sinope),
        continue_orbit: Box::new(jupiter_research_lab),
      };
      let prev = passthrough!(metis=>prev);
      let prev = next_empty_space_passthrough_node(prev);
      let prev = passthrough!(thebe=>prev);
      let prev = passthrough!(federation_station_ii=>prev);
      let prev = passthrough!(callisto=>prev);
      let prev = passthrough!(elara=>prev);
      let prev = next_empty_space_passthrough_node(prev);
      let prev = passthrough!(jupiter_space_dock=>prev);
      let prev = merge!(io=>prev);

      let prev = next_empty_space_passthrough_node(prev);
      let prev = next_empty_space_passthrough_node(prev);
      let prev = merge!(federation_station_i=>prev);
      let prev = merge!(mercury=>prev);
      let prev = merge!(solar_space_dock=>prev);
      let prev = merge!(moon=>prev);
      let prev = next_gravity_well_passthrough_node(prev);
      let earth = merge!(earth=>prev);

      let board_path = BoardPath {
        start: earth
      };

      Board {
        board_path,
      }
    }

    pub fn new_player(&'a self) -> PlayerCursor {
      PlayerCursor { current_spot_id: SolarID::Earth }
    }

    fn find_node (&self, query: &SolarID) -> Result<&BoardNode, Box<dyn Error>> {

      fn recursive_find<'a>(start: &'a BoardNode, query: &SolarID) -> Result<&'a BoardNode, Box<dyn Error>> {
        let mut current_node = start;
        loop {
          if let BoardNode::Link(_) = current_node {
            return Err(Box::new(NotFoundError { looking_for: query.to_owned() }));
          };
          let spot = current_node.spot();
          match spot {
            Spot::EmptySpace { name } |
            Spot::GravityWell { name } |
            Spot::FederationStation { name, .. } |
            Spot::Earth { name, .. }=> {
              if name == query {
                return Ok(current_node);
              };
            },
            Spot::Planet(prop) |
            Spot::Moon(prop) |
            Spot::SpaceDock(prop) |
            Spot::ResearchLab(prop) => {
              if &prop.name == query {
                return Ok(current_node);
              }
            }
          }
          match current_node {
            BoardNode::PassThrough { next, ..} |
            BoardNode::Merge { next, .. } => {
              current_node = next;
            },
            BoardNode::Fork { escape_orbit, continue_orbit, .. } => {
              return {
                recursive_find(continue_orbit, query)
                              .or(recursive_find(escape_orbit, query))
              }
            }
            BoardNode::Link(_) => return Err(Box::new(NotFoundError { looking_for: query.to_owned() })),
          }
        };
      }

      let start = &self.board_path.start;
      recursive_find(start, query)
    }

    #[allow(dead_code)]
    pub fn find_spot(&self, query: &SolarID) -> &Spot {
      self.find_node(query).unwrap().spot()
    }

    pub fn move_player (&'a self, player: &'a mut PlayerCursor, amount: u16) {
      let mut movement_remaining = amount;
      let mut current_node = self.find_node(&player.current_spot_id).unwrap();
      let mut last_fork = Option::None;
      let mut moves_since_last_fork = 0;
      let mut take_fork = false;
      let mut last_non_gravity_well = current_node;
      
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
          BoardNode::Link(name) => current_node = &self.find_node(name).unwrap(),
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
        BoardNode::Link(name) => current_node = &self.find_node(name).unwrap(),
        _ => (),
      };

      player.current_spot_id = current_node.spot().id();
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

  pub struct PlayerCursor {
    pub current_spot_id: SolarID,
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
    Earth {name: SolarID, reward: Fedron, passing_reward: Fedron}, // todo: use it
  }

  impl Spot {
    pub fn id(&self) -> SolarID {
      match &self {
        EmptySpace { name } => *name,
        GravityWell { name } => *name,
        FederationStation { name, .. } => *name,
        Planet(prop) => prop.name,
        Moon(prop) => prop.name,
        SpaceDock(prop) => prop.name,
        ResearchLab(prop) => prop.name,
        Earth { name, .. } => *name,
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
        Earth { name, .. } => name,
      };
      write!(f, "{:?}", spot_name)
    }
  }

  #[derive(Debug)]
struct NotFoundError<T: Display> {
  looking_for: T,
}

impl<T: Display + Debug> Error for NotFoundError<T> {}
impl<T: Display> fmt::Display for NotFoundError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find: {}", self.looking_for)
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
      assert_eq!(player_1.current_spot_id, SolarID::Earth);

      board.move_player(player_1, 1);
      assert_eq!(player_1.current_spot_id, SolarID::Moon);

      board.move_player(player_1, 1);
      assert_eq!(player_1.current_spot_id, SolarID::Earth);

      board.move_player(player_1, 8);
      assert_eq!(player_1.current_spot_id, SolarID::Earth);

      board.move_player(player_1, 3);
      assert_eq!(player_1.current_spot_id, SolarID::Moon);
    }

    #[test]
    fn find_node() {
      let board = Board::new_nested_loop();
      let io = board.find_spot(&SolarID::Io);
      assert_eq!(io.id(), SolarID::Io);
    }

    #[test]
    fn nested_loop_board() {
      let board = Board::new_nested_loop();

      let player_1 = &mut board.new_player();
      assert_eq!(player_1.current_spot_id, SolarID::Earth);

      board.move_player(player_1, 1);
      assert_eq!(player_1.current_spot_id, SolarID::Moon);

      board.move_player(player_1, 1);
      assert_eq!(player_1.current_spot_id, SolarID::Io);

      board.move_player(player_1, 3);
      assert_eq!(player_1.current_spot_id, SolarID::EmptySpace2);

      board.move_player(player_1, 1);
      assert_eq!(player_1.current_spot_id, SolarID::Io);

      board.move_player(player_1, 6);
      assert_eq!(player_1.current_spot_id, SolarID::EmptySpace1);

      board.move_player(player_1, 5);
      assert_eq!(player_1.current_spot_id, SolarID::Venus);

      board.move_player(player_1, 6);
      assert_eq!(player_1.current_spot_id, SolarID::EmptySpace2);

      board.move_player(player_1, 7);
      assert_eq!(player_1.current_spot_id, SolarID::Io);

      board.move_player(player_1, 6);
      assert_eq!(player_1.current_spot_id, SolarID::EmptySpace1);

      board.move_player(player_1, 12);
      assert_eq!(player_1.current_spot_id, SolarID::Io);
    }

    #[test]
    fn full_board() {
      let board = Board::new_full_board();

      let player_1 = &mut board.new_player();
      assert_eq!(player_1.current_spot_id, SolarID::Earth);

      board.move_player(player_1, 1);
      assert_eq!(player_1.current_spot_id, SolarID::Earth);

      board.move_player(player_1, 2);
      assert_eq!(player_1.current_spot_id, SolarID::Moon);

      board.move_player(player_1, 3);
      assert_eq!(player_1.current_spot_id, SolarID::FederationStationI);

      board.move_player(player_1, 1);
      assert_eq!(player_1.current_spot_id, SolarID::EmptySpace0);

      board.move_player(player_1, 6);
      assert_eq!(player_1.current_spot_id, SolarID::Callisto);

      board.move_player(player_1, 12);
      assert_eq!(player_1.current_spot_id, SolarID::EmptySpace4);

      board.move_player(player_1, 9);
      assert_eq!(player_1.current_spot_id, SolarID::FederationStationIV);

      board.move_player(player_1, 23);
      assert_eq!(player_1.current_spot_id, SolarID::VenusResearchLab);

      board.move_player(player_1, 31);
      assert_eq!(player_1.current_spot_id, SolarID::Naiad);

      board.move_player(player_1, 16);
      assert_eq!(player_1.current_spot_id, SolarID::Moon);
    }
}
