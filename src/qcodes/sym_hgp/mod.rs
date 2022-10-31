mod decoder;
mod error_model;

use serde::{Deserialize, Serialize};
use std::{cmp::max, collections::HashSet, fmt::Error, fs};

use crate::{
    decoder::{BpDecode, Decoder},
    error_models::{ErrorIIDConfigs, ErrorModel, ErrorRound, ErrorType, QubitError},
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

/// Bit position within either the bit_nbhd bit_nbhd array or check_nbhd check_nbhd array
/// depending on context
type BitPos = (usize, usize);

struct QubitsWrapper<T> {
    // vv: Array2<T>,
    vv_set: HashSet<(BitPos, T)>,
    // cc: Array2<T>,
    cc_set: HashSet<(BitPos, T)>,
}

#[derive(Serialize, Deserialize)]
pub struct HGPCodeSerial {
    bit_nbhd: Vec<Vec<u8>>,
    check_nbhd: Vec<Vec<u8>>,
}

impl QCode for HGPCode {
    type BitError = QubitsWrapper<QubitError>;

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

    fn syndrome_from_error(&self, e: &Self::BitError, is_X: bool) -> Self::Syndrome {
        let n_rows = if is_X { self.m } else { self.n };
        let n_cols = if is_X { self.n } else { self.m };
        let mut s = Array2::default((n_rows, n_cols));
        self.update_syndrome_from_error(&mut s, e, is_X);
        s
    }

    fn update_syndrome_from_error(
        &self,
        mut current_synd: &mut Self::Syndrome,
        e: &Self::BitError,
        is_X: bool,
    ) {
        for ((v1, v2), err) in &e.vv_set {
            if err.errored && is_X && (err.err_type == ErrorType::Z || err.err_type == ErrorType::Y)
            {
                //  TODO: how to???
                // current_synd[*v1][*v2] ^= true;
            } else if err.errored
                && !is_X
                && (err.err_type == ErrorType::X || err.err_type == ErrorType::Y)
            {
                *(current_synd.get_mut((*v1, ____)).unwrap()) ^= true;
            }
        }
        for ((c1, c2), err) in &e.cc_set {
            if err.errored && is_X && (err.err_type == ErrorType::Z || err.err_type == ErrorType::Y)
            {
                //  TODO: how to???
                // current_synd[*v1][*v2] ^= true;
            } else if err.errored
                && !is_X
                && (err.err_type == ErrorType::X || err.err_type == ErrorType::Y)
            {
                *(current_synd.get_mut((_____, *c2)).unwrap()) ^= true;
            }
        }
    }
}

mod test {
    use super::*;
    #[test]
    fn test_loading_code() {
        let code = HGPCode::load("examples/codes/3_4_40_30_hgp.json");
        assert_eq!(code.dc, 4);
        assert_eq!(code.dv, 3);
        assert_eq!(code.n, 40);
        assert_eq!(code.m, 30);
        assert_eq!(code.n_qubits, 2500);
        assert_eq!(code.m_X_syndrome_checks, 1200);
    }
}
