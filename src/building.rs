//Import external/standard modules
use rand::Rng;
use rand::distributions::Distribution;
use statrs::distribution::{Poisson, Binomial};

//Import source modules
use crate::person::Person;
use crate::people::People;
use crate::floor::Floor;
use crate::floors::Floors;
use crate::elevator::Elevator;
use crate::elevators::Elevators;

//Constant representing the probability a person leaves the building during a time step
const P_OUT: f64 = 0.05_f64;

//Constant representing the probability a person leaves a tip
const P_TIP: f64 = 0.5_f64;

//Constants defining the Bernoulli distribution parameters to sample from to generate tips
const DST_TIP_TRIALS: u64 = 100_u64;
const DST_TIP_SUCCESS: f64 = 0.5_f64;

/// # `Building` struct
///
/// A `Building` aggregates `Elevator`s and `Floor`s.  It also tracks the everage
/// energy usage by the elevators, and the average wait time among the people on
/// the building's floors and elevators.  It randomly generates arrivals.
#[derive(Clone)]
pub struct Building {
    pub elevators: Vec<Elevator>,
    pub floors: Vec<Floor>,
    pub avg_energy: f64,
    pub avg_wait_time: f64,
    pub tot_tips: f64,
    wait_time_denom: usize,
    p_in: f64,
    dst_in: Poisson,
    dst_tip: Binomial
}

/// # `Building` type implementation
///
/// The following functions are used to control the behavior of the people and elevators
/// owned by the `Building` type.
impl Building {
    /// Initialize a new building given the number of floors, the number of elevators
    /// the expected number of arrivals per time step, the base energy spent while moving
    /// an elevator up and down, and the additional energy spent per person moved.
    ///
    /// ## Example
    ///
    /// ```
    /// let num_floors: usize = 4_usize;
    /// let num_elevators: usize = 2_usize;
    /// let p_in: f64 = 0.5_f64;
    /// let energy_up: f64 = 5.0_f64;
    /// let energy_down: f64 = 2.5_f64;
    /// let energy_coef: f64 = 0.5_f64;
    /// let my_building: Building = Building::from(
    ///     num_floors,
    ///     num_elevators,
    ///     p_in,
    ///     energy_up,
    ///     energy_down,
    ///     energy_coef
    /// );
    /// ```
    pub fn from(num_floors: usize, num_elevators: usize, p_in: f64, energy_up: f64,
                energy_down: f64, energy_coef: f64) -> Building {
        //Initialize the Floors
        let floors: Vec<Floor> = {
            let mut tmp_floors: Vec<Floor> = Vec::new();
            for _ in 0_usize..num_floors {
                let tmp_floor: Floor = Floor::new();
                tmp_floors.push(tmp_floor);
            }
            tmp_floors
        };
    
        //Initialize the Elevators
        let elevators: Vec<Elevator> = {
            let mut tmp_elevators: Vec<Elevator> = Vec::new();
            for _ in 0_usize..num_elevators {
                let tmp_elevator: Elevator = Elevator::from(
                    energy_up, energy_down, energy_coef
                );
                tmp_elevators.push(tmp_elevator);
            }
            tmp_elevators
        };
    
        //Initialize the arrival probability distribution
        let dst_in = Poisson::new(p_in).unwrap();
    
        //Initialize and return the Building
        Building {
            floors: floors,
            elevators: elevators,
            avg_energy: 0_f64,
            avg_wait_time: 0_f64,
            wait_time_denom: 0_usize,
            tot_tips: 0_f64,
            p_in: p_in,
            dst_in: dst_in,
            dst_tip: Binomial::new(DST_TIP_SUCCESS, DST_TIP_TRIALS).unwrap()
        }
    }

