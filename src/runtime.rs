use std::char;
use std::string::String;

pub struct BfRuntime {
    pub memory: [u8; 30000],
    pub cursor: u16,
    pub loop_cursor: u16,
    pub loop_heap: String,
    pub is_in_loop: bool,
}

impl BfRuntime {
    pub fn process_bf(&mut self, bf: String) {
        self.cursor = 0;
        for c in bf.chars() {
            self.process_bf_char(c);
        }
    }

    fn process_bf_char(&mut self, char: char) {
        if self.is_in_loop {
            if char == ']' {
                self.is_in_loop = false;
                while self.memory[self.loop_cursor as usize] != 0 {
                    let chars = self.loop_heap.clone();
                    for c in chars.chars() {
                        self.process_bf_char(c);
                    }
                }
            } else {
                self.loop_heap.push(char);
            }
        } else {
            match char {
                '>' => self.cursor += 1,
                '<' => self.cursor -= 1,
                '+' => self.memory[self.cursor as usize] += 1,
                '-' => self.memory[self.cursor as usize] -= 1,
                '.' => println!("{}", self.memory[self.cursor as usize] as char),
                ',' => println!("',' isn't implemented"),
                '[' => {
                    self.is_in_loop = true;
                    self.loop_heap = "".to_owned();
                    self.loop_cursor = self.cursor.clone();
                }
                _ => (),
            }
        }
    }
}
