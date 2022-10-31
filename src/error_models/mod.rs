use crate::qcodes::{sym_hgp::HGPCode, QCode};
pub struct ErrorRound<QCodeT: QCode> {
    bit_error: <QCodeT>::BitError,
    syndrome_error: <QCodeT>::SyndromeError,
    syndrome: <QCodeT>::Syndrome,
}

pub struct ErrorIIDConfigs {
    bit_error_p: f32,
    measure_error_p: f32,
}

pub trait ErrorModel<QCodeT: QCode> {
    type Configs;
    /// Get back a set of errors for a quantum code
    fn error_round(&self, configs: Self::Configs) -> ErrorRound<QCodeT>;
}
