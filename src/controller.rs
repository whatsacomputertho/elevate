//Import source modules
use crate::building::Building;
use crate::floors::Floors;
use crate::people::People;

//Implement standard/imported modules
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::{Distribution, Uniform, Bernoulli};

/// # `ElevatorController` trait
///
/// An `ElevatorController` implementation controls the elevators of a building.
pub trait ElevatorController {
    fn get_building(&mut self) -> &Building;

    fn get_building_mut(&mut self) -> &mut Building;

    fn clone_building(&mut self) -> Building;

    fn can_be_upgraded(&self) -> bool;

    fn upgrade(&mut self, incrementation: f64);

    fn update_elevators(&mut self);
}

/// # `RandomController` struct
///
/// A `RandomController` implements the `ElevatorController` trait.  It randomly
/// generates destination floors for each of a building's elevators once the elevator
/// reaches its destination floor.
 pub struct RandomController {
    pub building: Building,
    num_floors: usize,
    floors_to: Vec<Option<usize>>,
    dst_to: Uniform<usize>,
    p_rational: f64,
    dst_rational: Bernoulli,
    upgradable: bool,
    rng: StdRng
}

//Implement the RandomController interface
impl RandomController {
    /// Initialize a new RandomController given a `Building`, an `StdRng` (from
    /// the rand library), and an `f64` representing the probability that the
    /// RandomController behaves rationally.
    ///
    /// ## Example
    ///
    /// ```
    /// let my_rng = rand::thread_rng();
    /// let my_building: Building = Building::from(
    ///     4_usize,
    ///     2_usize,
    ///     0.5_f64,
    ///     5.0_f64,
    ///     2.5_f64,
    ///     0.5_f64
    /// );
    /// let my_controller: RandomController = RandomController::from(
    ///     my_building,
    ///     my_rng,
    ///     0.5_f64
    /// );
    /// ```
    pub fn from(building: Building, rng: StdRng, p_rational: f64) -> RandomController {
        //Get the number of floors and elevators in the building
        let num_floors: usize = building.floors.len();
        let num_elevators: usize = building.elevators.len();

        //Initialize the destination floors for the elevators
        let floors_to: Vec<Option<usize>> = {
            let mut tmp_floors_to: Vec<Option<usize>> = Vec::new();
            for _ in 0..num_elevators {
                tmp_floors_to.push(None);
            }
            tmp_floors_to
        };

        //Initialize the distribution for randomizing dest floors
        let dst_to: Uniform<usize> = Uniform::new(0_usize, num_floors);

        //Initialize the controller
        RandomController {
            building: building,
            num_floors: num_floors,
            floors_to: floors_to,
            dst_to: dst_to,
            p_rational: p_rational,
            dst_rational: Bernoulli::new(p_rational).unwrap(),
            upgradable: true,
            rng: rng
        }
    }

    /// Initialize a new RandomController from just a building.  The rng is
    /// created on the fly, and the rational probability is defaulted to 0.
    ///
    /// ## Example
    ///
    /// ```
    /// let my_building: Building = Building::from(
    ///     4_usize,
    ///     2_usize,
    ///     0.5_f64,
    ///     5.0_f64,
    ///     2.5_f64,
    ///     0.5_f64
    /// );
    /// let my_controller: RandomController = RandomController::from(my_building);
    /// ```
    pub fn from_building(building: Building) -> RandomController {
        //Initialize default values for the additional properties for this controller
        let rng = StdRng::from_seed(rand::thread_rng().gen());
        let p_rational = 0.0_f64;

        //Initialize and return the RandomController
        RandomController::from(building, rng, p_rational)
    }

