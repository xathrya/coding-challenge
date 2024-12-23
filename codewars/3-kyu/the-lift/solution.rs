// https://www.codewars.com/kata/58905bfa1decb981da00009e

// solution 1: 2102ms
pub enum Direction {
    UP,
    Down,
}

impl Direction {
    pub fn flip(&self) -> Self {
        match self {
            Direction::UP => Direction::Down,
            Direction::Down => Direction::UP,
        }
    }
}

pub struct Lift {
    capacity: u32,
    direction: Direction,
    on_board: Vec<u32>,
}

impl Lift {
    pub fn new(capacity: u32, direction: Direction, on_board: Vec<u32>) -> Self {
        Self {
            capacity,
            direction,
            on_board,
        }
    }
}

fn the_lift(queues: &[Vec<u32>], capacity: u32) -> Vec<u32> {
    let mut outter_result = vec![];

    // start from first floor
    outter_result.push(0);

    let lift = Lift {
        capacity,
        direction: Direction::UP,
        on_board: vec![],
    };

    fn inner(queues: &mut [Vec<u32>], mut lift: Lift, result: &mut Vec<u32>) {
        if lift.on_board.is_empty() && queues.iter().all(|i| i.is_empty()) {
            return;
        }
        for idx in 0..queues.len() {
            let floor = match lift.direction {
                Direction::UP => idx as u32,
                Direction::Down => (queues.len() - idx - 1) as u32,
            };
            let mut stopped = false;
            if lift.on_board.contains(&floor) {
                stopped = true;
                lift.on_board = lift
                    .on_board
                    .into_iter()
                    .filter(|i| *i != floor)
                    .collect::<Vec<_>>();
            }

            let waited = &mut queues[floor as usize];
            if !waited.is_empty() {
                stopped |= waited.iter().any(|i| match lift.direction {
                    Direction::UP => *i >= floor,
                    Direction::Down => *i <= floor,
                });
                // lift capacity is inf
                let into_lift = &waited
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|(_, i)| match lift.direction {
                        Direction::UP => *i > floor,
                        Direction::Down => *i < floor,
                    })
                    .take(lift.capacity as usize - lift.on_board.len())
                    .collect::<Vec<_>>();

                if waited.contains(&floor) {
                    *waited = waited
                        .iter()
                        .copied()
                        .filter(|i| *i != floor)
                        .collect::<Vec<_>>();
                }

                if !into_lift.is_empty() {
                    let into_lift_idxes = into_lift
                        .iter()
                        .copied()
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();
                    *waited = waited
                        .iter()
                        .enumerate()
                        .filter(|(idx, _)| !into_lift_idxes.contains(idx))
                        .map(|(_, item)| *item)
                        .collect::<Vec<_>>();
                    for (_, people) in into_lift {
                        lift.on_board.push(*people);
                    }
                }
            }
            if stopped && result.last() != Some(&floor) {
                result.push(floor);
            }
        }
        lift.direction = lift.direction.flip();

        inner(queues, lift, result);
    }

    let mut new_queues = queues.to_owned();
    inner(&mut new_queues, lift, &mut outter_result);
    if outter_result.last() != Some(&0) {
        outter_result.push(0);
    }
    outter_result
}