use crate::cell::{body::Body, genome::Genome};

pub mod body;
pub mod chemical;
pub mod cmd;
pub mod genome;

#[derive(Debug, Clone)]
pub struct Cell {
    body: Body,
    genome: Genome,
}
