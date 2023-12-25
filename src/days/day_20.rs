use std::collections::{HashMap, VecDeque};
use std::fs;
use std::str::FromStr;

fn read_file() -> String {
    fs::read_to_string("src/inputs/day_20.txt").expect("Error reading the file")
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Module {
    FlipFlop {
        name: String,
        destinations: Vec<String>,
        state: bool,
    },
    Conjunction {
        name: String,
        destinations: Vec<String>,
        inputs: HashMap<String, bool>,
    },
    Broadcast {
        name: String,
        destinations: Vec<String>,
    },
}

impl Module {
    pub fn name(&self) -> String {
        match self {
            Module::FlipFlop { name, .. } => name.clone(),
            Module::Conjunction { name, .. } => name.clone(),
            Module::Broadcast { name, .. } => name.clone(),
        }
    }

    pub fn destinations(&self) -> Vec<String> {
        match self {
            Module::FlipFlop {
                name: _,
                destinations,
                state: _,
            } => destinations.clone(),
            Module::Conjunction {
                name: _,
                destinations,
                inputs: _,
            } => destinations.clone(),
            Module::Broadcast {
                name: _,
                destinations,
            } => destinations.clone(),
        }
    }

    pub fn process_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        match self {
            Module::FlipFlop {
                name,
                destinations,
                state,
            } => {
                if pulse.value {
                    return Vec::new();
                }

                *state = !*state;
                destinations
                    .iter()
                    .map(|destination| Pulse {
                        origin: name.clone(),
                        address: destination.clone(),
                        value: *state,
                    })
                    .collect()
            }
            Module::Conjunction {
                name,
                destinations,
                inputs,
            } => {
                *inputs.get_mut(&pulse.origin).unwrap() = pulse.value;

                let value = !inputs.values().all(|&value| value);

                destinations
                    .iter()
                    .map(|destination| Pulse {
                        origin: name.clone(),
                        address: destination.clone(),
                        value,
                    })
                    .collect()
            }
            Module::Broadcast { name, destinations } => destinations
                .iter()
                .map(|destination| Pulse {
                    origin: name.clone(),
                    address: destination.clone(),
                    value: pulse.value,
                })
                .collect(),
        }
    }
}

impl FromStr for Module {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        let destinations: Vec<String> = parts
            .last()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        let &first = parts.first().unwrap();
        if first == "broadcaster" {
            return Ok(Module::Broadcast {
                name: String::from("broadcaster"),
                destinations,
            });
        }

        let symbol = first.chars().next().unwrap();
        let name = &first[1..];

        match symbol {
            '&' => Ok(Module::Conjunction {
                name: name.to_string(),
                destinations,
                inputs: HashMap::new(),
            }),
            '%' => Ok(Module::FlipFlop {
                name: name.to_string(),
                destinations,
                state: false,
            }),
            _ => panic!("Invalid symbol: {}", symbol),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Pulse {
    origin: String,
    address: String,
    value: bool,
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn first() {
    let mut modules: HashMap<_, _> = read_file()
        .lines()
        .map(|s| {
            let module: Module = s.parse().unwrap();
            (module.name(), module)
        })
        .collect();

    for (name, module) in modules.clone() {
        for destination in module.destinations() {
            if let Some(Module::Conjunction {
                name: _,
                destinations: _,
                inputs,
            }) = &mut modules.get_mut(&destination)
            {
                inputs.insert(name.clone(), false);
            }
        }
    }

    let mut high_pulses = 0;
    let mut low_pulses = 0;
    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back(Pulse {
            origin: String::from("button"),
            address: String::from("broadcaster"),
            value: false,
        });
        while let Some(pulse) = pulses.pop_front() {
            if pulse.value {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }

            let Some(module) = modules.get_mut(&pulse.address) else {
                continue;
            };

            for pulse in module.process_pulse(pulse) {
                pulses.push_back(pulse);
            }
        }
    }

    println!("{}", high_pulses * low_pulses);
}

pub fn second() {
    // the modules make a series of 4 12-bit counters that are incremented each time a pulse is sent
    // each counter resets at a certain period given below
    let periods = [
        0b_11_11_10_11_01_01,
        0b_11_11_01_01_10_11,
        0b_11_11_01_00_01_11,
        0b_11_11_01_00_00_11,
    ];

    // calculate lcm of each number in periods
    let lcm = periods
        .iter()
        .fold(1, |acc, &period| acc * period / gcd(acc, period));

    println!("{}", lcm);
}