    /// Calculate the probability that each floor becomes a destination floor for an elevator
    /// during the next time step.  If the floor currently is a destination floor, then return
    /// `1_f64`.
    pub fn update_dest_probabilities(&mut self) {
        //Get the number of floors in the building
        let num_floors: usize = self.floors.len() as usize;

        //Get the destination floors across each elevator
        let dest_floors: Vec<usize> = self.elevators.get_dest_floors();

        //Loop through the floors
        for (i, floor) in self.floors.iter_mut().enumerate() {
            //Initialize an f64 for this floor's probability
            let dest_probability: f64 = if i == 0 {
                //If this is the first floor, then calculate the prob
                //based on arrival probability only
                let people_waiting: f64 = {
                    let waiting: f64 = if floor.are_people_waiting() { 1_f64 } else { 0_f64 };
                    let going: f64 = if dest_floors.contains(&i) { 1_f64 } else { 0_f64 };
                    if waiting > going { waiting } else { going }
                };
                let p_in: f64 = self.p_in * ((num_floors as f64 - 1_f64)/(num_floors as f64));
                if people_waiting > p_in { people_waiting } else { p_in }
            } else {
                //If this is not the first floor, then calculate the
                //prob based on the elevator's people and the floor's
                //people and append it to the list
                let people_waiting: f64 = {
                    let waiting: f64 = if floor.are_people_waiting() { 1_f64 } else { 0_f64 };
                    let going: f64 = if dest_floors.contains(&i) { 1_f64 } else { 0_f64 };
                    if waiting > going { waiting } else { going }
                };
                let p_out: f64 = floor.get_p_out();
                if people_waiting > p_out { people_waiting } else { p_out }
            };
            floor.dest_prob = dest_probability;
        }
    }

    /// Generate the people arriving by sampling the Poisson distribution to receive a number
    /// of arrivals, and then instantiate that many people and append them to the first floor.
    pub fn gen_people_arriving(&mut self, mut rng: &mut impl Rng) {
        //Initialize a vector of Persons
        let mut arrivals: Vec<Person> = Vec::new();

        //Loop until no new arrivals occur, for each arrival append a new person
        for _ in 0_i32..self.dst_in.sample(&mut rng) as i32 {
            let new_person: Person = Person::from(P_OUT, P_TIP, self.floors.len(), &mut rng);
            arrivals.push(new_person);
        }

        //Extend the first floor with the new arrivals
        self.floors[0].extend(arrivals);
    }

    /// Given the number of people who decided to tip, generate the total value of their tips
    pub fn gen_tip_value(&self, num_tips: usize, rng: &mut impl Rng) -> f64 {
        //Initialize a float to store the tip value
        let mut tip_value: f64 = 0.0_f64;

        //Sample the tip distribution for each tip, randomizing the value of the tip
        //according to the tip distribution
        for _ in 0..num_tips {
            tip_value += self.dst_tip.sample(rng);
        }

        //Return the total tip value
        tip_value
    }

    /// For each of the building's elevators, exchange people between the elevator and its
    /// current floor if anyone on the elevator is going to the current floor, or if anyone on
    /// the floor is waiting for the elevator.
    pub fn exchange_people_on_elevator(&mut self) {
        for elevator in self.elevators.iter_mut() {
            //If the elevator is not stopped then continue
            if !elevator.stopped {
                continue;
            }

            //Get the elevator's floor index
            let floor_index: usize = elevator.floor_on;

            //Move people off the floor and off the elevator
            let people_leaving_floor: Vec<Person> = self.floors[floor_index].flush_people_entering_elevator();
            let mut people_leaving_elevator: Vec<Person> = elevator.flush_people_leaving_elevator();

            //Aggregate the wait times of the people leaving the elevator into the average and reset
            let wait_times: usize = people_leaving_elevator.get_aggregate_wait_time();
            let num_people: usize = people_leaving_elevator.get_num_people();
            self.avg_wait_time = {
                let tmp_num: f64 = wait_times as f64 + (self.avg_wait_time * self.wait_time_denom as f64);
                let tmp_denom: f64 = num_people as f64 + self.wait_time_denom as f64;
                if tmp_denom == 0_f64 {
                    0_f64 //If the denominator is 0, return 0 to avoid NaNs
                } else {
                    tmp_num / tmp_denom
                }
            };
            self.wait_time_denom += num_people;
            people_leaving_elevator.reset_wait_times();

            //Extend the current floor and elevator with the people getting on and off
            elevator.extend(people_leaving_floor);
            self.floors[floor_index].extend(people_leaving_elevator);
        }
    }

    /// Removes anyone who is leaving the first floor, and generates tips
    pub fn flush_and_update_tips(&mut self, rng: &mut impl Rng) {
        let people_leaving_floor: Vec<Person> = self.floors.flush_first_floor();
        let num_tips: usize = people_leaving_floor.gen_num_tips(rng);
        let tip_value: f64 = self.gen_tip_value(num_tips, rng);
        self.tot_tips += tip_value;
    }

