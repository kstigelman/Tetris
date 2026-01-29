use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;
use crate::game::Game;
use crate::board::Board;

use rand::{thread_rng, Rng};

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Down,
    Clockwise,
    CounterClockwise,
}


#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub x: i32,
    pub y: i32
}

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub blocks: [Block; 4],
    pub direction: Direction,
}

impl Tetromino {
    pub fn new (x: i32, y: i32) -> Tetromino {

        let mut block_positions: [(i32, i32); 4];
        let mut color: Color;

        let mut blocks= [ Block{x: 0, y: 0}, Block{x: 0, y: 0}, Block{x: 0, y: 0}, Block{x: 0, y: 0} ];

        let mut rng = thread_rng ();

        match rng.gen_range (1, 8) {
            1 => {
                blocks[0] = Block {x: 0, y: -1};
                blocks[1] = Block {x: 0, y: 0};
                blocks[2] = Block {x: 0, y: 1};
                blocks[3] = Block {x: 0, y: 2};

                color = [1.0, 0.0, 0.0, 1.0];
            }
            2 => {
                blocks[0] = Block {x: 0, y: 0};
                blocks[1] = Block {x: 0, y: 1};
                blocks[2] = Block {x: 1, y: 0};
                blocks[3] = Block {x: 1, y: 1};

                color = [0.0, 1.0, 0.0, 1.0];
            }
            3 => {
                blocks[0] = Block {x: 0, y: -1};
                blocks[1] = Block {x: 0, y: 0};
                blocks[2] = Block {x: 0, y: 1};
                blocks[3] = Block {x: 1, y: 1};

                color = [0.0, 0.0, 1.0, 1.0];
            }
            4 => {
                blocks[0] = Block {x: 0, y: -1};
                blocks[1] = Block {x: 0, y: 0};
                blocks[2] = Block {x: 0, y: 1};
                blocks[3] = Block {x: -1, y: 1};

                color = [1.0, 1.0, 0.0, 1.0];
            }
            5 => {
                blocks[0] = Block {x: -1, y: 0};
                blocks[1] = Block {x: 0, y: 0};
                blocks[2] = Block {x: 0, y: 1};
                blocks[3] = Block {x: 1, y: 1};

                color =[1.0, 0.0, 1.0, 1.0];
            }
            6 => {
                blocks[0] = Block {x: 1, y: 0};
                blocks[1] = Block {x: 0, y: 0};
                blocks[2] = Block {x: 0, y: 1};
                blocks[3] = Block {x: -1, y: 1};

                color =[0.0, 1.0, 1.0, 1.0];
            }
            7 => {
                blocks[0] = Block {x: 1, y: 0};
                blocks[1] = Block {x: 0, y: 0};
                blocks[2] = Block {x: -1, y: 0};
                blocks[3] = Block {x: 0, y: 1};

                color =[0.25, 0.75, 0.25, 1.0];
            }
            _ => {
                color =[1.0, 0.0, 0.0, 1.0];
            }
        };

        Tetromino {
            x,
            y,
            direction: Direction::Down,
            blocks,
            color,
        }
    }
    pub fn create_tetromino (&mut self) {
        
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.blocks {
            draw_block(self.color, block.x, block.y, con, g);
        }
    }
    pub fn move_down (&mut self, width: i32, height: i32, board: &mut Vec<Color>) -> bool {
        let mut can_move = true;

        for block in self.blocks {
            let shifted_block = self.get_new_block_pos (block, Direction::Down);
            
            if shifted_block.1 + self.y >= height {
                can_move = false;
                break;
            }
            
            let current_index = (shifted_block.0 + self.x) + (width * (shifted_block.1 + self.y));

            if board[current_index as usize] != [0.0, 0.0, 0.0, 0.0] {
                can_move = false;
                break;
            }
            
        }
        if can_move {
            self.y += 1;
        }
        
        can_move
    }
    pub fn can_rotate (&mut self, direction: Direction, width: i32, height: i32, board: &mut Vec<Color>) -> bool {
        let mut rotatable = true;
        let mut i = 0;
        for block in self.blocks {
            i += 1;

            let rotated_block = self.get_new_block_pos (block, direction);

            if rotated_block.0 + self.x < 0 || rotated_block.0 + self.x > width {
                rotatable = false;
                break;
            }
            
            
            
            if self.is_contained_in_self (Block { x: rotated_block.0, y: rotated_block.1}) {
                continue;
            }
            // We need to verify that the rotated block is not included in SELF.
            //if rotated_block.x == block.x && block.y == rotated_block.y
            let index = (rotated_block.0 + self.x)+ (width * (rotated_block.1 + self.y));
            if board[index as usize] != [0.0, 0.0, 0.0, 0.0] {
                rotatable = false;
                break;
            }
        

        }
        rotatable
    }
    pub fn is_contained_in_self (&mut self, block: Block) -> bool {
        let mut found_self = false;

        for b in self.blocks {
            if b.x == block.x && b.y == block.y {
                found_self = true;
                break;
            }
        }
        found_self
    }

    
    pub fn move_to_side (&mut self, direction: Direction, width: i32, height: i32, board: &mut Vec<Color>) -> bool {
        let mut can_move = true;

        match direction {
            Direction::Left => {
                for block in self.blocks {
                    let shifted_block = self.get_new_block_pos (block, Direction::Left);

                    if shifted_block.0 + self.x < 0 {
                        can_move = false;
                        break;
                    }
                    let current_index = (shifted_block.0 + self.x) + (width * (shifted_block.1 + self.y));

                    if current_index < 0 || current_index > width * height {
                        can_move = false;
                        break;
                    }
                    if board[current_index as usize] != [0.0, 0.0, 0.0, 0.0] {
                        can_move = false;
                        break;
                    }
                }
                if can_move {
                    self.x -= 1;
                }
            }
            Direction::Right => {
                for block in self.blocks {
                    let shifted_block = self.get_new_block_pos (block, Direction::Right);

                    if shifted_block.0 + self.x >= width {
                        can_move = false;
                        break;
                    }
                    let current_index = (shifted_block.0 + self.x) + (width * (shifted_block.1 + self.y));
                    if current_index < 0 || current_index > width * height {
                        can_move = false;
                        break;
                    }
                    if board[current_index as usize] != [0.0, 0.0, 0.0, 0.0] {
                        can_move = false;
                        break;
                    }
                }
                if can_move {
                    self.x += 1;
                }
            }
            _ => {

            }
        }
        can_move
    }

