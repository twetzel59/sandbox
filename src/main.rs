use luminance::context::GraphicsContext;
use std::time::Instant;
//use luminance::face_culling::FaceCulling;
use luminance::framebuffer::Framebuffer;
use luminance::linear::M44;
use luminance::render_state::RenderState;
use luminance::shader::program::{Program, Uniform};
use luminance::tess::{Mode, TessBuilder};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_glfw_custom::event::{Action, Key, WindowEvent};
use luminance_glfw_custom::surface::{GlfwSurface, Surface, WindowDim, WindowOpt};
use sandbox::maths::matrix::*;
use sandbox::maths::vector::{MathVec, Vec2f, Vec3, Vec4, Vec4f};

// TODO: Group imports, fix globs.

const VS: &'static str = include_str!("vs.glsl");
const FS: &'static str = include_str!("fs.glsl");

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantic {
    #[sem(name = "pos", repr = "[f32; 3]", type_name = "PosAttrib")]
    Pos,

    #[sem(name = "color", repr = "[f32; 3]", type_name = "ColorAttrib")]
    Color,
}

#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantic")]
struct Vertex {
    pos: PosAttrib,
    color: ColorAttrib,
}

const VERTICES: [Vertex; 4] = [
    Vertex {
        pos: PosAttrib::new([0.0, 0.0, 0.0]),
        color: ColorAttrib::new([1., 0., 0.]),
    },
    Vertex {
        pos: PosAttrib::new([1.0, 0.0, 0.0]),
        color: ColorAttrib::new([0., 1., 0.]),
    },
    Vertex {
        pos: PosAttrib::new([0.5, 0.0, 0.87]),
        color: ColorAttrib::new([0., 0., 1.]),
    },
    Vertex {
        pos: PosAttrib::new([0.5, 0.5, 0.435]),
        color: ColorAttrib::new([1., 1., 1.]),
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

#[derive(Debug, UniformInterface)]
struct ShaderInterface {
    time: Uniform<f32>,
    model_mat: Uniform<M44>,
    view_mat: Uniform<M44>,
    projection_mat: Uniform<M44>,
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
        "Hello, world!",
        WindowOpt::default(),
    )
    .expect("GLFW surface creation!");

    let (program, _) = Program::<Semantic, (), ShaderInterface>::from_strings(None, VS, None, FS)
        .expect("program creation");

    let indexed_triangles = TessBuilder::new(&mut surface)
        .add_vertices(VERTICES)
        .set_indices(INDICES)
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();

    let mut translation = Translation::new((0., 0., 0.));
    let mut camera = Translation::new((0., 0., 0.));
    let mut proj_mat = make_proj(&surface).to_matrix();

    let mut back_buffer = Framebuffer::back_buffer(surface.size());
    let start_time = Instant::now();

    let mut resized = true;
    'game: loop {
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

        if resized {
            println!("make proj matrix!");
            proj_mat = make_proj(&surface).to_matrix();
        }

        let speed = 0.0025;

        /*
        if surface.lib_handle().get_key(Key::Left) == Action::Press {
            translation.slide((-speed, 0., 0.));
        } else if surface.lib_handle().get_key(Key::Right) == Action::Press {
            translation.slide((speed, 0., 0.));
        }

        if surface.lib_handle().get_key(Key::Down) == Action::Press {
            translation.slide((0., -speed, 0.));
        } else if surface.lib_handle().get_key(Key::Up) == Action::Press {
            translation.slide((0., speed, 0.));
        }
        */

        if surface.lib_handle().get_key(Key::D) == Action::Press {
            camera.slide((-speed, 0., 0.));
        } else if surface.lib_handle().get_key(Key::A) == Action::Press {
            camera.slide((speed, 0., 0.));
        }

        if surface.lib_handle().get_key(Key::Space) == Action::Press {
            camera.slide((0., -speed, 0.));
        } else if surface.lib_handle().get_key(Key::LeftShift) == Action::Press {
            camera.slide((0., speed, 0.));
        }

        if surface.lib_handle().get_key(Key::S) == Action::Press {
            camera.slide((0., 0., -speed));
        } else if surface.lib_handle().get_key(Key::W) == Action::Press {
            camera.slide((0., 0., speed));
        }

        surface
            .pipeline_builder()
            .pipeline(&back_buffer, BLACK, |_, shd_gate| {
                shd_gate.shade(&program, |rdr_gate, iface| {
                    let elapsed = Instant::now() - start_time;
                    let elapsed =
                        elapsed.as_secs() as f64 + (elapsed.subsec_millis() as f64 * 1e-3);

                    iface.time.update(elapsed as f32);

                    iface.model_mat.update(translation.to_matrix().0);
                    iface.view_mat.update(camera.to_matrix().0);

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

        surface.swap_buffers();

        resized = false;
    }
}

fn make_proj(surface: &impl Surface) -> Projection {
    let [w, h] = surface.size();
    let (w, h) = (w as f32, h as f32);

    Projection::new(70.0 * std::f32::consts::PI / 180.0, w / h, 0.1, 1000.0)
}
