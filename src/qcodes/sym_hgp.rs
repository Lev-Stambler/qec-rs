use serde::{Deserialize, Serialize};
use std::{cmp::max, fmt::Error, fs};

use crate::{
    decoder::{BpDecode, Decoder},
    error_models::{ErrorIIDConfigs, ErrorModel, ErrorRound},
    libs::QecError,
};

use super::QCode;
use nalgebra::Matrix;
use ndarray::{Array1, Array2};

pub struct HGPCode {
    bit_nbhd: Array2<bool>,
    check_nbhd: Array2<bool>,
    pub n_qubits: usize,
    pub m_X_syndrome_checks: usize,
    pub m_Z_syndrome_checks: usize,
    n: usize,
    m: usize,
    dc: usize,
    dv: usize,
}

#[derive(Serialize, Deserialize)]
pub struct HGPCodeSerial {
    bit_nbhd: Vec<Vec<u8>>,
    check_nbhd: Vec<Vec<u8>>,
}

impl QCode for HGPCode {
    type BitError = Array2<bool>;

    type SyndromeError = Array2<bool>;

    type Syndrome = Array2<bool>;

    type Configs = ();

    fn load(serialized_path: &str) -> Self {
        let data = fs::read_to_string(serialized_path).unwrap();
        let hgp_s: HGPCodeSerial = serde_json::from_str(&data).unwrap();

        let to_array2 = |v: Vec<Vec<u8>>| {
            let n = v.len();
            let m = v[0].len();
            let flattened = v
                .iter()
                .map(|inner| inner.iter().map(|b| b >= &0).collect::<Vec<bool>>())
                .flatten()
                .collect::<Vec<bool>>();
            Array2::from_shape_vec((n, m), flattened).unwrap()
        };

        let bit_nbhd = to_array2(hgp_s.bit_nbhd);
        let (n, dv) = (bit_nbhd.nrows(), bit_nbhd.ncols());

        let check_nbhd = to_array2(hgp_s.check_nbhd);
        let (m, dc) = (check_nbhd.nrows(), check_nbhd.ncols());

        let s = HGPCode {
            bit_nbhd,
            check_nbhd,
            n,
            m,
            dv,
            dc,
            n_qubits: n * n + m * m,
            m_X_syndrome_checks: n * m,
            m_Z_syndrome_checks: n * m,
        };
        s
    }

    fn equivalent_up_to_stabilizer(&self, e1: Self::BitError, e2: Self::BitError) -> bool {
        todo!()
    }
}

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

impl ErrorModel<HGPCode> for HGPCode {
    // TODO:
    type Configs = ErrorIIDConfigs;

    fn error_round(&self, configs: Self::Configs) -> ErrorRound<HGPCode> {
        todo!()
    }
}
