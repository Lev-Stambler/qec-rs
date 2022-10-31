use crate::{error_models::ErrorRound, qcodes::QCode};

pub trait Decoder {
    type Configs;
}

pub trait BpDecode<QCodeT: QCode + Decoder> {
fn bp_decode_iterative_step(
        &self,
        input_error: ErrorRound<QCodeT>,
        initial_bit_error_llr: &[f32],
        initial_syndrome_error_llr: &[f32],
    ) -> ErrorRound<QCodeT>;

    fn bp_decode_first_min(
        &self,
        input_error: ErrorRound<QCodeT>,
        initial_bit_error_llr: &[f32],
        initial_syndrome_error_llr: &[f32],
    ) -> ErrorRound<QCodeT>;
}
