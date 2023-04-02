use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    fmt,
    ops::{Add, Sub},
};

type Position = (usize, usize);
pub type KeyboardLayout = [[char; 10]; 4];

pub const KEYS: KeyboardLayout = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
    ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ';'],
    ['Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '?'],
];

enum Instruction {
    Left(usize),
    Up(usize),
    Right(usize),
    Down(usize),
    Space,
    NewLine,
    Select,
    Unknown,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<instruction>L|R|U|D|S|_|N)(:(?P<count>\d*))?$").unwrap();
        }

        RE.captures(s).map_or(Instruction::Unknown, |caps| {
            let instruction = caps.name("instruction").map_or("", |m| m.as_str());
            let count = caps
                .name("count")
                .map_or(1, |m| m.as_str().parse().unwrap_or(1));

            match instruction {
                "L" => Instruction::Left(count),
                "R" => Instruction::Right(count),
                "U" => Instruction::Up(count),
                "D" => Instruction::Down(count),
                "_" => Instruction::Space,
                "N" => Instruction::NewLine,
                "S" => Instruction::Select,
                _ => Instruction::Unknown,
            }
        })
    }
}

pub struct Keyboard<'a> {
    pub keyboard_layout: KeyboardLayout,
    pub position: Position,
    pub selected_keys: &'a mut Vec<char>,
}

impl<'a> fmt::Display for Keyboard<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.selected_keys.iter().collect();
        write!(f, "{}", s)
    }
}

impl<'a> Keyboard<'a> {
    pub fn update_position(&mut self, position: Position) {
        self.position = (position.0 % 10, position.1 % 4);
    }

    fn selected_key(&mut self, key: char) {
        self.selected_keys.push(key);
    }

    fn execute(&mut self, instruction: Instruction) {
        let (x, y) = self.position;

        match instruction {
            Instruction::Left(count) => self.update_position((x.sub(count), y)),
            Instruction::Up(count) => self.update_position((x, y.sub(count))),
            Instruction::Right(count) => self.update_position((x.add(count), y)),
            Instruction::Down(count) => self.update_position((x, y.add(count))),
            Instruction::Space => self.selected_key(' '),
            Instruction::NewLine => self.selected_key('\n'),
            Instruction::Select => self.selected_key(self.keyboard_layout[y][x]),
            Instruction::Unknown => {}
        }
    }

    /// # Examples
    ///
    /// ```
    /// # let mut keyboard = keyboard_madness::Keyboard {
    /// #    keyboard_layout: keyboard_madness::KEYS,
    /// #    position: (4, 2),
    /// #    selected_keys: &mut vec![],
    /// # };
    /// keyboard.run("R,S,U,L:3,S,D,R:6,S,S,U,S");
    /// assert_eq!(keyboard.to_string(), "HELLO");
    /// ```
    pub fn run(&mut self, instructions: &str) {
        instructions
            .split(',')
            .map(|i| i.into())
            .for_each(|instruction| self.execute(instruction));
    }

    /// # Examples
    ///
    /// ```
    /// # let mut keyboard = keyboard_madness::Keyboard {
    /// #    keyboard_layout: keyboard_madness::KEYS,
    /// #    position: (4, 2),
    /// #    selected_keys: &mut vec![],
    /// # };
    /// keyboard.run("R,S,U,L:3,S,D,R:6,S,S,U,S");
    /// assert_eq!(keyboard.to_string(), "HELLO");
    /// keyboard.clear();
    /// assert_eq!(keyboard.to_string(), "");
    /// ```
    pub fn clear(&mut self) {
        self.selected_keys.truncate(0)
    }

    fn find_position(&mut self, key: char) -> Option<Position> {
        for (y, row) in self.keyboard_layout.iter().enumerate() {
            if let Some(x) = row.iter().position(|&ch| ch == key) {
                return Some((x, y));
            }
        }
        None
    }

    /// Generates a series of instructions to produce the given text using the custom keyboard.
    ///
    /// # Arguments
    ///
    /// * `text` - The input text to generate instructions for.
    /// * `keyboard_layout` - The custom keyboard layout.
    ///
    /// # Returns
    ///
    /// A `String` containing the series of instructions.
    ///
    /// # Examples
    ///
    /// ```
    /// let text = "HELLO";
    ///
    /// let mut keyboard = keyboard_madness::Keyboard {
    ///     keyboard_layout: keyboard_madness::KEYS,
    ///     position: (4, 2),
    ///     selected_keys: &mut vec![],
    /// };
    ///
    /// let instructions = keyboard.generate_instructions(text);
    ///
    /// keyboard.run(&instructions);
    /// assert_eq!(keyboard.to_string(), text);
    /// ```
    pub fn generate_instructions(&mut self, text: &str) -> String {
        let mut instructions = String::new();
        let mut position = self.position; // Starting position

        let mut char_positions = HashMap::new();
        for ch in text.chars() {
            if let std::collections::hash_map::Entry::Vacant(e) = char_positions.entry(ch) {
                let pos = self.find_position(ch);
                if let Some(pos) = pos {
                    e.insert(pos);
                }
            }
        }

        for ch in text.chars() {
            if ch == ' ' {
                instructions.push_str("_,");
                continue;
            }
            if ch == '\n' {
                instructions.push_str("N,");
                continue;
            }

            if let Some(target) = char_positions.get(&ch) {
                let dx = target.0 as i32 - position.0 as i32;
                let dy = target.1 as i32 - position.1 as i32;

                match dx.cmp(&0) {
                    std::cmp::Ordering::Greater => instructions.push_str(&format!("R:{},", dx)),
                    std::cmp::Ordering::Less => instructions.push_str(&format!("L:{},", dx.abs())),
                    _ => {}
                }

                match dy.cmp(&0) {
                    std::cmp::Ordering::Greater => instructions.push_str(&format!("D:{},", dy)),
                    std::cmp::Ordering::Less => instructions.push_str(&format!("U:{},", dy.abs())),
                    _ => {}
                }

                instructions.push_str("S,");
                position = *target;
            }
        }

        instructions.pop(); // Remove the trailing comma
        instructions
    }
}

