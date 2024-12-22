use std::borrow::Cow;

use glfw::{Action, Context, GlfwReceiver, Key, PWindow, WindowEvent};

use crate::{graphics::geometry::square::KSquare, logger::{LogLevel, Logger}};

use super::geometry::line::KLine;

pub struct Window {
    glfw: glfw::Glfw,
    window_handler: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    event_callbacks: Vec<Box<dyn FnMut(&glfw::WindowEvent) + Send>>,
    width: u32,
    height: u32,
    pub cols: Option<u32>,
    pub rows: Option<u32>
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW windowed");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Window {
            glfw,
            window_handler: window,
            events,
            event_callbacks: Vec::new(),
            width,
            height,
            cols: None,
            rows: None
        }
    }

    pub fn init_gl(&mut self) {
        self.window_handler.make_current();
        gl::load_with(|s| self.window_handler.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handler.should_close()       
    }

    pub fn clear(&self, background_color: [f32; 4]) {
        unsafe {
            gl::ClearColor(background_color[0], background_color[1], background_color[2], background_color[3]);
            gl::Clear(gl::COLOR_BUFFER_BIT)
        }
    }

    pub fn update(&mut self) {
        self.process_events_no_cb();
        self.glfw.poll_events();
        self.window_handler.set_cursor_pos_polling(true);
        self.window_handler.swap_buffers();
    }

    pub fn set_grid(&mut self, rows: u32, cols: u32) {
        self.cols = Some(cols);
        self.rows = Some(rows)
    }

    pub fn draw_grid(&mut self) {
        let color = [1.0, 1.0, 1.0, 1.0]; 

        match (self.cols, self.rows) {
            (None, None) => {
                Logger::log(LogLevel::Error, "For draw_grid() first use set_grid(u32, u32)!");
                panic!("DEFINE A GRID FIRST");
            },
            _ => {}
        }

        for i in 0..self.rows.unwrap() {
            let y = 1.0 - (i as f32) * (2.0 / (self.rows.unwrap() as f32 - 1.0));
            let line = KLine::new(-1.0, y, 1.0, y, color); 
            line.draw();
        }
        
        for j in 0..self.cols.unwrap() {
            let x = -1.0 + (j as f32) * (2.0 / (self.cols.unwrap() as f32 - 1.0));
            let line = KLine::new(x, -1.0, x, 1.0, color);
            line.draw();
        }
    }

    pub fn convert_window_pos_to_grid_cell(&self, pos_x: u32, pos_y: u32) -> (f32, f32) {
        let grid_size = self.get_grid_size();
        let (grid_x, grid_y) = self.convert_to_grid_position(pos_x as u32, pos_y as u32);

        let x_pos = grid_x + (grid_size.0 / 2.0);
        let y_pos = grid_y + (grid_size.1 / 2.0);
        let x_pos_rounded = (x_pos * (self.rows.unwrap() as f32 - 1.0) / 2.0).floor() / ((self.rows.unwrap() as f32 - 1.0) / 2.0);
        let y_pos_rounded = (y_pos * (self.cols.unwrap() as f32 - 1.0) / 2.0).floor() / ((self.cols.unwrap() as f32 - 1.0) / 2.0);
        (x_pos_rounded, y_pos_rounded)
    }

    pub fn convert_grid_pos_to_grid_cell(&self, norm_x: f32, norm_y: f32) -> (f32, f32) {
        let grid_size = self.get_grid_size();
    
        let x_pos = norm_x * (grid_size.0 / 2.0);
        let y_pos = norm_y * (grid_size.1 / 2.0);
    
        let x_pos_rounded = (x_pos * (self.rows.unwrap() as f32 - 1.0) / 2.0).floor() / ((self.rows.unwrap() as f32 - 1.0) / 2.0);
        let y_pos_rounded = (y_pos * (self.cols.unwrap() as f32 - 1.0) / 2.0).floor() / ((self.cols.unwrap() as f32 - 1.0) / 2.0);
    
        (x_pos_rounded, y_pos_rounded)
    }

    pub fn convert_to_grid_position(&self, pos_x: u32, pos_y: u32) -> (f32, f32) {
        let x_grid = 2.0 * (pos_x as f32) / (self.width as f32) - 1.0;
        let y_grid = 1.0 - 2.0 * (pos_y as f32) / (self.height as f32);
        (x_grid, y_grid)
    }

    pub fn get_grid_size(&self) -> (f32, f32) {
        let grid_size_x = 2.0 / (self.cols.unwrap_or(1) as f32 - 1.0);
        let grid_size_y = 2.0 / (self.rows.unwrap_or(1) as f32 - 1.0);
        (grid_size_x, grid_size_y)
    }

    pub fn is_key_down(&self, key: glfw::Key) -> bool {
        self.window_handler.get_key(key) == glfw::Action::Press
    }

    pub fn process_events_no_cb(&mut self) {
        let mut qGrid = KSquare::new(3.0, 3.0, self.get_grid_size().0, [0.0, 1.0, 0.0, 1.0]);
        for (_, event) in glfw::flush_messages(&self.events) {
            
            for callback in &mut self.event_callbacks {
                callback(&event);
            }

            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe {gl::Viewport(0,0, width, height)}
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    qGrid.x = self.convert_window_pos_to_grid_cell(x as u32, y as u32).0;
                    qGrid.y = self.convert_window_pos_to_grid_cell(x as u32, y as u32).1;
                    qGrid.draw();
                }
                _ => {}
            }
        }
    }

    pub fn process_events<F>(&mut self, mut callback: F)
    where
        F: FnMut(&glfw::WindowEvent),
    {
        for (_, event) in glfw::flush_messages(&self.events) {
            callback(&event);

            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handler.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}