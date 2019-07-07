use luminance::{
    context::GraphicsContext,
    framebuffer::Framebuffer,
    linear::M44,
    pipeline::BoundTexture,
    pixel::Floating,
    render_state::RenderState,
    shader::program::{Program, Uniform},
    tess::{Mode, TessBuilder},
    texture::{Dim2, Flat},
};
use luminance_derive::UniformInterface;
use luminance_glfw_custom::{
    event::{Action, Key, WindowEvent},
    surface::{GlfwSurface, Surface, WindowDim, WindowOpt},
};
use sandbox::{
    entity::camera::Camera,
    maths::{
        matrix::{Projection, Transform, IDENTITY},
        vector::{MathVec, Vec2f, Vec3, Vec4, Vec4f},
    },
    resource::ResourceManager,
    vertexattrib::{PosAttrib, Semantic, UvAttrib, VoxelVertex},
};
use std::{f32::consts::PI, time::Instant};

const VS: &'static str = include_str!("vs.glsl");
const FS: &'static str = include_str!("fs.glsl");

const VERTICES: [VoxelVertex; 4] = [
    VoxelVertex {
        pos: PosAttrib::new([0.0, 0.0, 0.0]),
        uv: UvAttrib::new([1., 0.]),
    },
    VoxelVertex {
        pos: PosAttrib::new([1.0, 0.0, 0.0]),
        uv: UvAttrib::new([0., 0.]),
    },
    VoxelVertex {
        pos: PosAttrib::new([0.5, 0.0, 0.87]),
        uv: UvAttrib::new([0., 1.]),
    },
    VoxelVertex {
        pos: PosAttrib::new([0.5, 0.5, 0.435]),
        uv: UvAttrib::new([1., 1.]),
    },
];

const INDICES: [u32; 12] = [
    2, 1, 0, // bottom
    3, 1, 0, // side
    3, 1, 2, // side
    3, 2, 0, //side
];

const BLACK: [f32; 4] = [0., 0., 0., 0.];
//const WHITE: [f32; 4] = [1., 1., 1., 0.];

#[derive(UniformInterface)]
struct ShaderInterface {
    //time: Uniform<f32>,
    model_mat: Uniform<M44>,
    view_mat: Uniform<M44>,
    projection_mat: Uniform<M44>,
    terrain_texture: Uniform<&'static BoundTexture<'static, Flat, Dim2, Floating>>,
}

fn main() {
    // Informal tests of math.
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

    let mut surface = GlfwSurface::new(
        WindowDim::Windowed(960, 540),
        "sandbox",
        WindowOpt::default(),
    )
    .expect("GLFW surface creation!");

    let res_mgr = ResourceManager::load_all(&mut surface);
    let terrain_tex = res_mgr.texture_mgr().terrain();

    let (program, _) = Program::<Semantic, (), ShaderInterface>::from_strings(None, VS, None, FS)
        .expect("program creation");

    let indexed_triangles = TessBuilder::new(&mut surface)
        .add_vertices(VERTICES)
        .set_indices(INDICES)
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();

    let mut cam = Camera::at_origin();
    let mut proj_mat = make_proj(&surface).to_matrix();

    let mut back_buffer = Framebuffer::back_buffer(surface.size());
    let start_time = Instant::now();

    let mut resized = true;
    'game: loop {
        // Poll events
        for event in surface.poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'game
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

        let move_speed = 0.0025;

        // Movement
        if surface.lib_handle().get_key(Key::D) == Action::Press {
            cam.translation.slide((move_speed, 0., 0.));
        } else if surface.lib_handle().get_key(Key::A) == Action::Press {
            cam.translation.slide((-move_speed, 0., 0.));
        }

        if surface.lib_handle().get_key(Key::Space) == Action::Press {
            cam.translation.slide((0., move_speed, 0.));
        } else if surface.lib_handle().get_key(Key::LeftShift) == Action::Press {
            cam.translation.slide((0., -move_speed, 0.));
        }

        if surface.lib_handle().get_key(Key::S) == Action::Press {
            cam.translation.slide((0., 0., move_speed));
        } else if surface.lib_handle().get_key(Key::W) == Action::Press {
            cam.translation.slide((0., 0., -move_speed));
        }

        let rot_speed = 0.0030;

        // Pan / pitch
        if surface.lib_handle().get_key(Key::Left) == Action::Press {
            cam.rotation.spin((0., rot_speed));
        } else if surface.lib_handle().get_key(Key::Right) == Action::Press {
            cam.rotation.spin((0., -rot_speed));
        }

        if surface.lib_handle().get_key(Key::Up) == Action::Press {
            cam.rotation.spin((rot_speed, 0.));
        } else if surface.lib_handle().get_key(Key::Down) == Action::Press {
            cam.rotation.spin((-rot_speed, 0.));
        }

        // Render frame
        surface
            .pipeline_builder()
            .pipeline(&back_buffer, BLACK, |pipe, shd_gate| {
                let bound_terrain_tex = pipe.bind_texture(terrain_tex.inner());
                
                shd_gate.shade(&program, |rdr_gate, iface| {
                    //let elapsed = Instant::now() - start_time;
                    //let elapsed =
                    //    elapsed.as_secs() as f64 + (elapsed.subsec_millis() as f64 * 1e-3);

                    //iface.time.update(elapsed as f32);

                    iface.model_mat.update(IDENTITY.0);
                    iface.view_mat.update(cam.to_matrix().0);
                    iface.terrain_texture.update(&bound_terrain_tex);

                    if resized {
                        println!("load proj matrix!");
                        //iface.projection_mat.update(IDENTITY.0);
                        iface.projection_mat.update(proj_mat.0);
                    }

                    let state = RenderState::default();
                    //.set_face_culling(FaceCulling::default());

                    rdr_gate.render(state, |tess_gate| {
                        tess_gate.render(&mut surface, (&indexed_triangles).into());
                    });
                });
            });

        // Show the backbuffer
        surface.swap_buffers();

        resized = false;
    }
}

fn make_proj(surface: &impl Surface) -> Projection {
    let [w, h] = surface.size();
    let (w, h) = (w as f32, h as f32);

    Projection::new(40.0 * PI / 180.0, w / h, 0.1, 1000.0)
}
