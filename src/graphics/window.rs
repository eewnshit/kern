use glfw::{Action, Context, GlfwReceiver, Key, PWindow, WindowEvent};

pub struct Window {
    glfw: glfw::Glfw,
    window_handler: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
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
            events
        }
    }

    pub fn init_gl(&mut self) {
        self.window_handler.make_current();
        gl::load_with(|s| self.window_handler.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handler.should_close()       
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handler.swap_buffers();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe {gl::Viewport(0,0, width, height)}
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handler.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}