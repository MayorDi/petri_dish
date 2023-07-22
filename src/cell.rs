use crate::{
    position::*,
    genome::*
};
use macro_tools_pd::{GetPosition, ArrayPosition};

#[derive(derive_new::new, Debug, Clone, Copy, GetPosition, ArrayPosition)]
pub struct Cell {
    pub id: usize,
    pub position: Position,
    pub nutrients: f32,
    pub energy: f32,

    pub genome: Genome,
    pub direction: Direction,
    pub time_life: usize, 
}

pub trait Behaviour {
    #[must_use]
    fn update(&mut self);

    fn generate_energy(&mut self);

    fn attack(&self, target: &mut Cell);

    fn veer(&mut self, direction: Direction);
}