#[cfg(test)]
mod tests {
    // this brings everything from parent's scope into this scope
    use super::*;

    #[test]
    fn test_should_select_the_starting_points_key() {
        let mut keyboard = Keyboard {
            keyboard_layout: KEYS,
            position: (4, 2),
            selected_keys: &mut vec![],
        };

        keyboard.run("S");

        assert_eq!(keyboard.to_string(), "G");
    }

    #[test]
    fn test_should_select_the_first_letter_to_the_left_of_the_starting_point() {
        let mut keyboard = Keyboard {
            keyboard_layout: KEYS,
            position: (4, 2),
            selected_keys: &mut vec![],
        };

        keyboard.run("L,S");

        assert_eq!(keyboard.to_string(), "F");
    }

    #[test]
    fn test_should_select_the_third_letter_to_the_left_of_the_starting_point() {
        let mut keyboard = Keyboard {
            keyboard_layout: KEYS,
            position: (4, 2),
            selected_keys: &mut vec![],
        };

        keyboard.run("L:3,S");

        assert_eq!(keyboard.to_string(), "S");
    }

    #[test]
    fn test_should_select_the_first_letter_to_the_right_of_the_starting_point() {
        let mut keyboard = Keyboard {
            keyboard_layout: KEYS,
            position: (4, 2),
            selected_keys: &mut vec![],
        };

        keyboard.run("R,S");

        assert_eq!(keyboard.to_string(), "H");
    }

    #[test]
    fn test_should_select_the_third_letter_to_the_right_of_the_starting_point() {
        let mut keyboard = Keyboard {
            keyboard_layout: KEYS,
            position: (4, 2),
            selected_keys: &mut vec![],
        };

        keyboard.run("R:3,S");

        assert_eq!(keyboard.to_string(), "K");
    }

    #[test]
    fn test_should_select_the_letter_above_of_the_starting_point() {
        let mut keyboard = Keyboard {
            keyboard_layout: KEYS,
            position: (4, 2),
            selected_keys: &mut vec![],
        };

        keyboard.run("U,S");

        assert_eq!(keyboard.to_string(), "T");
    }

    #[test]
    fn test_should_select_letter_below_of_the_starting_point() {
        let mut keyboard = Keyboard {
            keyboard_layout: KEYS,
            position: (4, 2),
            selected_keys: &mut vec![],
        };

        keyboard.run("D,S");

        assert_eq!(keyboard.to_string(), "B");
    }

    #[test]
    fn test_should_add_a_space_into_the_selected_keys() {
        let mut keyboard = Keyboard {
            keyboard_layout: KEYS,
            position: (4, 2),
            selected_keys: &mut vec![],
        };

        keyboard.run("S,_,S");

        assert_eq!(keyboard.to_string(), "G G");
    }

    #[test]
    fn test_should_add_a_new_line_into_the_selected_keys() {
        let mut keyboard = Keyboard {
            keyboard_layout: KEYS,
            position: (4, 2),
            selected_keys: &mut vec![],
        };

        keyboard.run("S,N,S");

        assert_eq!(keyboard.to_string(), "G\nG");
    }

    #[test]
    fn test_should_ignore_any_unknown_instructions() {
        let mut keyboard = Keyboard {
            keyboard_layout: KEYS,
            position: (4, 2),
            selected_keys: &mut vec![],
        };

        keyboard.run("S,Testing,Testing,Testing,S");

        assert_eq!(keyboard.to_string(), "GG");
    }

    #[test]
    fn test_should_select_the_correct_keys() {
        let starting_position: Position = (4, 2);
        let mut keyboard = Keyboard {
            keyboard_layout: KEYS,
            position: starting_position,
            selected_keys: &mut vec![],
        };

        keyboard.run("R,S,R:2,U,S");
        assert_eq!(keyboard.to_string(), "HI");
        keyboard.clear();
        keyboard.update_position(starting_position);

        keyboard.run("R,S,U,L:3,S,D,R:6,S,S,U,S");
        assert_eq!(keyboard.to_string(), "HELLO");
        keyboard.clear();
        keyboard.update_position(starting_position);

        keyboard.run("L:3,S,U,R:5,S,R:3,S,D:2,S");
        assert_eq!(keyboard.to_string(), "SUP?");
        keyboard.clear();
        keyboard.update_position(starting_position);

        keyboard.run("R,S,L,U,S,S,R:5,S,_,U:1,L:6,S,R:6,S,L:6,S");
        assert_eq!(keyboard.to_string(), "HTTP 404");
    }

    #[test]
    fn test_generate_instructions_hello() {
        let starting_position: Position = (4, 2);
        let mut keyboard = Keyboard {
            keyboard_layout: KEYS,
            position: starting_position,
            selected_keys: &mut vec![],
        };
        let instructions = keyboard.generate_instructions("HELLO");

        assert_eq!(instructions, "R:1,S,L:3,U:1,S,R:6,D:1,S,S,U:1,S");
    }

    #[test]
    fn test_generate_instructions_and_run_them() {
        let starting_position: Position = (4, 2);
        let text = "THIS IS A TEST";
        let mut keyboard = Keyboard {
            keyboard_layout: KEYS,
            position: starting_position,
            selected_keys: &mut vec![],
        };
        let instructions = keyboard.generate_instructions(text);

        keyboard.run(&instructions);
        assert_eq!(keyboard.to_string(), text);
    }
}
