// #[allow(unused_imports)] #[macro_use] extern crate eztrace;
use glium::glutin::{
    event::{*, VirtualKeyCode::{Escape, LShift, RShift}, ElementState::{Pressed, Released}},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::*,
};
use glium::Surface;

fn main() {
    let event_loop = EventLoop::new();
    let mut size: PhysicalSize<u32> = [100, 900].into();
    let wb = WindowBuilder::new()
        .with_inner_size(Size::Physical(size))
        .with_resizable(true)
        .with_decorations(false)
        .with_always_on_top(true)
        .with_title("portable infinity")
    ;
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let mut sneak = 0;
    let mut change = None;
    let mut since_last_set = 0;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::MainEventsCleared => {
                if since_last_set > 2 {
                    if let Some(new) = change.take() {
                        since_last_set = 0;
                        display.gl_window().window().set_cursor_position(new).ok();
                    }
                } else {
                    since_last_set += 1;
                }
            },
            Event::RedrawRequested(_) => {
                let mut s = display.draw();
                s.clear(
                    None,
                    Some((0.125, 0.125, 0.125, 1.0)),
                    false,
                    None,
                    None,
                );
                s.finish().ok();
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } | Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput { state: Pressed, virtual_keycode: Some(Escape), .. }, .. }, .. }
            => {
                *control_flow = ControlFlow::Exit;
            },

            Event::DeviceEvent { event: DeviceEvent::Key(KeyboardInput { state: Pressed, virtual_keycode: Some(RShift), .. }), .. } 
            | Event::DeviceEvent { event: DeviceEvent::Key(KeyboardInput { state: Pressed, virtual_keycode: Some(LShift), .. }), .. } => {
                if sneak == 0 {
                    sneak = 1;
                }
            },
            Event::DeviceEvent { event: DeviceEvent::Key(KeyboardInput { state: Released, virtual_keycode: Some(RShift), .. }), .. } 
            | Event::DeviceEvent { event: DeviceEvent::Key(KeyboardInput { state: Released, virtual_keycode: Some(LShift), .. }), .. } => {
                if sneak == 1 {
                    sneak = 0;
                }
            },
            Event::WindowEvent { event: WindowEvent::CursorEntered { .. }, .. } => {
            },
            Event::WindowEvent { event: WindowEvent::CursorLeft { .. }, .. } => {
                if sneak != 0 {
                    sneak = 0;
                    display.gl_window().window().set_decorations(false);
                }
            },
            Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
                if sneak > 0 {
                    sneak = 2;
                    display.gl_window().window().set_decorations(true);
                    return;
                }
                if change.is_some() { return; }
                let sx = size.width as f64;
                let sy = size.height as f64;
                let dx = if position.x < sx / 2.0 {
                    -(position.x + 1.0)
                } else {
                    sx - position.x
                };
                let dy = if position.y < sy / 2.0 {
                    -(position.y + 1.0)
                } else {
                    sy - position.y
                };
                let new = if dx.abs() < dy.abs() {
                    [dx, 0.0]
                } else if dy.abs() < dx.abs() {
                    [0.0, dy]
                } else {
                    [dx, dy]
                };
                change = Some(PhysicalPosition {
                    x: position.x + new[0],
                    y: position.y + new[1],
                });
            },
            Event::WindowEvent { event: WindowEvent::Resized(new), .. } => {
                size = new;
                sneak = 2;
            },
            Event::WindowEvent { event: WindowEvent::Moved(_), .. } => {
                sneak = 2;
            },
            /*Event::WindowEvent { event, .. } => trace!(event),*/
            _ => (),
        }
    });
}
