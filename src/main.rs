use glfw::{fail_on_errors,Action, Context, Key};

fn main(){
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let(mut window, events)  = glfw.create_window(640, 480, "Rust Engine", glfw::WindowMode::Windowed).unwrap();


    window.make_current();

    while !window.should_close(){
        glfw.poll_events();
        window.swap_buffers();

    }
}
