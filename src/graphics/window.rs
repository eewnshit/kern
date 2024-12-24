use std::{borrow::Cow, time::{Duration, Instant}};

use glfw::{Action, Context, GlfwReceiver, Key, PWindow, WindowEvent};

use crate::{graphics::geometry::square::KSquare, logger::{self, LogLevel, Logger}};

use super::geometry::line::KLine;

pub struct Window {
    glfw: glfw::Glfw,
    window_handler: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    event_callbacks: Vec<Box<dyn FnMut(&glfw::WindowEvent) + Send>>,
    width: u32,
    height: u32,
    pub cols: u32,
    pub rows: u32,
    fps_limit: Option<u32>,
    last_frame_time: Instant,
    grid_lines: Option<Vec<KLine>>,
    pub cursor_pos_x: f32,
    pub cursor_pos_y: f32,
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
            cols: 10,
            rows: 10,
            fps_limit: Some(120),
            last_frame_time: Instant::now(),
            grid_lines: None,
            cursor_pos_x: 900.0,
            cursor_pos_y: 900.0
        }
    }

    pub fn init_gl(&mut self) {
        self.window_handler.make_current();
        gl::load_with(|s| self.window_handler.get_proc_address(s) as *const _);
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
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
        self.enforce_fps_limit();
    }

    pub fn set_grid(&mut self, rows: u32, cols: u32) {
        self.cols = cols + 1;
        self.rows = rows + 1;
        self.update_grid_lines();
    }

    pub fn update_grid_lines(&mut self) {
        let mut lines = Vec::new();
        let color = [1.0, 1.0, 1.0, 1.0];
        let row_step = 2.0 / (self.rows as f32 - 1.0);
        let col_step = 2.0 / (self.cols as f32 - 1.0);

        for i in 0..self.rows {
            let y = 1.0 - i as f32 * row_step;
            lines.push(KLine::new(-1.0, y, 1.0, y, color));
        }

        for j in 0..self.cols {
            let x = -1.0 + j as f32 * col_step;
            lines.push(KLine::new(x, -1.0, x, 1.0, color));
        }

        self.grid_lines = Some(lines);
    }

    pub fn draw_grid(&self) {
        if let Some(lines) = &self.grid_lines {
            for line in lines {
                line.draw();
            }
        }
    }

    pub fn convert_window_pos_to_grid_cell(&self, pos_x: u32, pos_y: u32) -> (f32, f32) {
        let grid_size = self.get_grid_size();
        let (grid_x, grid_y) = self.convert_to_grid_position(pos_x as u32, pos_y as u32);

        let mut x_pos = grid_x + (grid_size.0 / 2.0);
        let mut y_pos = grid_y + (grid_size.1 / 2.0);

        let mut x_pos_rounded = (x_pos * (self.rows as f32 - 1.0) / 2.0).floor() / ((self.rows as f32 - 1.0) / 2.0);
        let mut y_pos_rounded = (y_pos* (self.cols as f32 - 1.0) / 2.0).floor() / ((self.cols as f32 - 1.0) / 2.0);

        if self.rows % 2 != 0 && self.cols % 2 != 0 {
            let x_cell = (grid_x / grid_size.0).floor();
            let y_cell = (grid_y / grid_size.1).floor();
            
            x_pos_rounded = (x_cell * grid_size.0) + (grid_size.0 / 2.0);
            y_pos_rounded = (y_cell * grid_size.1) + (grid_size.1 / 2.0);
            
            println!("Snapped to cell center -> x_pos: {} - y_pos: {}", x_pos_rounded, y_pos_rounded);
        }
        

        (x_pos_rounded, y_pos_rounded)
    }

    pub fn set_resizable(&mut self, resizable: bool) {
        self.window_handler.set_resizable(resizable);
    }

    pub fn convert_grid_pos_to_grid_cell(&self, norm_x: f32, norm_y: f32) -> (f32, f32) {
        let grid_size = self.get_grid_size();
    
        let x_pos = norm_x * (grid_size.0 / 2.0);
        let y_pos = norm_y * (grid_size.1 / 2.0);
    
        let x_pos_rounded = (x_pos * (self.rows as f32 - 1.0) / 2.0).floor() / ((self.rows as f32 - 1.0) / 2.0);
        let y_pos_rounded = (y_pos * (self.cols as f32 - 1.0) / 2.0).floor() / ((self.cols as f32 - 1.0) / 2.0);
    
        (x_pos_rounded, y_pos_rounded)
    }

    pub fn convert_to_grid_position(&self, pos_x: u32, pos_y: u32) -> (f32, f32) {
        let x_grid = 2.0 * (pos_x as f32) / (self.width as f32) - 1.0;
        let y_grid = 1.0 - 2.0 * (pos_y as f32) / (self.height as f32);
        (x_grid, y_grid)
    }

    pub fn get_grid_size(&self) -> (f32, f32) {
        let grid_size_x = 2.0 / (self.cols as f32 - 1.0);
        let grid_size_y = 2.0 / (self.rows as f32 - 1.0);
        (grid_size_x, grid_size_y)
    }

    pub fn is_key_down(&self, key: glfw::Key) -> bool {
        self.window_handler.get_key(key) == glfw::Action::Press
    }

    pub fn process_events_no_cb(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            
            for callback in &mut self.event_callbacks {
                callback(&event);
            }

            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe {gl::Viewport(0,0, width, height)}
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    let (grid_x, grid_y) = self.convert_window_pos_to_grid_cell(x as u32, y as u32);
                    self.cursor_pos_x = grid_x;
                    self.cursor_pos_y = grid_y;
                }
                _ => {}
            }
        }
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.fps_limit = Some(fps);
    }

    fn enforce_fps_limit(&mut self) {
        if let Some(fps) = self.fps_limit {
            let frame_duration = Duration::from_secs_f32(1.0 / fps as f32);
            let now = Instant::now();
            let elapsed = now.duration_since(self.last_frame_time);
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
            self.last_frame_time = Instant::now();
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