use crate::{grid::Grid, environment::Environment};

pub const WIDTH_WORLD: usize = 64;
pub const HEIGHT_WORLD: usize = 64;
pub const SIZE_WORLD: usize = WIDTH_WORLD * HEIGHT_WORLD;

/// Определяет мир с сеткой и условиями.
#[derive(derive_new::new, Debug, Clone)]
pub struct World {
    pub grid: Grid,
    pub environment: Environment,
}