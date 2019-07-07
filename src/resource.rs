//! Provides utilities for managing media resources.
//!
//! Currently, only textures are managed by this implementation,
//! but in the future, sounds or models could be loaded as well.

use luminance::{
    context::GraphicsContext,
    pixel::RGB32F,
    texture::{Dim2, Flat, MagFilter, MinFilter, Sampler, Texture},
};
use png::{self, Decoder, OutputInfo};
use std::{
    fs::File,
    path::{Path, PathBuf},
    rc::Rc,
};

const RESOURCE_PATH: &str = "./res";

/// The master resource manager.
///
/// A ``ResourceManager`` has subordinate resource manangers
/// that load and store various types of media.
pub struct ResourceManager {
    textures: TextureManager,
}

impl ResourceManager {
    /// Load all media and return a ``ResourceManager``
    /// that owns the loaded resources.
    ///
    /// ``ctx`` is a GraphicsContext that acts as a handle
    /// for the current OpenGL state. Usually, the GLFW
    /// window will be supplied for this parameter.
    pub fn load_all<C: GraphicsContext>(ctx: &mut C) -> ResourceManager {
        ResourceManager {
            textures: TextureManager::load_all(ctx),
        }
    }
    
    /// Return a reference to the ``TextureManager`` for this
    /// parent resource manager.
    pub fn texture_mgr(&self) -> &TextureManager {
        &self.textures
    }
}

/// A texture manager.
///
/// This ``struct`` owns all textures that are used by the
/// game during runtime.
///
/// Each texture stored in the manager is behind a ``Rc``.
/// Obtaining a reference to the texture thus requires a
/// reference count update, but it is not horribly expensive
/// since the update is non-atomic.
pub struct TextureManager {
    terrain_tex: Rc<Texture2D>,
}

impl TextureManager {
    const TEXTURE_PATH: &'static str = "tex";

    const TERRAIN: &'static str = "terrain.png";

    /// Load all textures and store them in a new
    /// ``TextureManager`` instance.
    ///
    /// For the ``ctx`` parameter, an instance of
    /// ``GraphicsContext`` must be specified. This
    /// parameter represents the OpenGL context.
    /// The current GLFW window normally should be
    /// supplied for ``ctx``.
    pub fn load_all<C: GraphicsContext>(ctx: &mut C) -> TextureManager {
        let tex_path: PathBuf = [RESOURCE_PATH, Self::TEXTURE_PATH].iter().collect();

        let terrain_path = tex_path.join(Self::TERRAIN);

        let mut sampler = Sampler::default();
        sampler.min_filter = MinFilter::Nearest;
        sampler.mag_filter = MagFilter::Nearest;

        TextureManager {
            terrain_tex: Rc::new(Texture2D::with_path(ctx, terrain_path, &sampler)),
        }
    }
    
    pub fn terrain(&self) -> Rc<Texture2D> {
        Rc::clone(&self.terrain_tex)
    }
}

/// The type of a low-level simple 2D texture.
///
/// This is an alias to the underlying ``luminance``
/// texture. If you are not talking directly to the
/// graphics API, use ``Texture2D`` instead.
pub type Tex2DInner = Texture<Flat, Dim2, RGB32F>;

/// An individual 2D texture.
///
/// A texture is composed of a ``luminance`` texture and
/// an ``info`` field that contains size and format
/// metadata.
pub struct Texture2D {
    inner: Tex2DInner,
    info: OutputInfo,
}

impl Texture2D {
    /// Create a new 2D texture with the given
    /// ``luminance`` ``Texture`` and ``OutputInfo``.
    pub fn new(inner: Tex2DInner, info: OutputInfo) -> Texture2D {
        Texture2D { inner, info }
    }

    /// Create a new 2D texture by loading the texture
    /// data from ``file``.
    ///
    /// The ``sampler`` is used by ``luminance`` and
    /// customizes how the image is sampled by OpenGL.
    pub fn from_file<C>(ctx: &mut C, file: File, sampler: &Sampler) -> Texture2D
    where
        C: GraphicsContext,
    {
        let (inner, info) = load_png(ctx, file, sampler);
        Self::new(inner, info)
    }

    /// Create a new 2D texture by loading the texture
    /// data from the file located at ``path``.
    ///
    /// The ``sampler`` is passed on to ``luminance``
    /// to control how the image is sampled by the
    /// OpenGL backend.
    pub fn with_path<C>(ctx: &mut C, path: impl AsRef<Path>, sampler: &Sampler) -> Texture2D
    where
        C: GraphicsContext,
    {
        let file = File::open(path).unwrap();
        Self::from_file(ctx, file, sampler)
    }
    
    /// Return the low-level inner ``luminance`` texture.
    pub fn inner(&self) -> &Tex2DInner {
        &self.inner
    }
}

/// Load a PNG image from the given ``File``.
///
/// The ``sampler`` parameter allows the caller
/// to customize how the image data is sampled
/// by OpenGL.
#[rustfmt::skip]
fn load_png<C>(ctx: &mut C, file: File, sampler: &Sampler) -> (Tex2DInner, OutputInfo)
where
    C: GraphicsContext,
{
    let decoder = Decoder::new(file);
    let (info, mut reader) = decoder.read_info().unwrap();
    
    assert_eq!(info.color_type, png::ColorType::RGB);
    assert_eq!(info.bit_depth, png::BitDepth::Eight);
    
    let mut data = vec![0; info.buffer_size()];
    
    reader.next_frame(&mut data).unwrap();
    
    let mut image = Vec::with_capacity(data.len() / 3);
    for i in 0..(data.len() / 3) {
        let idx = i * 3;
        
        image.push((data[idx]     as f32 / 255.,
                    data[idx + 1] as f32 / 255.,
                    data[idx + 2] as f32 / 255.));
    }
    
    let tex = Tex2DInner::new(ctx, [info.width, info.height], 0, sampler).unwrap();
    
    tex.upload(false, &image);
    
    (tex, info)
}