    /// Returns all tips collected by the building, and resets the total tips to 0
    pub fn collect_tips(&mut self) -> f64 {
        let tips: f64 = self.tot_tips;
        self.tot_tips = 0.0_f64;
        tips
    }

    /// Update the average energy spent by the building's elevators given the time
    /// step and the energy spent during the time step.
    pub fn update_average_energy(&mut self, time_step: i32, energy_spent: f64) {
        self.avg_energy = {
            let tmp_num: f64 = (self.avg_energy * time_step as f64) + energy_spent;
            let tmp_denom: f64 = (time_step + 1_i32) as f64;
            tmp_num / tmp_denom
        };
    }
}

//Display trait implementation for a building
impl std::fmt::Display for Building {
    /// Format a `Building` as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// println!("{}", my_building);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut building_status: String = String::new();
        let elevator_space: String = String::from("   \t ");
        for (i, floor) in self.floors.iter().enumerate() {
            //Initialize strings representing this floor
            let mut floor_roof: String = String::from("----\t||---\t||");
            let mut floor_body: String = format!("{:.2}\t||{}\t||", floor.dest_prob, floor.get_num_people());

            //Loop through the elevators to check if any are on this floor
            let mut last_elevator_on_floor: usize = 0_usize;
            for (j, elevator) in self.elevators.iter().enumerate() {
                if elevator.floor_on != i as usize {
                    continue;
                }

                //If the elevator is on this floor, then display it i spaces away from the building
                let elevator_roof: String = format!("{}{}", str::repeat(&elevator_space, j - last_elevator_on_floor as usize), String::from("|-\t|"));
                let elevator_body: String = format!("{}|{}\t|", str::repeat(&elevator_space, j - last_elevator_on_floor as usize), elevator.get_num_people());

                //Append the elevator to the floor strings
                floor_roof.push_str(&elevator_roof);
                floor_body.push_str(&elevator_body);

                //Increment the counter for num elevators on this floor
                last_elevator_on_floor = j + 1_usize;
            }

            //Add the floor to the building status
            building_status = [floor_roof, floor_body, building_status].join("\n");
        }
        //Add the average energy and wait times throughout the building
        let wait_time_str: String = format!("Average wait time:\t{:.2}", self.avg_wait_time);
        let energy_str: String = format!("Average energy spent:\t{:.2}", self.avg_energy);
        let tip_str: String = format!("Total tips collected:\t${:.2}", self.tot_tips);
        building_status = [building_status, wait_time_str, energy_str, tip_str].join("\n");

        //Format the string and return
        f.write_str(&building_status)
    }
}

//Floors trait implementation for a building
impl Floors for Building {
    /// Determines whether there are any people waiting on a given floor.  Returns a bool
    /// which is true if so, and false if not.
    fn are_people_waiting_on_floor(&self, floor_index: usize) -> bool {
        self.floors.are_people_waiting_on_floor(floor_index)
    }

    /// Determines the nearest floor at which people are waiting with respect to the given
    /// floor.  Returns a tuple of usizes representing the floor index and the distance to
    /// the floor.
    fn get_nearest_wait_floor(&self, floor_on: usize) -> (usize, usize) {
        self.floors.get_nearest_wait_floor(floor_on)
    }

    /// Gets the probability that each floor becomes a destination floor in the next
    /// time step.
    fn get_dest_probabilities(&self) -> Vec<f64> {
        self.floors.get_dest_probabilities()
    }

    /// Randomly generates the people leaving each floor using each `Floor`'s
    /// `gen_people_leaving` function, which itself uses each `Person`'s `gen_is_leaving`
    /// function.
    fn gen_people_leaving(&mut self, rng: &mut impl Rng) {
        self.floors.gen_people_leaving(rng)
    }

    /// Removes anyone who is leaving the first floor, and generates tips
    fn flush_first_floor(&mut self) -> Vec<Person> {
        self.floors.flush_first_floor()
    }

    /// Increments the waiting times among people who are waiting/not at their destination
    /// floor throughout the building's floors and elevators.
    fn increment_wait_times(&mut self) {
        self.elevators.increment_wait_times();
        self.floors.increment_wait_times();
    }
}