use glfw::{Action, CursorMode, Key, WindowEvent};
use luminance::{
    context::GraphicsContext,
    face_culling::FaceCulling,
    framebuffer::Framebuffer,
    linear::M44,
    pipeline::BoundTexture,
    pixel::Floating,
    render_state::RenderState,
    shader::program::{Program, Uniform},
    texture::{Dim2, Flat},
};
use luminance_derive::UniformInterface;
use luminance_glfw_custom::surface::{GlfwSurface, Surface, WindowDim, WindowOpt};
use sandbox::{
    entity::{camera::Camera, player::Player, sector::SectorManager},
    maths::{
        matrix::{Projection, Transform},
        vector::{MathVec, Vec2f, Vec3, Vec4, Vec4f},
    },
    resource::ResourceManager,
    timing::Clock,
    vertexattrib::Semantic,
};
use std::f32::consts::PI;

const VS: &'static str = include_str!("vs.glsl");
const FS: &'static str = include_str!("fs.glsl");

const BLACK: [f32; 4] = [0., 0., 0., 0.];

#[derive(UniformInterface)]
struct ShaderInterface {
    //time: Uniform<f32>,
    model_mat: Uniform<M44>,
    view_mat: Uniform<M44>,
    projection_mat: Uniform<M44>,
    terrain_texture: Uniform<&'static BoundTexture<'static, Flat, Dim2, Floating>>,
}

fn main() {
    // Informal tests of math
    let v = Vec2f::new(1., 1.);
    println!("{} -> {}", v.mag_sq(), v.mag());
    println!("{:?}", v + Vec2f::new(-1., 3.));
    println!("{:?}", v - Vec2f::new(-1., 3.));
    println!("{:?}", v * 5.0);
    println!("{:?}", -v);
    println!("{:?}", Vec4::<char>::from(('a', 'b', 'c', 'd')));
    println!("{:?}", Into::<Vec3<i32>>::into((1, 2, 3)));

    let mut v2 = Vec2f::new(0., 0.);
    v2 += (1., 2.).into();
    println!("{:?}", v2);

    v2 = Vec2f::new(0., 0.);
    v2 -= Vec2f::new(5., -5.);
    println!("{:?}", v2);

    let mut v4 = Vec4f::new(-1., 0., 1., 2.);
    v4 += Vec4f::new(1., 1., 1., 1.);
    println!("{:?}", v4);

    v4 = Vec4f::new(-1., 0., 1., 2.);
    v4 *= 3.0;
    println!("{:?}", v4);

    // Window creation
    let mut surface = GlfwSurface::new(
        WindowDim::Windowed(960, 540),
        "sandbox",
        WindowOpt::default(),
    )
    .expect("GLFW surface creation!");

    // Set the correct mouse mode
    surface
        .lib_handle_mut()
        .set_cursor_mode(CursorMode::Disabled);

    // Resource loading
    let res_mgr = ResourceManager::load_all(&mut surface);
    let terrain_tex = res_mgr.texture_mgr().terrain();

    // Shader compilation
    let (program, _) = Program::<Semantic, (), ShaderInterface>::from_strings(None, VS, None, FS)
        .expect("program creation");

    // Create a ``Player`
    let mut player = Player::at_origin();

    // Camera and projection matrix
    let mut cam = Camera::new();
    let mut proj_mat = make_proj(&surface).to_matrix();

    // Create a ``SectorManager``.
    let mut sector_mgr = SectorManager::new(terrain_tex.info());

    // Framebuffer
    let mut back_buffer = Framebuffer::back_buffer(surface.size());

    // Track frame time and window resize
    let mut resized = true;
    let mut clock = Clock::begin();
    'game: loop {
        // Handle timing
        let dt = clock.restart_seconds();

        //std::thread::sleep(Duration::from_millis(200));

        // Load pending sectors
        sector_mgr.finalize_sectors(&mut surface);

        // Poll events
        for event in surface.poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'game
                }

                WindowEvent::Key(Key::P, _, Action::Release, _) => {
                    println!("{}\t{}", 1. / dt, dt);
                }

                WindowEvent::FramebufferSize(width, height) => {
                    println!("resize!");
                    back_buffer = Framebuffer::back_buffer([width as u32, height as u32]);
                    resized = true;
                }

                _ => {}
            }
        }

        // Handle resize
        if resized {
            println!("make proj matrix!");
            proj_mat = make_proj(&surface).to_matrix();
        }

        let move_speed = 4.0 * dt as f32;

        // Movement
        if surface.lib_handle().get_key(Key::D) == Action::Press {
            player.move_x(move_speed);
        } else if surface.lib_handle().get_key(Key::A) == Action::Press {
            player.move_x(-move_speed);
        }

        if surface.lib_handle().get_key(Key::Space) == Action::Press {
            player.slide((0., move_speed, 0.));
        } else if surface.lib_handle().get_key(Key::LeftShift) == Action::Press {
            player.slide((0., -move_speed, 0.));
        }

        if surface.lib_handle().get_key(Key::S) == Action::Press {
            //player.slide((0., 0., move_speed));
            player.move_z(move_speed);
        } else if surface.lib_handle().get_key(Key::W) == Action::Press {
            //player.slide((0., 0., -move_speed));
            player.move_z(-move_speed);
        }

        let rot_speed = 0.012;

        // Pan / pitch with arrow keys

        if surface.lib_handle().get_key(Key::Left) == Action::Press {
            player.spin((0., rot_speed));
        } else if surface.lib_handle().get_key(Key::Right) == Action::Press {
            player.spin((0., -rot_speed));
        }

        if surface.lib_handle().get_key(Key::Up) == Action::Press {
            player.spin((rot_speed, 0.));
        } else if surface.lib_handle().get_key(Key::Down) == Action::Press {
            player.spin((-rot_speed, 0.));
        }

        // Pan / pitch with mouse

        let mouse_speed: f64 = 0.002;

        //println!("{:?}", surface.lib_handle().get_cursor_pos());

        let mouse_delta = surface.lib_handle().get_cursor_pos();
        surface.lib_handle_mut().set_cursor_pos(0., 0.);

        // swap x and y
        let cam_delta = (
            (-mouse_delta.1 * mouse_speed) as f32,
            (-mouse_delta.0 * mouse_speed) as f32,
        );
        player.spin(cam_delta);

        // Update camera
        cam.snap_to(&player);

        // Render frame
        surface
            .pipeline_builder()
            .pipeline(&back_buffer, BLACK, |pipe, shd_gate| {
                let bound_terrain_tex = pipe.bind_texture(terrain_tex.inner());

                shd_gate.shade(&program, |rdr_gate, iface| {
                    if resized {
                        println!("load proj matrix!");
                        iface.projection_mat.update(proj_mat.0);
                    }

                    iface.view_mat.update(cam.to_matrix().0);
                    iface.terrain_texture.update(&bound_terrain_tex);

                    for (_, sector) in &sector_mgr {
                        iface.model_mat.update(sector.translation().0);

                        let state = RenderState::default().set_face_culling(FaceCulling::default());

                        if let Some(geometry) = sector.geometry() {
                            rdr_gate.render(state, |tess_gate| {
                                tess_gate.render(&mut surface, geometry.into());
                            });
                        }
                    }
                });
            });

        // Show the backbuffer
        surface.swap_buffers();

        // Reset resize flag
        resized = false;
    }
}

fn make_proj(surface: &impl Surface) -> Projection {
    let [w, h] = surface.size();
    let (w, h) = (w as f32, h as f32);

    Projection::new(40.0 * PI / 180.0, w / h, 0.1, 1000.0)
}