    /// Set the destination floors of the elevators randomly according to
    /// random or rational logic, depending on the p_rational
    pub fn update_floors_to(&mut self) {
        //If the number of elevators in the building is greater than the number
        //of destination floors in the controller, then add new destination
        //floors
        while self.building.elevators.len() > self.floors_to.len() {
            self.floors_to.push(None);
        }

        //If the numer of floors in the building is greater than the number of
        //floors tracked by the controller, then update the number of floors
        //tracked by the controller and re-instantiate the dest distribution
        if self.building.floors.len() != self.num_floors {
            self.num_floors = self.building.floors.len();
            self.dst_to = Uniform::new(0, self.num_floors);
        }

        //Loop through the elevators in the building
        for (i, elevator) in self.building.elevators.iter().enumerate() {
            //If the destination floor for the elevator is None, then update it
            match self.floors_to[i] {
                Some(_) => {},
                None => {
                    if self.dst_rational.sample(&mut self.rng) {
                        if elevator.stopped {
                            //Find the nearest destination floor among people on the elevator
                            let (nearest_dest_floor, min_dest_floor_dist): (usize, usize) = elevator.get_nearest_dest_floor();
                
                            //If the nearest dest floor is identified, then set as the dest floor
                            if min_dest_floor_dist != 0_usize {
                                self.floors_to[i] = Some(nearest_dest_floor);
                                continue;
                            }
                
                            //Find the nearest waiting floor among people throughout the building
                            let (nearest_wait_floor, min_wait_floor_dist): (usize, usize) = self.building.get_nearest_wait_floor(elevator.floor_on);
                
                            //If the nearest wait floor is identified, then set as the dest floor
                            if min_wait_floor_dist != 0_usize {
                                self.floors_to[i] = Some(nearest_wait_floor);
                                continue;
                            }
                        }
                    } else {
                        self.floors_to[i] = Some(self.dst_to.sample(&mut self.rng));
                        continue;
                    }
                    self.floors_to[i] = Some(elevator.floor_on);
                }
            }
        }
    }

    /// If any elevators are at their destination floor, then set that floor
    /// to None so that it can be re-randomized next time step.
    pub fn clear_floors_to(&mut self) {
        //Loop through the elevators in the building
        for (i, elevator) in self.building.elevators.iter().enumerate() {
            let dest_floor = self.floors_to[i].unwrap();
            if dest_floor == elevator.floor_on {
                self.floors_to[i] = None;
            }
        }
    }
}

//Implement the ElevatorController trait for the RandomController
impl ElevatorController for RandomController {
    /// Immutably borrow the building belonging to the controller
    fn get_building(&mut self) -> &Building {
        &self.building
    }

    /// Mutably borrow the building belonging to the controller
    fn get_building_mut(&mut self) -> &mut Building {
        &mut self.building
    }

    /// Clone the building belonging to the controller.  Generally used when
    /// swapping controllers.
    fn clone_building(&mut self) -> Building {
        self.building.clone()
    }

    /// Return a boolean signifying whether the controller can be upgraded or
    /// not.
    fn can_be_upgraded(&self) -> bool {
        //If the controller is 100% rational, then no further upgrades are
        //possible
        if self.p_rational >= 1.0_f64 {
            return false;
        }

        //Otherwise, the elevator controller can be upgraded
        self.upgradable
    }

    /// Upgrade the controller given an incrementation float
    fn upgrade(&mut self, incrementation: f64) {
        //Add the current rationality probability to the incrementation and
        //check to see if it exceeds 1, if so then ceiling it at 1.0
        let mut new_p_rational: f64 = self.p_rational + incrementation;
        if new_p_rational > 1.0_f64 {
            new_p_rational = 1.0_f64;
        }

        //Update the rationality probability and distribution of the controller
        self.p_rational = new_p_rational;
        self.dst_rational = Bernoulli::new(self.p_rational).unwrap();
    }

    /// If the destination floor is None, then randomize a new destination floor.
    /// If the elevator is not on its destination floor then move toward it.  If the
    /// elevator is on its destination floor then stop it and set its destination
    /// floor to None for randomization during the next step.
    fn update_elevators(&mut self) {
        //Update the destination floors
        self.update_floors_to();
        
        //Loop through the dest floors and update the building's elevators accordingly
        for (i, floor_to) in self.floors_to.iter().enumerate() {
            //Unwrap the destination floor
            let dest_floor: usize = floor_to.unwrap();

            //Update the elevator's direction based on its destination floor
            self.building.elevators[i].update_direction(dest_floor);

            //Update the elevator
            let _new_floor_index = self.building.elevators[i].update_floor();
        }

        //Clear the destination floors if any elevators arrived at their destinations
        self.clear_floors_to();
    }
}

/// # `NearestController` struct
///
/// A `NearestController` implements the `ElevatorController` trait.  It decides each
/// elevator's direction based on the nearest destination floor among people on the
/// elevator, then the nearest floor with people waiting.
pub struct NearestController {
    pub building: Building,
    upgradable: bool
}

