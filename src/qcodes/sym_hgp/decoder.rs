use crate::{decoder::{Decoder, BpDecode}, error_models::ErrorRound};

use super::HGPCode;

impl Decoder for HGPCode {
    // TODO: config types
    type Configs = bool;
}

impl BpDecode<HGPCode> for HGPCode {
    fn bp_decode_iterative_step(
        &self,
        input_error: ErrorRound<HGPCode>,
        initial_bit_error_llr: &[f32],
        initial_syndrome_error_llr: &[f32],
    ) -> ErrorRound<HGPCode> {
        todo!()
    }

    fn bp_decode_first_min(
        &self,
        input_error: ErrorRound<HGPCode>,
        initial_bit_error_llr: &[f32],
        initial_syndrome_error_llr: &[f32],
    ) -> ErrorRound<HGPCode> {
        todo!()
    }
}

/// Helper functions for BP
impl HGPCode {}

/// Implement the SSF Decoder which is specific to only HGP codes
impl HGPCode {
    fn ssf_decode(
        &self,
        input_error: crate::error_models::ErrorRound<HGPCode>,
    ) -> ErrorRound<HGPCode> {
        todo!()
    }
}
