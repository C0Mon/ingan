use std::num::NonZeroU32;
use std::sync::Arc;
use ingan::format::Obj;
use ingan::math::Vec3;
use ingan::render::Render3D;
use softbuffer::Surface;
use winit::event::{DeviceId, ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::Key;
use winit::platform::modifier_supplement::KeyEventExtModifierSupplement;
use winit::window::{Window, WindowBuilder};

struct App {
    window: Arc<Window>,
    surface: Surface<Arc<Window>, Arc<Window>>,
    renderer: Render3D
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());
    
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let surface: Surface<Arc<Window>, Arc<Window>> = Surface::new(&context, window.clone()).unwrap();
    
    let mut renderer = Render3D::new(640, 480);
    let mut hex_model = Obj::new();
    hex_model.read_obj_file("/home/null/hex.obj");

    renderer.add_model(hex_model);

    let app = &mut App { window, surface, renderer };

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => elwt.exit(),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                    on_key_input(device_id, event, is_synthetic, app);
                },
                WindowEvent::RedrawRequested => {
                on_draw(app);
            },
            _ => {}
            }
            _ => {
                app.renderer.get_model(0).transform.rotation += Vec3::new(0.1, 0.1, 0.1);
                app.window.request_redraw();
            }
        }
    }).unwrap();
}

fn on_key_input(_device_id: DeviceId, event: KeyEvent, _is_synthetic: bool, app: &mut App) {
    if event.state == ElementState::Pressed && !event.repeat {
        match event.key_without_modifiers().as_ref() {
            Key::Character("a") => {
                app.renderer.get_model(0).transform.position.x -= 1.0;
                app.window.request_redraw();
            },
            Key::Character("d") => {
                app.renderer.get_model(0).transform.position.x += 1.0;
                app.window.request_redraw();
            },
            Key::Character("w") => {
                app.renderer.get_model(0).transform.position.y -= 1.0;
                app.window.request_redraw();
            },
            Key::Character("s") => {
                app.renderer.get_model(0).transform.position.y += 1.0;
                app.window.request_redraw();
            },
            _ => (),
        }
    }
}

fn on_draw(app: &mut App) {
    let (width, height) = {
        let size = app.window.inner_size();
        (size.width, size.height)
    };

    app.surface.resize(
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
    ).unwrap();


    let mut buffer = app.surface.buffer_mut().unwrap();

    app.renderer.width = width as usize;
    app.renderer.height = height as usize;
    app.renderer.render_scene();
    let colour_buffer = app.renderer.get_buffer();

    for (i, chunk) in colour_buffer.chunks_exact(3).enumerate() {
        let r = chunk[0] as u32;
        let g = chunk[1] as u32;
        let b = chunk[2] as u32;

        buffer[i] = (r << 16) | (g << 8) | b;
    }

    buffer.present().unwrap();
}