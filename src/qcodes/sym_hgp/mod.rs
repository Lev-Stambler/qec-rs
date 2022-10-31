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
use ndarray::Array2;

pub struct HGPCode {
    bit_nbhd: Array2<usize>,
    check_nbhd: Array2<usize>,
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

pub struct QubitsWrapper<T> {
    // vv: Array2<T>,
    vv_set: HashSet<(BitPos, T)>,
    // cc: Array2<T>,
    cc_set: HashSet<(BitPos, T)>,
}

#[derive(Serialize, Deserialize)]
pub struct HGPCodeSerial {
    bit_nbhd: Vec<Vec<usize>>,
    check_nbhd: Vec<Vec<usize>>,
}

impl QCode for HGPCode {
    type BitError = QubitsWrapper<QubitError>;

    type SyndromeError = Array2<bool>;

    type Syndrome = Array2<bool>;

    type Configs = ();

    fn load(serialized_path: &str) -> Self {
        let data = fs::read_to_string(serialized_path).unwrap();
        let hgp_s: HGPCodeSerial = serde_json::from_str(&data).unwrap();

        let to_array2 = |v: Vec<Vec<usize>>| {
            let n = v.len();
            let m = v[0].len();
            let flattened = v.into_iter().flatten().collect::<Vec<usize>>();
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
        current_synd: &mut Self::Syndrome,
        e: &Self::BitError,
        is_X_syndrome: bool,
    ) {
        for ((v1, v2), err) in &e.vv_set {
            if err.errored
                && is_X_syndrome
                && (err.err_type == ErrorType::Z || err.err_type == ErrorType::Y)
            {
                //  TODO: how to???
                // current_synd[*v1][*v2] ^= true;
                todo!()
            } else if err.errored
                && !is_X_syndrome
                && (err.err_type == ErrorType::X || err.err_type == ErrorType::Y)
            {
                for c2 in self.bit_nbhd.row(*v2) {
                    *(current_synd.get_mut((*v1, *c2)).unwrap()) ^= true;
                }
            }
        }
        for ((c1, c2), err) in &e.cc_set {
            if err.errored
                && is_X_syndrome
                && (err.err_type == ErrorType::Z || err.err_type == ErrorType::Y)
            {
                todo!()
            } else if err.errored
                && !is_X_syndrome
                && (err.err_type == ErrorType::X || err.err_type == ErrorType::Y)
            {
                for v1 in self.check_nbhd.row(*c1) {
                    *(current_synd.get_mut((*v1, *c2)).unwrap()) ^= true;
                }
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

    use super::*;
    #[test]
    fn test_err_to_syndrome() {
        let code = HGPCode::load("examples/codes/3_4_40_30_hgp.json");
        let mut vv_set = HashSet::new();
        let mut cc_set = HashSet::new();

        vv_set.insert((
            (0, 0),
            QubitError {
                err_type: ErrorType::X,
                errored: true,
            },
        ));

        cc_set.insert((
            (0, 0),
            QubitError {
                err_type: ErrorType::X,
                errored: true,
            },
        ));

        let e = QubitsWrapper { vv_set, cc_set };
        let synd = code.syndrome_from_error(&e, false);

        assert!(synd.get((0, 27)).unwrap());
        assert!(synd.get((0, 14)).unwrap());
        assert!(synd.get((0, 21)).unwrap());

        assert!(synd.get((34, 0)).unwrap());
        assert!(synd.get((14, 0)).unwrap());
        assert!(synd.get((16, 0)).unwrap());
        assert!(synd.get((4, 0)).unwrap());
    }
}
