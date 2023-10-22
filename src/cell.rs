use crate::{
    position::*,
    genome::*
};

#[derive(derive_new::new, Debug, Clone, Copy)]
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

    fn rotate(&mut self, direction: Direction);
}