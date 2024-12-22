use std::borrow::Cow;

use glfw::{Action, Context, GlfwReceiver, Key, PWindow, WindowEvent};

use super::geometry::line::KLine;

pub struct Window {
    glfw: glfw::Glfw,
    pub window_handler: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    event_callbacks: Vec<Box<dyn FnMut(&glfw::WindowEvent) + Send>>
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
            event_callbacks: Vec::new()
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
        self.window_handler.swap_buffers();
    }

    pub fn draw_grid(&mut self, rows: i32, cols: i32) {
        let color = [1.0, 1.0, 1.0, 1.0]; 

        for i in 0..rows {
            let y = 1.0 - (i as f32) * (2.0 / (rows as f32 - 1.0));
            let line = KLine::new(-1.0, y, 1.0, y, color); 
            line.draw();
        }
        
        for j in 0..cols {
            let x = -1.0 + (j as f32) * (2.0 / (cols as f32 - 1.0));
            let line = KLine::new(x, -1.0, x, 1.0, color);
            line.draw();
        }
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