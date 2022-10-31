use std::collections::HashSet;

use gray_codes::GrayCode32;
use rayon::prelude::*;

use crate::{
    decoder::{BpDecode, Decoder},
    error_models::{ErrorRound, ErrorType, QubitError},
    qcodes::QCode,
};

use super::{HGPCode, QubitsWrapper};

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
///
/// We will skip adding Waterfall or Flip for now...
impl HGPCode {
    /// Get the "score" for a generator
    ///
    /// `return` - the syndrome difference size, the weight of the flip, and the flip
    // TODO: this || thing is stupid here. Use real gray code + reimpl C code...
    fn gen_best_score(
        &self,
        generator_bit: usize,
        generator_check: usize,
        is_X_gen: bool,
        syndrome: &<Self as QCode>::Syndrome,
    ) -> (usize, usize, Vec<bool>) {
        // TODO:!()
        let get_score = |bit_str: &u32| {
            let mut vv_set = HashSet::new();
            let mut cc_set = HashSet::new();

            let q_err = if is_X_gen {
                QubitError::new(true, ErrorType::Z)
            } else {
                QubitError::new(true, ErrorType::X)
            };
            for v_idx in 0..self.dc {
                if (*bit_str) & (1 << (v_idx + self.dc)) > 0 {
                    let b = *self.check_nbhd.get((generator_check, v_idx)).unwrap();
                    let (v1_bit, v2_bit) = if is_X_gen {
                        (generator_bit, b)
                    } else {
                        (b, generator_bit)
                    };
                    vv_set.insert(((v1_bit, v2_bit), q_err.clone()));
                }
            }
            for c_idx in 0..self.dv {
                if (*bit_str) & (1 << (c_idx)) > 0 {
                    let b = *self.bit_nbhd.get((generator_bit, c_idx)).unwrap();
                    let (c1_bit, c2_bit) = if is_X_gen {
                        (b, generator_check)
                    } else {
                        (generator_bit, b)
                    };
                    cc_set.insert(((c1_bit, c2_bit), q_err.clone()));
                }
            }
            let weight = vv_set.len() + cc_set.len();
            // If we are an X gen, then we have Z measurements and vice versa
            let diff = self.syndrome_difference_weight(syndrome, &QubitsWrapper { vv_set, cc_set }, !is_X_gen);
            (diff as usize, weight, vec![false; 1])
        };
        let red_op = |(diff_1, weight_1, flips_1), (diff_2, weight_2, flips_2)| {
            if diff_1 * weight_2 > diff_2 * weight_1 {
                (diff_1, weight_1, flips_1)
            } else {
                (diff_2, weight_2, flips_2)
            }
        };
        self.gray_code
            .par_iter()
            .map(get_score)
            .reduce(|| (0, 0, vec![false; self.dc + self.dv]), red_op)
    }

    fn ssf_decode(
        &self,
        input_error: crate::error_models::ErrorRound<HGPCode>,
    ) -> ErrorRound<HGPCode> {
        todo!()
    }
}