//Implement the NearestController interface
impl NearestController {
    /// Initialize a new NearestController given a `Building`.
    ///
    /// ## Example
    ///
    /// ```
    /// let my_building: Building = Building::from(
    ///     4_usize,
    ///     2_usize,
    ///     0.5_f64,
    ///     5.0_f64,
    ///     2.5_f64,
    ///     0.5_f64
    /// );
    /// let my_controller: NearestController = NearestController::from(my_building);
    /// ```
    pub fn from(building: Building) -> NearestController {
        //Initialize the controller
        NearestController {
            building: building,
            upgradable: false
        }
    }

    /// Initialize a new NearestController from just a building
    ///
    /// ## Example
    ///
    /// ```
    /// let my_building: Building = Building::from(
    ///     4_usize,
    ///     2_usize,
    ///     0.5_f64,
    ///     5.0_f64,
    ///     2.5_f64,
    ///     0.5_f64
    /// );
    /// let my_controller: NearestController = NearestController::from(my_building);
    /// ```
    pub fn from_building(building: Building) -> NearestController {
        //Initialize the controller
        NearestController {
            building: building,
            upgradable: false
        }
    }
}

//Implement the ElevatorController trait for the NearestController
impl ElevatorController for NearestController {
    /// Get the building belonging to the controller
    fn get_building(&mut self) -> &Building {
        &self.building
    }

    /// Mutably borrow the building belonging to the controller
    fn get_building_mut(&mut self) -> &mut Building {
        &mut self.building
    }

    /// Clone the building belonging to the controller.  Generally used when
    /// swapping controllers.
    fn clone_building(&mut self) -> Building {
        self.building.clone()
    }

    /// Return a boolean signifying whether the controller can be upgraded or
    /// not.  Always returns false, since the NearestController cannot be
    /// upgraded.
    fn can_be_upgraded(&self) -> bool {
        self.upgradable
    }

    /// Upgrade the controller given an incrementation float.  Does nothing for
    /// the NearestController since it cannot be upgraded.
    fn upgrade(&mut self, _incrementation: f64) {}

    /// Decide each elevator's direction based on the nearest destination floor among
    /// people on the elevator, then the nearest floor with people waiting.
    fn update_elevators(&mut self) {
        //Initialize a vector of decisions for the elevators
        let mut elevator_decisions: Vec<usize> = Vec::new();

        //Loop through the elevators in the building
        for elevator in self.building.elevators.iter() {
            //If stopped, check where to go next
            if elevator.stopped {
                //Find the nearest destination floor among people on the elevator
                let (nearest_dest_floor, min_dest_floor_dist): (usize, usize) = elevator.get_nearest_dest_floor();

                //If the nearest dest floor is identified, then update the elevator
                if min_dest_floor_dist != 0_usize {
                    elevator_decisions.push(nearest_dest_floor);
                    continue;
                }

                //Find the nearest waiting floor among people throughout the building
                let (nearest_wait_floor, min_wait_floor_dist): (usize, usize) = self.building.get_nearest_wait_floor(elevator.floor_on);

                //If the nearest wait floor is identified, then update the elevator
                if min_wait_floor_dist != 0_usize {
                    elevator_decisions.push(nearest_wait_floor);
                    continue;
                }
            } else {
                //If moving down and on the bottom floor, then stop
                if !elevator.moving_up && elevator.floor_on == 0_usize {
                    elevator_decisions.push(elevator.floor_on);
                    continue;
                }

                //If moving up and on the top floor, then stop
                if elevator.moving_up && elevator.floor_on == (self.building.floors.len() - 1_usize) {
                    elevator_decisions.push(elevator.floor_on);
                    continue;
                }

                //If there are people waiting on the elevator for the current floor, then stop
                if elevator.are_people_going_to_floor(elevator.floor_on) {
                    elevator_decisions.push(elevator.floor_on);
                    continue;
                }

                //If there are people waiting on the current floor, then stop
                if self.building.are_people_waiting_on_floor(elevator.floor_on) {
                    elevator_decisions.push(elevator.floor_on);
                    continue;
                }
            }

            //If we make it this far without returning, then return the current state
            elevator_decisions.push(elevator.floor_on);
        }

        //Loop through the elevator decisions and update the elevators
        for (i, decision) in elevator_decisions.iter().enumerate() {
            //Update the elevator direction
            self.building.elevators[i].update_direction(*decision);

            //Update the elevator
            let _new_floor_index = self.building.elevators[i].update_floor();
        }
    }
}