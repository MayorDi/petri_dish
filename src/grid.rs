use std::collections::LinkedList;

use crate::{cell::Cell, world::SIZE_WORLD};

/// Сетка хранит основную сетку и реальную сетку.
/// Реальная сетка определяет все существующие клетки в мире.
#[derive(derive_new::new, Debug, Clone)]
pub struct Grid {
    pub grid: [Cell; SIZE_WORLD],
    pub real_grid: LinkedList<usize>,
}