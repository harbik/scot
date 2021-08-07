pub mod xyz;
pub mod lab;

pub use crate::models::xyz::{CieXYZ, CieYxy};
pub use crate::models::lab::CieLab;

pub trait Model{}
