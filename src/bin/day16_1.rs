use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct ValveKey([u8; 2]);

#[derive(Debug)]
struct Valve {
    key: ValveKey,
    tunnels: Vec<ValveKey>,
    rate: u64,
}

#[derive(Debug)]
struct Network {
    valves: HashMap<ValveKey, Valve>,
}

#[derive(Debug)]
struct Move {
    target: ValveKey,
    reward: u64,
    path: Vec<ValveKey>,
}

#[derive(Debug, Clone)]
struct State<'a> {
    max_time: u64,
    network: &'a Network,
    current_position: ValveKey,
    current_time: u64,
    opened_valves: HashSet<ValveKey>,
    pressure: u64,
}

impl Debug for ValveKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let [a, b] = self.0;
        write!(f, "{}{}", a as char, b as char)
    }
}

impl Display for ValveKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl FromStr for ValveKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.as_bytes().try_into().unwrap()))
    }
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (valve_info, tunnel_info) = s.split_once("; ").unwrap();

        let stripped_valve_info = valve_info
            .replace("Valve ", "")
            .replace(" has flow rate", "");

        let stripped_tunnel_info = tunnel_info
            .replace("tunnels lead to valves ", "")
            .replace("tunnel leads to valve ", "")
            .replace("valve ", "");

        let (key_str, rate) = stripped_valve_info.split_once("=").unwrap();

        Ok(Self {
            key: key_str.parse::<ValveKey>().unwrap(),
            rate: rate.parse::<u64>().unwrap(),
            tunnels: stripped_tunnel_info
                .split(", ")
                .map(|s| s.parse::<ValveKey>().unwrap())
                .collect(),
        })
    }
}

impl Network {
    fn get_shortest_paths(&self, start: ValveKey) -> HashMap<ValveKey, Vec<ValveKey>> {
        let mut current: HashMap<ValveKey, Vec<ValveKey>> = HashMap::new();

        current.insert(start, vec![]);

        let mut connections = current.clone();

        while !current.is_empty() {
            let mut next: HashMap<ValveKey, Vec<ValveKey>> = HashMap::new();
            for (name, path) in current {
                for link in self.valves[&name].tunnels.iter().copied() {
                    if let Entry::Vacant(e) = connections.entry(link) {
                        let conn_path: Vec<ValveKey> =
                            path.iter().copied().chain(std::iter::once(link)).collect();

                        e.insert(conn_path.clone());
                        next.insert(link, conn_path);
                    }
                }
            }

            current = next;
        }

        connections
    }
}

impl State<'_> {
    fn turns_left(&self) -> u64 {
        self.max_time - self.current_time
    }

    fn apply_move(&self, m: &Move) -> Self {
        let mut next_state = self.clone();

        next_state.current_position = m.target;
        next_state.opened_valves.insert(m.target);
        next_state.current_time += (m.path.len() as u64) + 1;
        next_state.pressure += m.reward;

        next_state
    }

    fn get_possible_moves(&self) -> Vec<Move> {
        let possible_moves = self
            .network
            .get_shortest_paths(self.current_position)
            .iter()
            .filter_map(|(key, path)| {
                if self.opened_valves.contains(key) {
                    return None;
                }

                let valve = self.network.valves.get(key).unwrap();

                if valve.rate == 0 {
                    return None;
                }

                let travel_time = path.len() as u64;
                if let Some(time_spent) = self.turns_left().checked_sub(travel_time + 1) {
                    let reward = time_spent * valve.rate;

                    return Some(Move {
                        reward,
                        target: *key,
                        path: path.clone(),
                    });
                }

                None
            })
            .collect();

        possible_moves
    }

    fn get_best_state_dfs(&self) -> Self {
        let mut best_state = self.clone();
        let mut max_pressure: u64 = 0;

        let mut all_moves = self.get_possible_moves();
        all_moves.sort_unstable_by(|a, b| b.reward.cmp(&a.reward));

        for m in all_moves {
            let next_state = self.apply_move(&m).get_best_state_dfs();

            if next_state.pressure > max_pressure {
                max_pressure = next_state.pressure;
                best_state = next_state;
            }
        }

        best_state
    }
}

fn main() {
    let input = include_str!("../inputs/day16.txt");

    let valves = input
        .lines()
        .map(|l| {
            let valve = l.parse::<Valve>().unwrap();
            (valve.key, valve)
        })
        .collect::<HashMap<_, _>>();

    let network = Network { valves };

    let state = State {
        network: &network,
        current_position: ValveKey(*b"AA"),
        max_time: 30,
        current_time: 0,
        opened_valves: HashSet::new(),
        pressure: 0,
    };

    let best_state = state.get_best_state_dfs();

    println!("{}", best_state.pressure);
}
