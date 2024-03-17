//Import external/standard modules
use rand::Rng;

//Import source modules
use crate::person::Person;
use crate::people::People;

/// # `Floor` struct
///
/// A `Floor` is aggregated by buildings.  People travel between them using
/// elevators.  The floor struct generally should not be directly instantiated;
/// instead it should be managed in aggregate via the `Building` type.
#[derive(Clone)]
pub struct Floor {
    people: Vec<Person>,
    pub capacity: usize,
    pub dest_prob: f64
}

/// # `Floor` type implementation
///
/// The following functions are used by `Building`s and `Floors` implementations.
impl Floor {
    /// Initialize a new Floor with a zero destination probability and an empty
    /// vector of `Person`s.
    ///
    /// ## Example
    ///
    /// ```
    /// let capacity: usize = 100_usize;
    /// let my_floor: Floor = Floor::new(capacity);
    /// ```
    pub fn new(capacity: usize) -> Floor {
        Floor {
            people: Vec::new(),
            capacity: capacity,
            dest_prob: 0_f64
        }
    }

    /// Calculate the free capacity for the floor
    pub fn get_free_capacity(&self) -> usize {
        self.capacity - self.people.get_num_people()
    }

    /// Calculate the probability that a person on the floor leaves during the next
    /// time step, and return the result as an f64.
    pub fn get_p_out(&self) -> f64 {
        //If there is no one on the floor, return 0_f64
        if self.people.len() == 0 {
            return 0_f64;
        }

        //Initialize a p_out variable and a vec for each p_out
        let mut p_out: f64 = 0_f64;
        let mut past_p_outs: Vec<f64> = Vec::new();

        //Loop through the people in the floor and iteratively calculate
        //the p_out value
        for pers in self.people.iter() {
            //Calculate the product of each of the past people's inverse
            //p_out values
            let inverse_p_outs: f64 = {
                let mut tmp_inverse_p_outs: f64 = 1_f64;
                for past_p_out in &past_p_outs {
                    tmp_inverse_p_outs = tmp_inverse_p_outs * (1_f64 - past_p_out);
                }
                tmp_inverse_p_outs
            };

            //Calculate the summand value based on the person's p_out and
            //the product of each of the past people's p_out values
            let tmp_p_out: f64 = pers.p_out * inverse_p_outs;

            //Add the newly calculated value onto the p_out value and then
            //append the current p_out
            p_out += tmp_p_out;
            past_p_outs.push(pers.p_out);
        }

        //Return the p_out value
        p_out
    }

    /// Randomly generate whether anyone on the floor is leaving using each `Person`'s
    /// `gen_is_leaving` function.
    pub fn gen_people_leaving(&mut self, rng: &mut impl Rng) {
        //Loop through the people on the floor and decide if they are leaving
        for pers in self.people.iter_mut() {
            //Skip people who are waiting for the elevator
            if pers.floor_on != pers.floor_to {
                continue;
            }

            //Randomly generate whether someone not waiting for the elevator will leave
            let _is_person_leaving: bool = pers.gen_is_leaving(rng);
        }
    }

    /// Remove people from a floor who are currently waiting/not on their desired floor
    /// and return as a `Vec<Person>`.  This is used when the elevator is on this floor
    /// and there is an exchange of people between the elevator and the floor.  The people
    /// removed from the floor are limited to the free capacity of the elevator they are
    /// entering, which is given as a usize function parameter.
    pub fn flush_people_entering_elevator(&mut self, free_elevator_capacity: usize) -> Vec<Person> {
        //Initialize a vector of people for the people entering the elevator
        let mut people_entering_elevator: Vec<Person> = Vec::new();

        //Loop through the people on the floor and add to the vec
        let mut removals = 0_usize;
        for i in 0..self.people.len() {
            //Break if the people entering the elevator hits the elevator's
            //remaining free capacity
            if people_entering_elevator.len() == free_elevator_capacity {
                break;
            }
            
            //If the person is not waiting, then skip
            if self.people[i-removals].floor_on == self.people[i-removals].floor_to {
                continue;
            }

            //If the person is waiting, then remove them from the elevator
            //and add them to the leaving vec, incrementing the removals
            let person_entering_elevator: Person = self.people.remove(i - removals);
            people_entering_elevator.push(person_entering_elevator);
            removals += 1_usize;
        }

        //Return the vector of people leaving
        people_entering_elevator
    }