    pub fn get_new_block_pos (&mut self, mut block: Block, direction: Direction) -> (i32, i32) {
        let mut new_position = (0, 0);
        match direction {
            Direction::Down => {
                new_position = (block.x, block.y + 1);
            }
            Direction::Left => {
                new_position = (block.x - 1, block.y);
            }
            Direction::Right => {
                new_position = (block.x + 1, block.y);
            }
            Direction::Clockwise => {
                //let temp = block.y;
                if block.y >= 0 {
                    //block.y = -block.x;
                    new_position = (block.y, -block.x);
                }
                else {
                    //block.y = block.x;
                    new_position = (block.y, block.x);
                }
                //block.x = temp;
            }
            Direction::CounterClockwise => {
                //let temp = block.x;
                if block.y > 0 {
                    new_position = (-block.y, block.x);
                    //block.x = -block.y;
                }
                else {
                    //block.x = block.y;
                    new_position = (block.y, block.x);
                }
                //block.y = temp;
            }
        }
        new_position
    }

    pub fn rotate (&mut self, direction: Direction) {
        for block in self.blocks.iter_mut() {
            match direction {
                Direction::Clockwise => {
                    let mut temp = block.y;

                    block.y = block.x;
                    block.x = -temp;
                }
                Direction::CounterClockwise => {
                    let mut temp = block.x;

                    block.x = block.y;
                    block.y = -temp;
                }
                _ => {

                }
            }
        }
    }
    pub fn get_blocks (&mut self) -> [Block; 4] {
        self.blocks
    }
}