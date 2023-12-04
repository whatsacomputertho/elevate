//Import source modules
use crate::building::Building;
use crate::floors::Floors;
use crate::people::People;

//Implement standard/imported modules
use rand::rngs::StdRng;
use rand::distributions::{Distribution, Uniform};

/// # `ElevatorController` trait
///
/// An `ElevatorController` implementation controls the elevators of a building.
pub trait ElevatorController {
    fn get_building(&mut self) -> &Building;

    fn get_building_mut(&mut self) -> &mut Building;

    fn update_elevators(&mut self);
}

/// # `RandomController` struct
///
/// A `RandomController` implements the `ElevatorController` trait.  It randomly
/// generates destination floors for each of a building's elevators once the elevator
/// reaches its destination floor.
 pub struct RandomController {
    pub building: Building,
    floors_to: Vec<Option<usize>>,
    dst_to: Uniform<usize>,
    rng: StdRng
}

//Implement the RandomController interface
impl RandomController {
    /// Initialize a new RandomController given a `Building` and a `StdRng` (from
    /// the rand library).
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
    ///     my_rng
    /// );
    /// ```
    pub fn from(building: Building, rng: StdRng) -> RandomController {
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
            floors_to: floors_to,
            dst_to: dst_to,
            rng: rng
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

    /// If the destination floor is None, then randomize a new destination floor.
    /// If the elevator is not on its destination floor then move toward it.  If the
    /// elevator is on its destination floor then stop it and set its destination
    /// floor to None for randomization during the next step.
    fn update_elevators(&mut self) {
        //Loop through the elevators in the building
        for (i, elevator) in self.building.elevators.iter_mut().enumerate() {
            //If the destination floor for the elevator is None, then randomize it
            let floor_to: usize = match self.floors_to[i] {
                Some(x) => x as usize,
                None => self.dst_to.sample(&mut self.rng)
            };

            //If the elevator is not on its destination floor, then move toward it
            if floor_to > elevator.floor_on {
                elevator.stopped = false;
                elevator.moving_up = true;
            } else if floor_to < elevator.floor_on {
                elevator.stopped = false;
                elevator.moving_up = false;
            //If the elevator is on its destination floor, then stop and set is destination floor to None
            } else {
                elevator.stopped = true;
                self.floors_to[i] = None;
            }

            //Update the elevator
            let _new_floor_index = elevator.update_floor();
        }
    }
}

/// # `NearestController` struct
///
/// A `NearestController` implements the `ElevatorController` trait.  It decides each
/// elevator's direction based on the nearest destination floor among people on the
/// elevator, then the nearest floor with people waiting.
pub struct NearestController {
    pub building: Building
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
            building: building
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

    /// Decide each elevator's direction based on the nearest destination floor among
    /// people on the elevator, then the nearest floor with people waiting.
    fn update_elevators(&mut self) {
        //Initialize a vector of decisions for the elevators
        let mut elevator_decisions: Vec<i32> = Vec::new();

        //Loop through the elevators in the building
        for elevator in self.building.elevators.iter() {
            //If stopped, check where to go next
            if elevator.stopped {
                //Find the nearest destination floor among people on the elevator
                let (nearest_dest_floor, min_dest_floor_dist): (usize, usize) = elevator.get_nearest_dest_floor();

                //If the nearest dest floor is identified, then update the elevator
                if min_dest_floor_dist != 0_usize {
                    //Unstop the elevator and move toward the nearest dest floor
                    if nearest_dest_floor > elevator.floor_on {
                        elevator_decisions.push(1_i32);
                        continue;
                    } else {
                        elevator_decisions.push(-1_i32);
                        continue;
                    }
                }

                //Find the nearest waiting floor among people throughout the building
                let (nearest_wait_floor, min_wait_floor_dist): (usize, usize) = self.building.get_nearest_wait_floor(elevator.floor_on);

                //If the nearest wait floor is identified, then update the elevator
                if min_wait_floor_dist != 0_usize {
                    //Unstop the elevator and move toward the nearest dest floor
                    if nearest_wait_floor > elevator.floor_on {
                        elevator_decisions.push(1_i32);
                        continue;
                    } else {
                        elevator_decisions.push(-1_i32);
                        continue;
                    }
                }
            } else {
                //If moving down and on the bottom floor, then stop
                if !elevator.moving_up && elevator.floor_on == 0_usize {
                    elevator_decisions.push(0_i32);
                    continue;
                }

                //If moving up and on the top floor, then stop
                if elevator.moving_up && elevator.floor_on == (self.building.floors.len() - 1_usize) {
                    elevator_decisions.push(0_i32);
                    continue;
                }

                //If there are people waiting on the current floor, then stop
                if self.building.are_people_waiting_on_floor(elevator.floor_on) {
                    elevator_decisions.push(0_i32);
                    continue;
                }

                //If there are people waiting on the elevator for the current floor, then stop
                if elevator.are_people_going_to_floor(elevator.floor_on) {
                    elevator_decisions.push(0_i32);
                    continue;
                }
            }

            //If we make it this far without returning, then return the current state
            if elevator.stopped {
                elevator_decisions.push(0_i32);
                continue;
            } else if elevator.moving_up {
                elevator_decisions.push(1_i32);
                continue;
            } else {
                elevator_decisions.push(-1_i32);
                continue;
            }
        }

        //Loop through the elevator decisions and update the elevators
        for (i, decision) in elevator_decisions.iter().enumerate() {
            //Update the elevator direction
            if *decision > 0_i32 {
                self.building.elevators[i].stopped = false;
                self.building.elevators[i].moving_up = true;
            } else if *decision < 0_i32 {
                self.building.elevators[i].stopped = false;
                self.building.elevators[i].moving_up = false;
            } else {
                self.building.elevators[i].stopped = true;
            }

            //Update the elevator
            let _new_floor_index = self.building.elevators[i].update_floor();
        }
    }
}