use crate::libs::QecError;

pub mod sym_hgp;

pub trait QCode: Sized {
    type BitError;
    type SyndromeError;
    type Syndrome;
    type Configs;

    fn load(serialized_path: &str) -> Self;

    fn equivalent_up_to_stabilizer(&self, e1: Self::BitError, e2: Self::BitError) -> bool;

    /// Get the syndrome from an error pattern
    /// `is_X` - denotes if the syndrome is of the X type. If not, assume that we are dealing with Z type
    fn syndrome_from_error(&self, e: &Self::BitError, is_X: bool) -> Self::Syndrome;

    /// `is_X` - denotes if the syndrome is of the X type. If not, assume that we are dealing with Z type
    fn update_syndrome_from_error(
        &self,
        current_synd: &mut Self::Syndrome,
        e: &Self::BitError,
        is_X: bool,
    );
}
