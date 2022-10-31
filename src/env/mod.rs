use crate::{decoder::Decoder, error_models::ErrorModel, qcodes::QCode};

pub struct Configs<QCodeT: QCode, DecoderT: Decoder> {
    pub qcode_config: QCodeT::Configs,
    // pub error_model_config: ErrorModelT::Configs,
    pub decoder_configs: DecoderT::Configs,
}
pub trait SimulatorTrait<QCodeT: QCode, ErrorModelT: ErrorModel<QCodeT>, DecoderT: Decoder> {
    type OneRoundRet;

    fn new(configs: Configs<QCodeT, DecoderT>) -> Self;
}

pub struct Simulator();

impl Simulator {}
