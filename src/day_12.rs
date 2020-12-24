use std::io::BufRead;

#[derive(Debug, PartialEq)]
pub enum Cardinal {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    // Cardinals
    NORTH,
    SOUTH,
    EAST,
    WEST,
    // Mutators
    LEFT,
    RIGHT,
    FORWARD,
}

#[derive(Debug, PartialEq)]
pub struct NavigationInstruction {
    direction: Direction,
    value: isize,
}

#[derive(Debug, PartialEq)]
pub struct Waypoint {
    x: isize,
    y: isize,
}

impl Waypoint {
    pub fn new() -> Self {
        Waypoint { x: 10, y: 1 }
    }

    /// Rotates the waypoint about the given point.
    pub fn rotate(&mut self, degrees: isize, point: (isize, isize)) {
        // Calculate the position of the waypoint, relative to `point`.
        let x_relative = self.x - point.0;
        let y_relative = self.y - point.1;

        // If a counter-clockwise rotation was provided, make it clockwise.
        let degrees = modulo(degrees as f32, 360.0) as isize;

        // The degrees of rotation define how we should be updating the
        // position based on the location of the ship.
        if degrees > 0 && degrees <= 90 {
            // Clockwise 90 degrees.
            self.x = point.0 + y_relative;
            self.y = point.1 - x_relative;
        } else if degrees > 90 && degrees <= 180 {
            // Mirror the position of the waypoint.
            self.x = point.0 - x_relative;
            self.y = point.1 - y_relative;
        } else if degrees > 180 && degrees <= 270 {
            // Counter clockwise 90 degrees.
            self.x = point.0 - y_relative;
            self.y = point.1 + x_relative;
        }
    }
}

impl Default for Waypoint {
    fn default() -> Self {
        Waypoint::new()
    }
}

#[derive(Debug, PartialEq)]
pub struct Ship {
    x: isize,
    y: isize,
    orientation: isize,
    waypoint: Waypoint,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            x: 0,
            y: 0,
            orientation: 90,
            waypoint: Waypoint::new(),
        }
    }

    /// Interprets the instruction to move the ship.
    /// This is used for part one of the exercise.
    pub fn move_ship(&mut self, instruction: &NavigationInstruction) {
        match instruction.direction {
            Direction::NORTH => self.y += instruction.value,
            Direction::SOUTH => self.y -= instruction.value,
            Direction::EAST => self.x += instruction.value,
            Direction::WEST => self.x -= instruction.value,
            Direction::LEFT => {
                self.rotate(-instruction.value as isize);
            }
            Direction::RIGHT => {
                self.rotate(instruction.value as isize);
            }
            Direction::FORWARD => {
                // Translate the instruction and invoke this method again.
                let translated = NavigationInstruction {
                    direction: cardinal_to_direction(&self.cardinal()),
                    value: instruction.value,
                };
                self.move_ship(&translated);
            }
        }
    }

    /// Interprets the instruction to move the ship or the associated waypoint.
    pub fn move_waypoint(&mut self, instruction: &NavigationInstruction) {
        match instruction.direction {
            Direction::NORTH => self.waypoint.y += instruction.value,
            Direction::SOUTH => self.waypoint.y -= instruction.value,
            Direction::EAST => self.waypoint.x += instruction.value,
            Direction::WEST => self.waypoint.x -= instruction.value,
            Direction::LEFT => self.waypoint.rotate(-instruction.value, (self.x, self.y)),
            Direction::RIGHT => self.waypoint.rotate(instruction.value, (self.x, self.y)),
            Direction::FORWARD => {
                // Update the position of the ship.
                let delta_x = (self.waypoint.x - self.x) * instruction.value;
                let delta_y = (self.waypoint.y - self.y) * instruction.value;
                self.x += delta_x;
                self.y += delta_y;

                // Update the position of the waypoint accordingly as the waypoint moves
                // with the ship.
                self.waypoint.x += delta_x;
                self.waypoint.y += delta_y;
            }
        }
    }

    /// Rotates the ship by `degrees`, where positive `degrees` means rotate "right",
    /// and negative degrees mean rotate "left".
    pub fn rotate(&mut self, degrees: isize) {
        self.orientation = modulo((self.orientation + degrees) as f32, 360.0) as isize
    }

    /// Returns the distance travelled by the ship.
    pub fn distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    /// Returns a cardinal representing the orientation of the ship.
    fn cardinal(&self) -> Cardinal {
        match self.orientation {
            0..=89 => Cardinal::NORTH,
            90..=179 => Cardinal::EAST,
            180..=269 => Cardinal::SOUTH,
            270..=359 => Cardinal::WEST,
            _ => {
                panic!(
                    "Orientation '{}' is out of the expected range!",
                    self.orientation
                );
            }
        }
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self::new()
    }
}

