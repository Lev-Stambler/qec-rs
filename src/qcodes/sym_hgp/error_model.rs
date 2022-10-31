use std::collections::HashSet;

use crate::{
    error_models::{ErrorIIDConfigs, ErrorModel, ErrorRound, ErrorType, QubitError},
    qcodes::QCode,
};
use ndarray::Array2;
use rand::{
    distributions::{Bernoulli, Uniform},
    prelude::Distribution,
    Rng,
};

use super::HGPCode;

impl ErrorModel<HGPCode> for HGPCode {
    fn error_round_X_only_iid(&self, configs: ErrorIIDConfigs) -> ErrorRound<HGPCode> {
        //  TODO: does this rng have to be inited at the very beginning?
        let mut rng = rand::thread_rng();
        let bit_err = Bernoulli::new(configs.bit_error_p).unwrap();
        let meas_err = Bernoulli::new(configs.measure_error_p).unwrap();
        let mut bit_error = super::QubitsWrapper {
            // TODO: hash set impl
            vv_set: HashSet::new(),
            cc_set: HashSet::new(),
        };

        // Generate VV errors
        for v1 in 0..self.n {
            for v2 in 0..self.n {
                let is_err = bit_err.sample(&mut rand::thread_rng());
                if is_err {
                    bit_error
                        .vv_set
                        .insert(((v1, v2), QubitError::new(is_err, ErrorType::X)));
                }
            }
        }

        // Generate CC errors
        for c1 in 0..self.m {
            for c2 in 0..self.m {
                let is_err = bit_err.sample(&mut rand::thread_rng());
                if is_err {
                    bit_error
                        .cc_set
                        .insert(((c1, c2), QubitError::new(is_err, ErrorType::X)));
                }
            }
        }

        // Generate syndrome errors
        let syndrome_error = Array2::from_shape_fn((self.n, self.m), |_| {
            meas_err.sample(&mut rand::thread_rng())
        });

        let syndrome_Z = self.syndrome_from_error(&bit_error, false) ^ syndrome_error;

        ErrorRound {
            bit_error,
            syndrome_error_X: Array2::default((self.m, self.n)), // C V
            syndrome_error_Z: syndrome_error,
            syndrome_X: Array2::default((self.m, self.n)),
            syndrome_Z: syndrome_Z,
        }
    }
}
