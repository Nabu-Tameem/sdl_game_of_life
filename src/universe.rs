use crate::cell::Cell;

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    running: bool,
    x_offset: i32,
    y_offset: i32,
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
            running: false,
            x_offset: 0,
            y_offset: 0,
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

        match self.running {
            false => return,
            true => {
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

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {

        let mut current_y = 0 + self.y_offset;
        // currently the size of the cell is 10x10 pixels with 2 pixel border
        for row in self.cells.as_slice().chunks(self.width as usize) {
            let mut current_x = 0 + self.x_offset;
            for &cell in row {
                if cell == Cell::Alive {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                canvas.fill_rect(Rect::new(current_x, current_y, 10, 10)).unwrap();


                current_x += 12;
            }

            current_y += 12;
        }
    }
    
    pub fn toggle_state(&mut self) {
        self.running ^= true;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn run(&mut self) {
        self.running = true;
    }

    pub fn shift(&mut self, x: i32, y: i32) {
        self.x_offset += x;
        self.y_offset += y;
    }

    fn get_by_coordinates(&self, x: i32, y: i32) -> Option<usize> {
        // TODO use dynamic cell size to get coordinates when scaling
        let x_size = 12;
        let y_size = 12;
        // Take the cell size and spacing, multiply by it's index
        
        let y_index = (((y - self.y_offset) as f32) / (y_size as f32)).floor();
        let x_index = (((x - self.x_offset) as f32) / (x_size as f32)).floor();

        if y_index < 0.0 || x_index < 0.0 || y_index >= self.height as f32 || x_index >= self.width as f32 {
            return None;
        }

        println!("x: {} y: {} x index: {} y index {}", x, y, x_index, y_index);

        Some((y_index as u32 * self.width + x_index as u32) as usize)

    }

    pub fn kill(&mut self, x: i32, y: i32) {
        let cell_index = match self.get_by_coordinates(x, y) {
            Some(index) => index,
            None => return
        };
        self.cells[cell_index] = Cell::Dead;
    }

    pub fn revive(&mut self, x: i32, y: i32) {
        let cell_index = match self.get_by_coordinates(x, y) {
            Some(index) => index,
            None => return
        };
        self.cells[cell_index] = Cell::Alive
    }
}
