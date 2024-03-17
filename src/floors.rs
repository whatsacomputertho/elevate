//Import source modules
use crate::floor::Floor;
use crate::person::Person;
use crate::people::People;

//Import external/standard modules
use rand::Rng;

/// # `Floors` trait
///
/// A `Floors` implementation is representative of a collection of `Floor`s.  It is
/// implemented by the `Building` struct.
pub trait Floors {
    /// Expected to determine whether there are any people waiting on a given floor.
    /// Returns a bool which is true if so, and false if not.
    fn are_people_waiting_on_floor(&self, floor_index: usize) -> bool;

    /// Expected to determine the nearest floor at which people are waiting with
    /// respect to the given floor.  Returns a tuple of usizes representing the floor
    /// index and the distance to the floor.
    fn get_nearest_wait_floor(&self, floor_on: usize) -> (usize, usize);

    /// Expected to get the probability that each floor becomes a destination floor in
    /// the next time step.
    fn get_dest_probabilities(&self) -> Vec<f64>;

    /// Expected to randomly generate the people leaving each floor using each `Floor`'s
    /// `gen_people_leaving` function, which itself uses each `Person`'s `gen_is_leaving`
    /// function.
    fn gen_people_leaving(&mut self, rng: &mut impl Rng);

    /// Expected to remove anyone who is leaving the first floor.
    fn flush_first_floor(&mut self) -> Vec<Person>;

    /// Expected to increment the waiting times among people who are waiting/not at their
    /// destination floor throughout the collection of floors.
    fn increment_wait_times(&mut self);

    /// Expected to append a new floor to the collection of floors.
    fn append_floor(&mut self, capacity: usize);

    /// Expected to update the capacities across each of the floors.
    fn update_capacities(&mut self, capacity: usize);
}

//Implement people trait for Vec<Floor>
impl Floors for Vec<Floor> {
    /// Determines whether there are any people waiting on a given floor.  Returns a bool
    /// which is true if so, and false if not.
    fn are_people_waiting_on_floor(&self, floor_index: usize) -> bool {
        self[floor_index].are_people_waiting()
    }

    /// Determines the nearest floor at which people are waiting with respect to the given
    /// floor.  Returns a tuple of usizes representing the floor index and the distance to
    /// the floor.
    fn get_nearest_wait_floor(&self, floor_on: usize) -> (usize, usize) {
        //Initialize variables to track the nearest waiting floor and
        //the min distance between here and that floor
        let mut nearest_wait_floor: usize = 0_usize;
        let mut min_wait_floor_dist: usize = 0_usize;

        //Loop through the floors and find the minimum distance floor
        //with waiting people
        for (i, floor) in self.iter().enumerate() {
            //Check if there is anyone waiting on the floor, if not
            //then continue
            if !floor.are_people_waiting() {
                continue;
            }

            //Calculate the distance between this floor and the waiting
            //floor
            let wait_floor_dist: usize = if floor_on > i {
                floor_on - i
            } else {
                i - floor_on
            };

            //Check whether this is less than the current minimum, or
            //if no minimum has been assigned yet (in which case it is
            //0_usize)
            if min_wait_floor_dist == 0_usize || wait_floor_dist < min_wait_floor_dist {
                min_wait_floor_dist = wait_floor_dist;
                nearest_wait_floor = i;
            }
        }

        //Return the nearest waiting floor
        (nearest_wait_floor, min_wait_floor_dist)
    }

    /// Gets the probability that each floor becomes a destination floor in the next
    /// time step.
    fn get_dest_probabilities(&self) -> Vec<f64> {
        //Initialize a new vec of f64s
        let mut dest_probabilities: Vec<f64> = Vec::new();

        //Loop through the floors
        for floor in self.iter() {
            //Push the floor's dest_prob value into the vector
            dest_probabilities.push(floor.dest_prob);
        }

        //Return the vector
        dest_probabilities
    }

    /// Randomly generates the people leaving each floor using each `Floor`'s
    /// `gen_people_leaving` function, which itself uses each `Person`'s `gen_is_leaving`
    /// function.
    fn gen_people_leaving(&mut self, mut rng: &mut impl Rng) {
        //Loop through the floors of the building
        for floor in self.iter_mut() {
            //Generate the people leaving on that floor
            floor.gen_people_leaving(&mut rng);
        }
    }

    /// Removes anyone who is leaving the first floor and returns the people who left as
    /// a vec of people.
    fn flush_first_floor(&mut self) -> Vec<Person> {
        self[0].flush_people_leaving_floor()
    }

    /// Increments the waiting times among people who are waiting/not at their destination
    /// floor throughout the collection of floors.
    fn increment_wait_times(&mut self) {
        for floor in self.iter_mut() {
            floor.increment_wait_times();
        }
    }

    /// Appends a new floor to the collection of floors.
    fn append_floor(&mut self, capacity: usize) {
        self.push(Floor::new(capacity));
    }

    /// Updates the capacity across each of the floors
    fn update_capacities(&mut self, capacity: usize) {
        //Ensure that the capacity is not less than the current
        //numer of people on any given floor
        let can_update_capacities: bool = {
            let mut tmp_can_update_capacities: bool = true;
            for floor in self.iter() {
                if capacity < floor.get_num_people() {
                    tmp_can_update_capacities = false;
                    break;
                }
            }
            tmp_can_update_capacities
        };

        //If the capacity can be updated across all floors, then
        //update the capacities
        if can_update_capacities {
            for floor in self.iter_mut() {
                floor.capacity = capacity;
            }
        }
    }
}