use std::time;

use cgmath as cgm;

use crate::physics::color;
use crate::render::vulkan::data;

/// An Omni point light, with position in 3d space, and 'color' defined in lumens per CIE XYZ
/// color channel.
pub struct Omni {
    pub position: cgm::Vector3<f32>,
    /// Luminour power/flux defined as lumens per XYZ color channel.
    pub color: color::XYZ,

    pub id: u64,
}

impl Omni {
    /// Make a test light. This has... a color. It's kinda yellow. And something close to 650
    /// lumens of luminous power. 
    // TODO(q3k): implement [Kry85] (eq. 68) somewhere in //engine/src/physics for generation
    // of nice lights colours from color temperature.
    //
    // [Kry85]
    // M. Krystek. 1985. "An algorithm to calculate correlated color temperature"
    // Color Research & Application, 10 (1), 38–40.
    pub fn test(position: cgm::Vector3<f32>) -> Self {
        Self {
            position,
            color: color::XYZ::new(234.7, 214.1, 207.9),
            // TODO: use a better method
            id: time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_nanos() as u64,
        }
    }

    pub fn vulkan_uniform(&self) -> data::OmniLight {
        // TODO: cache this?
        data::OmniLight {
            pos: [self.position.x, self.position.y, self.position.z, 1.0],
            color: [self.color.x, self.color.y, self.color.z, 0.0],
        }
    }
}