/// Converts a cardinal to a Direction.
pub fn cardinal_to_direction(cardinal: &Cardinal) -> Direction {
    match cardinal {
        Cardinal::NORTH => Direction::NORTH,
        Cardinal::SOUTH => Direction::SOUTH,
        Cardinal::EAST => Direction::EAST,
        Cardinal::WEST => Direction::WEST,
    }
}

pub fn read_instructions<R: BufRead>(reader: &mut R) -> Vec<NavigationInstruction> {
    let mut instructions = Vec::new();
    loop {
        let mut buffer = String::new();
        reader
            .read_line(&mut buffer)
            .expect("Expected to read input from the reader.");

        let line = buffer.trim();
        if line.is_empty() {
            return instructions;
        }

        // Split the instruction
        let direction: Direction;
        let value: isize;

        let mut iter = line.chars();
        match iter.next() {
            Some('N') => {
                direction = Direction::NORTH;
            }
            Some('S') => {
                direction = Direction::SOUTH;
            }
            Some('W') => {
                direction = Direction::WEST;
            }
            Some('E') => {
                direction = Direction::EAST;
            }
            Some('L') => {
                direction = Direction::LEFT;
            }
            Some('F') => {
                direction = Direction::FORWARD;
            }
            Some('R') => {
                direction = Direction::RIGHT;
            }
            _ => {
                panic!("Unable to parse instruction: {}", line);
            }
        }

        value = iter
            .collect::<String>()
            .parse::<isize>()
            .unwrap_or_else(|e| panic!("Unable to parse instruction '{}': {}", line, e));
        instructions.push(NavigationInstruction { direction, value });
    }
}

/// Implements the modulo operation, a.k.a. floored divison.
pub fn modulo(a: f32, b: f32) -> f32 {
    a - b * (a / b).floor()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn get_test_input() -> Vec<NavigationInstruction> {
        let instructions = "F10
N3
F7
R90
F11";
        let mut reader = BufReader::new(instructions.as_bytes());
        read_instructions(&mut reader)
    }

    #[test]
    fn test_read_instructions() {
        let instructions = get_test_input();
        assert_eq!(instructions.len(), 5);
        assert_eq!(
            instructions[0],
            NavigationInstruction {
                direction: Direction::FORWARD,
                value: 10
            }
        );
    }

    #[test]
    fn test_navigation() {
        let instructions = get_test_input();
        let mut ship = Ship::new();
        for instr in instructions {
            ship.move_ship(&instr);
        }
        assert_eq!(ship.distance(), 25);
    }

    #[test]
    fn test_navigation_waypoints() {
        let instructions = get_test_input();
        let mut ship = Ship::new();
        for instr in instructions {
            ship.move_waypoint(&instr);
        }
        assert_eq!(ship.distance(), 286);
    }

    #[test]
    fn test_waypoint_rotate_90_degrees() {
        let mut waypoint = Waypoint::new();
        assert_eq!(waypoint.x, 10);
        assert_eq!(waypoint.y, 1);

        // Rotate the waypoint 90 degrees
        waypoint.rotate(90, (0, 0));
        assert_eq!(waypoint.x, 1);
        assert_eq!(waypoint.y, -10);

        // Rotate the waypoint another 90 degrees, it should now
        // mirror the original position.
        waypoint.rotate(90, (0, 0));
        assert_eq!(waypoint.x, -10);
        assert_eq!(waypoint.y, -1);

        // Rotate the waypoint another 90 degrees (equating a total rotation of 270 degrees),
        waypoint.rotate(90, (0, 0));
        assert_eq!(waypoint.x, -1);
        assert_eq!(waypoint.y, 10);

        // Rotate the waypoint another 90 degrees and it should be back at its original position.
        waypoint.rotate(90, (0, 0));
        assert_eq!(waypoint.x, 10);
        assert_eq!(waypoint.y, 1);

        // Rotate the waypoint back 90 degrees and we should be back at the 270 degrees position.
        waypoint.rotate(-90, (0, 0));
        assert_eq!(waypoint.x, -1);
        assert_eq!(waypoint.y, 10);
    }

    #[test]
    fn test_waypoint_rotate_180() {
        let mut waypoint = Waypoint::new();
        assert_eq!(waypoint.x, 10);
        assert_eq!(waypoint.y, 1);

        waypoint.rotate(180, (0, 0));
        assert_eq!(waypoint.x, -10);
        assert_eq!(waypoint.y, -1);

        waypoint.rotate(-180, (0, 0));
        assert_eq!(waypoint.x, 10);
        assert_eq!(waypoint.y, 1);
    }

    #[test]
    fn test_waypoint_rotate_270() {
        let mut waypoint = Waypoint::new();
        assert_eq!(waypoint.x, 10);
        assert_eq!(waypoint.y, 1);

        waypoint.rotate(270, (0, 0));
        assert_eq!(waypoint.x, -1);
        assert_eq!(waypoint.y, 10);

        // Reverse the rotation.
        waypoint.rotate(-270, (0, 0));
        assert_eq!(waypoint.x, 10);
        assert_eq!(waypoint.y, 1);
    }
}
