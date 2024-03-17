//Import standard/imported modules
use rand::Rng;

//Import source modules
use crate::person::Person;

/// # `People` trait
///
/// A `People` implementation is representative of a collection of `Person`s.  It is
/// implemented by the `Elevator` and `Floor` structs.  It defines a set of functions
/// for managing `Person`s in aggregate.
pub trait People {
    /// Expected to generate the number of tips to collect from the people
    fn gen_num_tips(&self, rng: &mut impl Rng) -> usize;

    /// Expected to determine the destination floors for all people and return it as
    /// a vector.
    fn get_dest_floors(&self) -> Vec<usize>;

    /// Expected to determine the total number of people and return it as a usize.
    fn get_num_people(&self) -> usize;

    /// Expected to determine the number of people waiting, that is, not at their
    /// desired floor.
    fn get_num_people_waiting(&self) -> usize;

    /// Expected to determine the number of people going to a particular floor
    fn get_num_people_going_to_floor(&self, floor_to: usize) -> usize;

    /// Expected to read the wait times from people waiting/not at their desired floor
    /// and aggregate the total into a usize.
    fn get_aggregate_wait_time(&self) -> usize;

    /// Expected to determine whether anyone in the collection of people are going to
    /// a given floor, returning a bool which is true if so, and false if not.
    fn are_people_going_to_floor(&self, floor_index: usize) -> bool;

    /// Expected to determine whether anyone in the collection of people is waiting/not
    /// at their desired floor, returning a bool which is true if so, and false if not.
    fn are_people_waiting(&self) -> bool;

    /// Expected to increment the wait times (by `1_usize`) among all people waiting/not
    /// at their desired floor.
    fn increment_wait_times(&mut self);

    /// Expected to reset the wait times (to `0_usize`) among all people who have a
    /// nonzero wait time and are on their desired floor.
    fn reset_wait_times(&mut self);
}

impl People for Vec<Person> {
    /// Generates the number of people among the collection of people who will tip.
    fn gen_num_tips(&self, rng: &mut impl Rng) -> usize {
        //Initialize a counter for the number of people who will tip
        let mut num_tips: usize = 0_usize;

        //Loop through the people and generate whether each person will tip
        for pers in self.iter() {
            if pers.gen_tip(rng) {
                num_tips += 1_usize;
            }
        }

        //Return the counter
        num_tips
    }

    /// Determines the destination floors for all people and returns it as a vector.
    fn get_dest_floors(&self) -> Vec<usize> {
        //Initialize a new vector of usizes
        let mut dest_floors: Vec<usize> = Vec::new();

        //Loop through the vector of persons
        for pers in self.iter() {
            //Add the dest floor to the vector
            let dest_floor = pers.floor_to;
            dest_floors.push(dest_floor);
        }

        //Return the destination floors vector
        dest_floors
    }

    /// Determines the total number of people and returns it as a usize.
     fn get_num_people(&self) -> usize {
        //Return the length of the vector
        self.len()
    }

    /// Determines the number of people waiting, that is, not at their desired floor.
    fn get_num_people_waiting(&self) -> usize {
        //Initialize a usize counting the numper of people waiting
        let mut num_waiting: usize = 0_usize;

        //Loop through the vector of persons
        for pers in self.iter() {
            //Skip if the person is not waiting
            if pers.floor_on == pers.floor_to {
                continue;
            }

            //If the person is waiting, increment the counter
            num_waiting += 1_usize;
        }

        //Return the counter
        num_waiting
    }

    /// Determines the number of people going to a particular floor
    fn get_num_people_going_to_floor(&self, floor_to: usize) -> usize {
        //Initialize a usize counting the number of people going to the floor
        let mut num_going_to_floor: usize = 0_usize;

        //Loop through the vector of persons
        for pers in self.iter() {
            //Skip if the person is not going to that floor
            if pers.floor_to != floor_to {
                continue;
            }

            //If the person is going to that floor, increment the counter
            num_going_to_floor += 1_usize;
        }

        //Return the counter
        num_going_to_floor
    }

    /// Reads the wait times from people waiting/not at their desired floor and aggregates
    /// the total into a usize.
    fn get_aggregate_wait_time(&self) -> usize {
        //Initialize a usize for the number of time steps the people spent waiting
        let mut aggregate_wait_time: usize = 0_usize;

        //Loop through the vector of persons
        for pers in self.iter() {
            //Increment the usize with their wait time
            aggregate_wait_time += pers.wait_time;
        }

        //Return the usize
        aggregate_wait_time
    }

    /// Determines whether anyone in the collection of people are going to a given floor,
    /// and returns a bool which is true if so, and false if not.
    fn are_people_going_to_floor(&self, floor_index: usize) -> bool {
        //Initialize a boolean tracking if people are going to the given floor
        let mut is_going_to_floor: bool = false;

        //Loop through the people on the elevator and check
        for pers in self.iter() {
            //If the person is not going to the given floor then skip
            if pers.floor_to != floor_index {
                continue;
            }

            //Otherwise update the boolean and break
            is_going_to_floor = true;
            break;
        }

        //Return the is_going_to_floor boolean
        is_going_to_floor
    }

    /// Determines whether anyone in the collection of people is waiting/not at their
    /// desired floor, and returns a bool which is true if so, and false if not.
    fn are_people_waiting(&self) -> bool {
        //Initialize a boolean tracking if people are waiting
        let mut is_waiting: bool = false;

        //Loop through the people and check if they are waiting
        for pers in self.iter() {
            //If the person is not waiting, then skip
            if pers.floor_on == pers.floor_to {
                continue;
            }

            //Otherwise update the boolean and break
            is_waiting = true;
            break;
        }

        //Return the is_going_to_floor boolean
        is_waiting
    }

    /// Increments the wait times (by `1_usize`) among all people waiting/not at
    /// their desired floor.
    fn increment_wait_times(&mut self) {
        //Loop through the people and increment their wait times
        for pers in self.iter_mut() {
            pers.increment_wait_time();
        }
    }

    /// Resets the wait times (to `0_usize`) among all people who have a nonzero
    /// wait time and are on their desired floor.
    fn reset_wait_times(&mut self) {
        //Loop through the people and reset their wait times
        for pers in self.iter_mut() {
            pers.reset_wait_time();
        }
    }
}