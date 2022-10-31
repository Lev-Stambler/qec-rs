use crate::libs::QecError;

pub mod sym_hgp;

pub trait QCode: Sized {
    type BitError;
    type SyndromeError;
    type Syndrome;
    type Configs;

    fn load(serialized_path: &str) -> Self;

    fn equivalent_up_to_stabilizer(&self, e1: Self::BitError, e2: Self::BitError) -> bool;
}