    /// Remove people entirely who are leaving the building.  This is used exclusively
    /// on the first floor.
    pub fn flush_people_leaving_floor(&mut self) -> Vec<Person> {
        //Initialize a vector of people for the people leaving the floor
        let mut people_leaving_floor: Vec<Person> = Vec::new();

        //Loop through the people on the floor and add to the vec if leaving
        let mut removals = 0_usize;
        for i in 0..self.people.len() {
            //If the person is not leaving, then skip
            if !self.people[i-removals].is_leaving {
                continue;
            }

            //If the person is leaving, then remove them from the floor
            //and add them to the leaving vec, incrementing the removals
            let person_leaving_floor: Person = self.people.remove(i - removals);
            people_leaving_floor.push(person_leaving_floor);
            removals += 1_usize;
        }

        //Return the vector of people leaving
        people_leaving_floor
    }
}

//Implement the extend trait for the floor struct
impl Extend<Person> for Floor {
    fn extend<T: IntoIterator<Item=Person>>(&mut self, iter: T) {
        //Add people onto the floor until at capacity
        for pers in iter {
            //Break if we reach capacity
            if self.people.get_num_people() == self.capacity {
                break;
            }

            //Add a person
            self.people.push(pers);
        }
    }
}

//Implement the people trait for the floor struct
impl People for Floor {
    /// Generates the number of people among the collection of people who will tip.
    fn gen_num_tips(&self, rng: &mut impl Rng) -> usize {
        self.people.gen_num_tips(rng)
    }

    /// Determines the destination floors for all people on the floor and returns it as
    /// a vector.
    fn get_dest_floors(&self) -> Vec<usize> {
        self.people.get_dest_floors()
    }

    /// Determines the total number of people on the floor and returns it as a usize.
    fn get_num_people(&self) -> usize {
        self.people.get_num_people()
    }

    /// Determines the number of people waiting on the floor, that is, not at their
    /// desired floor.
    fn get_num_people_waiting(&self) -> usize {
        self.people.get_num_people_waiting()
    }

    /// Determines the number of people going to a particular floor
    fn get_num_people_going_to_floor(&self, floor_to: usize) -> usize {
        self.people.get_num_people_going_to_floor(floor_to)
    }

    /// Reads the wait times from people waiting on the floor/not at their desired floor
    /// and aggregates the total into a usize.
    fn get_aggregate_wait_time(&self) -> usize {
        self.people.get_aggregate_wait_time()
    }

    /// Determines whether anyone on the floor are going to a given floor, and returns a
    /// bool which is true if so, and false if not.
    fn are_people_going_to_floor(&self, floor_index: usize) -> bool {
        self.people.are_people_going_to_floor(floor_index)
    }

    /// Determines whether anyone on the floor is waiting/not at their desired floor, and
    /// returns a bool which is true if so, and false if not.
    fn are_people_waiting(&self) -> bool {
        self.people.are_people_waiting()
    }

    /// Increments the wait times (by `1_usize`) among all people waiting on the floor/not
    /// at their desired floor.
    fn increment_wait_times(&mut self) {
        //Loop through the people
        for pers in self.people.iter_mut() {
            //If the person is not waiting, then skip
            if pers.floor_on == pers.floor_to {
                continue;
            }

            //Increment the person's wait time if they are waiting
            pers.increment_wait_time();
        }
    }

    /// Resets the wait times (to `0_usize`) among all people on the floor who have a nonzero
    /// wait time and are on their desired floor.
    fn reset_wait_times(&mut self) {
        //Loop through the people
        for pers in self.people.iter_mut() {
            //If the person is waiting, then skip
            if pers.floor_on != pers.floor_to {
                continue;
            }

            //Reset the person's wait time if they are not waiting
            pers.reset_wait_time();
        }
    }
}