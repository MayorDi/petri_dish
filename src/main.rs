extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    unsafe {
        let sleeping = std::time::Duration::from_nanos(1_000_000_000 / 60);
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw
            .create_window(1200, 600, "Petri dish", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        gl::load_with(|s| window.get_proc_address(s) as *const _);
        gl::Viewport::load_with(|s| window.get_proc_address(s) as *const _);

        while !window.should_close() {
            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            

            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event);
            }

            window.swap_buffers();
            std::thread::sleep(sleeping);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
