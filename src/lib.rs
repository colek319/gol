
use piston_window::*;
use std::{thread, time};

pub struct World {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    window: PistonWindow
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        let window: PistonWindow =
            WindowSettings::new("Hello Piston!", [width, height])
            .exit_on_esc(true).build().unwrap();
        let cells = Vec::new();
        World {
            width,
            height,
            cells,
            window
        }
    }

    fn log_frame(&self, event: &Event) {
        print!("\x1B[2J\x1B[1;1H");
        print!("selected: {:?} \n ", event);
    }

    pub fn start(&mut self) {
        let hundo_millis = time::Duration::from_millis(100);
        let mut event_buffer: Option<Event> = None;
        let mut buffer_counter = 0;
        while let Some(event) = self.window.next() {
            self.log_frame(&event);
            self.draw_cells(&event);
            // match buffer_counter {
            //     10 => {
            //         self.log_frame(&event_buffer);
            //         thread::sleep(hundo_millis);
            //         buffer_counter = 0;
            //     },
            //     _ => {
            //         event_buffer = Some(event);
            //         thread::sleep(hundo_millis);
            //         buffer_counter += 1;
            //         self.log_frame(&event_buffer)
            //     }
            // }
        }
    }

    fn draw_cells(event: &Event) {
        self.window.draw_2d(event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                        [0.0, 0.0, 100.0, 100.0],
                        context.transform,
                        graphics);
        });
    }
}

pub struct Cell {
    x: u32,
    y: u32,
    alive: bool,
}

impl Cell {
    fn new(x: u32, y: u32) -> Cell {
        Cell {
            x: x,
            y: y,
            alive: false,
        }
    }
}
