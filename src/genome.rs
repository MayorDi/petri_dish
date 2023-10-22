use crate::position::Direction;

pub type Genes = [Gene; 8];

#[derive(derive_new::new, Debug, Clone, Copy)]
pub struct Genome(pub Genes);

#[derive(derive_new::new, Debug, Clone, Copy)]
pub enum Gene {
    Reproduce,
    ToRotate(Direction),
    GenerateEnergy,
}