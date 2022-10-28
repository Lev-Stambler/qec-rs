pub struct ErrorRound<QCodeT: QCode> {
    bit_error: <QCodeT>::BitError,
    syndrome_error: <QCodeT>::SyndromeError,
    syndrome: <QCodeT>::Syndrome,
}

pub trait ErrorModel {
    type Configs;
    /// Get back a set of errors for a quantum code
    fn error_round<QCodeT: QCode>(&self, qcode: &QCodeT) -> ErrorRound<QCodeT>;
}

pub trait QCode {
    type BitError;
    type SyndromeError;
    type Syndrome;
    type Configs;
}

pub trait Decoder {
    type Configs;
}

pub struct Configs<QCodeT: QCode, ErrorModelT: ErrorModel, DecoderT: Decoder> {
    pub qcode_config: QCodeT::Configs,
    pub error_model_config: ErrorModelT::Configs,
    pub decoder_configs: DecoderT::Configs,
}

pub trait Simulator<QCodeT: QCode, ErrorModelT: ErrorModel, DecoderT: Decoder> {
    type OneRoundRet;

    fn new(configs: Configs<QCodeT, ErrorModelT, DecoderT>) -> Self;
}

fn main() {
    println!("Hello, world!");
}
