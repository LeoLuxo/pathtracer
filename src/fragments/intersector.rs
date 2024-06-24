use brainrot::{bevy, path};

use super::shader_fragments::ShaderFragment;
use crate::core::{buffer::BufferRegistrar, shader::Shader};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Shader API:\
/// `fn send_ray(ray_origin: vec3f, ray_dir: vec3f) -> vec4f`
pub trait Intersector: ShaderFragment {}

pub struct Raymarcher;

#[repr(C)]
#[derive(bevy::Component, bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Debug, Default, PartialEq)]
pub struct RaymarchSettings {
	epsilon: f32,
	min_march: f32,
	max_march: f32,
	max_march_steps: u32,
}

impl Intersector for Raymarcher {}
impl ShaderFragment for Raymarcher {
	fn shader(&self) -> Shader {
		path!("raymarch/raymarch.wgsl").into()
	}

	fn fragments(&self) -> Vec<&dyn ShaderFragment> {
		vec![self]
	}

	fn declare_buffers(&self, buffers: &mut BufferRegistrar) {
		buffers.add_uniform_buffer(RaymarchSettings {
			epsilon: 0.000001,
			min_march: 0.001,
			max_march: 1000.0,
			max_march_steps: 2000,
		})
	}
}
