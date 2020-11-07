#![crate_name = "sdl_game_of_life"]

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Running,
    Paused,
}

impl State {
    pub fn toggle(&self) -> State {
        match self {
            State::Running => State::Paused,
            State::Paused => State::Running
        }
    }
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    state: State,
}

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
impl Universe {
    
    /// create a new universe populated with dead cells that is height x width big
    /// 
    /// # Arguments
    /// 
    /// * `height` - An unsigned 32 bit int representing the height of the universe
    /// * `width` - An unsigned 32 bit int representing the width of the universe
    /// ```
    /// use sdl_game_of_life::Universe;
    /// let universe = Universe::new(64, 64);
    /// ```
    pub fn new(height: u32, width: u32) -> Universe  {
        let cells = (0..(width * height))
            .map(|i| {
                if i % 3 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        
        Universe{
            height,
            width,
            cells,
            state: State::Paused,
        }
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// get the number of live neighbors
    fn get_live_neighbors(&self, row: u32, col: u32) -> u8 {
        let mut live_count = 0;
        
        for row_modifier in [self.height - 1, 0, 1].iter().cloned() {
            for col_modifier in [ self.width - 1, 0, 1].iter().cloned() {
                if row_modifier == 0 && col_modifier == 0 {
                    continue;
                }

                let neighbor_row = (row + row_modifier) % self.height;
                let neighbor_col = (col + col_modifier) % self.width;
                let index = self.get_index(neighbor_row, neighbor_col);
                live_count += self.cells[index] as u8; // increment if alive, because alive = 1

            }
        }

        live_count
    } 

    /// Moves the state of the game by one tick
    pub fn tick(& mut self) {

        match self.state {
            State::Paused => return,
            State::Running => {
                let mut next = self.cells.clone();
        
                for row in 0..self.height {
                    for col in 0..self.width {
                        let index = self.get_index(row, col);
                        let live_neighbors = self.get_live_neighbors(row, col);
        
                        next[index] = match (live_neighbors, self.cells[index]){
                            // if neighbors are less than two, then cell dies
                            (x, Cell::Alive) if x < 2 => Cell::Dead,
                            // if neighbors more than tree, then cell dies
                            (x, Cell::Alive) if x > 3 => Cell::Dead,
                            // if neighbors 2 or 3, then cell stays alive
                            (2, Cell::Alive) | (3, Cell::Alive) => Cell::Alive,
                            // if neighbors exactly 3, then revive
                            (3, Cell::Dead) => Cell::Alive,
                            // stay the same for other states
                            (_, otherwise) => otherwise,
                        };
                    }
                }
                
                self.cells = next;
            }
        }

    }



    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: u32, y: u32) {
        let mut current_y: u32 = y;
        // currently the size of the cell is 10x10 pixels with 2 pixel border
        for row in self.cells.as_slice().chunks(self.width as usize) {
            let mut current_x = x;
            for &cell in row {
                if cell == Cell::Alive {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.fill_rect(Rect::new(current_x as i32, current_y as i32, 10, 10)).unwrap();
                }

                current_x += 12;
            }

            current_y += 12;
        }
    }
    
    pub fn toggle_state(&mut self) {
        self.state = match self.state {
            State::Running => State::Paused,
            State::Paused => State::Running,
        }
    }
}
