//Import source modules
use crate::person::Person;
use crate::people::People;

/// # Elevator struct
///
/// An `Elevator` is aggregated by buildings, and transports people between floors.
/// The `Elevator` struct generally should not be directly instantiated; instead it
/// should be managed via the `Building` type and `ElevatorController` implementations.
pub struct Elevator {
    pub floor_on: usize,
    pub moving_up: bool,
    pub stopped: bool,
    pub people: Vec<Person>,
    energy_up: f64,
    energy_down: f64,
    energy_coef: f64
}

/// # Elevator type implementation
///
/// The following functions are used by `Building` and `Controller` types as well as
/// `Elevators` implementations to update and control the behavior of an `Elevator`.
impl Elevator {
    /// Initialize a new elevator given the elevator's energy spent moving up, energy
    /// spent moving down, and energy coefficient (additional energy spent per person
    /// transported).  The elevator is initialized stopped on the first floor with no
    /// people.
    ///
    /// ### Example
    ///
    /// ```
    /// let energy_up: f64 = 5.0_f64;
    /// let energy_down: f64 = 2.5_f64;
    /// let energy_coef: f64 = 0.5_f64;
    /// let my_elev: Elevator = Elevator::from(energy_up, energy_down, energy_coef);
    /// ```
    pub fn from(energy_up: f64, energy_down: f64, energy_coef: f64) -> Elevator {
        Elevator {
            floor_on: 0_usize,
            moving_up: false,
            stopped: true,
            people: Vec::new(),
            energy_up: energy_up,
            energy_down: energy_down,
            energy_coef: energy_coef
        }
    }
    
    /// Calculate the total energy spent (as an `f64`) while the elevator is moving.
    /// If the elevator is not moving then return `0.0_f64`.
    pub fn get_energy_spent(&mut self) -> f64 {
        let energy_spent = if self.stopped {
                0.0_f64
            } else if self.moving_up {
                self.energy_up + (self.energy_coef * (self.people.len() as f64))
            } else {
                self.energy_down + (self.energy_coef * (self.people.len() as f64))
            };
        energy_spent
    }

    /// Use the `stopped` and `moving_up` properties of the elevator to update the
    /// elevator's floor index.  If stopped, then no change.  If moving up then
    /// increment the `floor_on` by `1_usize`.  If moving down then decrement the
    /// `floor_on` by `1_usize`.
    pub fn update_floor(&mut self) -> usize {
        //If the elevator is stopped, then return early
        if self.stopped {
            return self.floor_on;
        }

        //If the elevator is moving then update the floor the elevator is on
        self.floor_on = if self.moving_up {
            self.floor_on + 1_usize
        } else {
            self.floor_on - 1_usize
        };

        //Loop through the elevator's people and update their floor accordingly
        for pers in self.people.iter_mut() {
            pers.floor_on = self.floor_on;
        }

        //Return the floor the elevator is on
        self.floor_on
    }
    
    /// If there are people on the elevator, this returns the nearest destination
    /// floor among those people represented as a length-2 tuple of `usize`s.  The
    /// first element is the destination floor, and the second is the distance to
    /// the floor.  If there are no people on the floor, it returns `(0_usize, 0_usize)`.
    pub fn get_nearest_dest_floor(&self) -> (usize, usize) {
        //Get the current floor the elevator is on
        let floor_index: usize = self.floor_on;

        //Get the destination floors from the elevator, if none then return
        let dest_floors: Vec<usize> = self.get_dest_floors();
        if dest_floors.len() == 0_usize {
            return (0_usize, 0_usize);
        }

        //Initialize variables to track the nearest destination floor
        //and the min distance between here and a destination floor
        let mut nearest_dest_floor: usize = 0_usize;
        let mut min_dest_floor_dist: usize = 0_usize;

        //Calculate the distance between each dest floor and the current floor
        for dest_floor_index in dest_floors.iter() {
            let dest_floor_dist: usize = if floor_index > *dest_floor_index {
                floor_index - dest_floor_index
            } else {
                dest_floor_index - floor_index
            };

            //Check whether this is less than the current minimum, or if no
            //minimum has been assigned yet (in which case it is 0_usize)
            if min_dest_floor_dist == 0_usize || dest_floor_dist < min_dest_floor_dist {
                min_dest_floor_dist = dest_floor_dist;
                nearest_dest_floor = *dest_floor_index;
            }
        }

        //Return the nearest destination floor
        (nearest_dest_floor, min_dest_floor_dist)
    }

    /// If the elevator is stopped, this function returns a `Vec<Person>` containing
    /// the people on the elevator whose destination floor is the current floor.  If
    /// the elevator is not stopped, this function returns an empty vector.
    pub fn flush_people_leaving_elevator(&mut self) -> Vec<Person> {
        //Initialize a vector of people for the people leaving
        let mut people_leaving: Vec<Person> = Vec::new();

        //If the elevator is not stopped then return the empty vector
        if !self.stopped {
            return people_leaving;
        }

        //Loop through the people on the elevator and add to the vec
        let mut removals = 0_usize;
        for i in 0..self.people.len() {
            //If the person is not on their destination floor, then skip
            if self.people[i-removals].floor_on != self.people[i-removals].floor_to {
                continue;
            }

            //If the person is on their destination floor, then remove them from
            //the elevator and add them to the leaving vec, incrementing the removals
            let person_leaving: Person = self.people.remove(i - removals);
            people_leaving.push(person_leaving);
            removals += 1_usize;
        }

        //Return the vector of people leaving
        people_leaving
    }
}

//Implement the extend trait for the elevator struct
impl Extend<Person> for Elevator {
    fn extend<T: IntoIterator<Item=Person>>(&mut self, iter: T) {
        for pers in iter {
            self.people.push(pers);
        }
    }
}

//Implement the people trait for the elevator struct
impl People for Elevator {
    /// Determines the destination floors for all people and returns it as a vector.
    fn get_dest_floors(&self) -> Vec<usize> {
        self.people.get_dest_floors()
    }

    /// Determines the total number of people and returns it as a usize.
    fn get_num_people(&self) -> usize {
        self.people.get_num_people()
    }

    /// Determines the number of people waiting, that is, not at their desired floor.
    fn get_num_people_waiting(&self) -> usize {
        self.people.get_num_people_waiting()
    }

    /// Reads the wait times from people waiting/not at their desired floor and aggregates
    /// the total into a usize.
    fn get_aggregate_wait_time(&self) -> usize {
        self.people.get_aggregate_wait_time()
    }

    /// Determines whether anyone in the collection of people are going to a given floor,
    /// and returns a bool which is true if so, and false if not.
    fn are_people_waiting(&self) -> bool {
        self.people.are_people_waiting()
    }

    /// Determines whether anyone in the collection of people is waiting/not at their
    /// desired floor, and returns a bool which is true if so, and false if not.
    fn are_people_going_to_floor(&self, floor_index: usize) -> bool {
        self.people.are_people_going_to_floor(floor_index)
    }

    /// Increments the wait times (by `1_usize`) among all people waiting/not at
    /// their desired floor.
    fn increment_wait_times(&mut self) {
        self.people.increment_wait_times()
    }

    /// Resets the wait times (to `0_usize`) among all people who have a nonzero
    /// wait time and are on their desired floor.
    fn reset_wait_times(&mut self) {
        self.people.reset_wait_times()
    }
}