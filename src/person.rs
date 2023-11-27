//Import standard/imported libraries
use std::fmt;
use rand::Rng;
use rand::distributions::{Distribution, Uniform, Bernoulli};

/// # Person struct
///
/// A `Person` is aggregated by floors and elevators, and transported between floors
/// by elevators. The person struct generally should not be directly instantiated;
/// instead it should be managed in aggregate via the `Building` type.
pub struct Person {
    pub floor_on: usize,
    pub floor_to: usize,
    pub is_leaving: bool,
    pub wait_time: usize,
    pub p_out: f64,
    dst_out: Bernoulli
}


/// # Person type implementation
///
/// The following functions are used by `Elevator`/`Elevators`, `Floor`/`Floors`, and
/// `Building` types to randomly generate the behavior of `Person`s
impl Person {
    /// Initialize a new person given that persons probability of leaving, the number of
    /// floors in the building, and an Rng implementation to randomize the person's
    /// destination floor
    ///
    /// ### Example
    ///
    /// ```
    /// let p_out: f64 = 0.05_f64; //Must be between 0 and 1
    /// let num_floors: usize = 5_usize;
    /// let my_rng = rand::thread_rng(); //From rand library
    /// let my_pers: Person = Person::from(p_out, num_floors, &mut my_rng);
    /// ```
    pub fn from(p_out: f64, num_floors: usize, mut rng: &mut impl Rng) -> Person {
        let dst_to = Uniform::new(0_usize, num_floors);
        let floor_to: usize = dst_to.sample(&mut rng);
        Person {
            floor_on: 0_usize,
            floor_to: floor_to,
            is_leaving: false,
            wait_time: 0_usize,
            p_out: p_out,
            dst_out: Bernoulli::new(p_out).unwrap()
        }
    }

    /// Sample a person's `dst_out` distribution to update the person's `is_leaving`
    /// property randomly and return the result as a bool.  Or if the person is already
    /// leaving then return the property as is.
    pub fn gen_is_leaving(&mut self, mut rng: &mut impl Rng) -> bool {
        //Check if the is_leaving boolean is true, if so return it
        if self.is_leaving {
            return self.is_leaving;
        }

        //If the person is not leaving, then randomly generate whether they wish to leave
        let pers_is_leaving: bool = self.dst_out.sample(&mut rng);
        if pers_is_leaving {
            self.floor_to = 0_usize;
            self.is_leaving = pers_is_leaving;
        }
        self.is_leaving
    }

    /// Increment a person's `wait_time` property by `1_usize`.  Generally this should be
    /// called by `Elevator`/`Elevators`, `Floor`/`Floors`, and `Building` types aggregating
    /// `Person`s when the `Person` is not at their desired floor.
    pub fn increment_wait_time(&mut self) {
        //Increment the person's wait time counter
        self.wait_time += 1_usize;
    }

    /// Reset a person's `wait_time` property to `0_usize`.  Generally this should be
    /// called by `Elevator`/`Elevators`, `Floor`/`Floors`, and `Building` types aggregating
    /// `Person`s when the `Person` finally reaches their desired floor.
    pub fn reset_wait_time(&mut self) {
        //Reset the person's wait time counter
        self.wait_time = 0_usize;
    }
}

impl fmt::Display for Person {
    /// Format a `Person` as a string.  If a person is not at their desired floor then display
    /// the person's current and desired floor like so: `Person 2 -> 4`.  If the person is at
    /// their desired floor then just display their current floor like so: `Person 4`.
    ///
    /// ### Example
    ///
    /// ```
    /// println!("{}", my_pers);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str: String = if self.floor_on != self.floor_to {
            format!("Person {} -> {}", self.floor_on, self.floor_to)
        } else {
            format!("Person {}", self.floor_on)
        };
        f.write_str(&display_str)
    }
}