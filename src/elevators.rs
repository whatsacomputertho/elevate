//Import source modules
use crate::elevator::Elevator;
use crate::people::People;

/// # `Elevators` trait
///
/// A `Elevators` implementation is representative of a collection of `Elevator`s.
/// It is implemented by the `Building` struct.
pub trait Elevators {
    fn get_dest_floors(&self) -> Vec<usize>;

    fn get_energy_spent(&mut self) -> f64;

    fn update_floors(&mut self);

    fn increment_wait_times(&mut self);

    fn append_elevator(&mut self, capacity: usize, energy_up: f64, energy_down: f64, energy_coef: f64);

    fn update_capacities(&mut self, capacity: usize);
}

//Implementation of elevators trait for Vec<Elevators>
impl Elevators for Vec<Elevator> {
    /// Get an aggregated list of destination floors across the vector of elevators.
    fn get_dest_floors(&self) -> Vec<usize> {
        //Initialize a vector of usizes to track the overall dest floors
        let mut dest_floors: Vec<usize> = Vec::new();

        //Loop through the elevators and get the dest floor vectors
        for elevator in self.iter() {
            //Get the dest floors of the elevator
            let elevator_dest_floors: Vec<usize> = elevator.get_dest_floors();

            //Append the dest floors to the list of dest floors if not contained
            for dest_floor in elevator_dest_floors.iter() {
                if dest_floors.contains(dest_floor) {
                    continue;
                }
                dest_floors.push(*dest_floor);
            }
        }

        //Return the dest floors
        dest_floors
    }

    /// Calculate the total energy spent across the vector of elevators.
    fn get_energy_spent(&mut self) -> f64 {
        //Initialize an f64 to aggregate the total energy spent
        let mut energy_spent: f64 = 0.0_f64;

        //Loop through the elevators and calculate their energy spent
        for elevator in self.iter_mut() {
            let elevator_energy_spent: f64 = elevator.get_energy_spent();

            //Add the energy spent to the total
            energy_spent += elevator_energy_spent;
        }

        //Return the aggregate energy spent
        energy_spent
    }

    /// For each elevator, update its floor based on its `stopped` boolean and its
    /// `moving_up` boolean.
    fn update_floors(&mut self) {
        for elevator in self.iter_mut() {
            elevator.update_floor();
        }
    }

    /// For each elevator, increment the wait times of the people on the elevator if
    /// they are not on their desired floor.
    fn increment_wait_times(&mut self) {
        for elevator in self.iter_mut() {
            elevator.increment_wait_times();
        }
    }

    /// Appends a new elevator to the collection of elevators
    fn append_elevator(&mut self, capacity: usize, energy_up: f64, energy_down: f64, energy_coef: f64) {
        self.push(Elevator::from(capacity, energy_up, energy_down, energy_coef));
    }

    /// Updates the capacity across each of the elevators
    fn update_capacities(&mut self, capacity: usize) {
        //Ensure that the capacity is not less than the current
        //numer of people on any given elevator
        let can_update_capacities: bool = {
            let mut tmp_can_update_capacities: bool = true;
            for elevator in self.iter() {
                if capacity < elevator.get_num_people() {
                    tmp_can_update_capacities = false;
                    break;
                }
            }
            tmp_can_update_capacities
        };

        //If the capacity can be updated across all elevators,
        //then update the capacities
        if can_update_capacities {
            for elevator in self.iter_mut() {
                elevator.capacity = capacity;
            }
        }
    }
}