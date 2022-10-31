use crate::qcodes::{sym_hgp::HGPCode, QCode};

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum ErrorType {
    X,
    Y,
    Z,
}
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct QubitError {
    pub errored: bool,
    pub err_type: ErrorType,
}

impl QubitError {
    pub(crate) fn new(err: bool, t: ErrorType) -> Self {
        QubitError {
            errored: err,
            err_type: t,
        }
    }
}

pub struct ErrorRound<QCodeT: QCode> {
    pub(crate) bit_error: <QCodeT>::BitError,
    pub(crate) syndrome_error_X: <QCodeT>::SyndromeError,
    pub(crate) syndrome_error_Z: <QCodeT>::SyndromeError,
    pub(crate) syndrome_X: <QCodeT>::Syndrome,
    pub(crate) syndrome_Z: <QCodeT>::Syndrome,
}

pub struct ErrorIIDConfigs {
    pub bit_error_p: f64,
    pub measure_error_p: f64,
}

pub trait ErrorModel<QCodeT: QCode> {
    /// Get back a set of errors for a quantum code
    fn error_round_X_only_iid(&self, configs: ErrorIIDConfigs) -> ErrorRound<QCodeT>;
}
