use crate::{qcodes::QCode, error_models::ErrorModel, decoder::Decoder};

pub struct Configs<QCodeT: QCode, ErrorModelT: ErrorModel<QCodeT>, DecoderT: Decoder> {
    pub qcode_config: QCodeT::Configs,
    pub error_model_config: ErrorModelT::Configs,
    pub decoder_configs: DecoderT::Configs,
}
pub trait SimulatorTrait<QCodeT: QCode, ErrorModelT: ErrorModel<QCodeT>, DecoderT: Decoder> {
    type OneRoundRet;

    fn new(configs: Configs<QCodeT, ErrorModelT, DecoderT>) -> Self;
}

pub struct Simulator();

impl Simulator {
    fn equivalent_up_to_stabilizer() {
    }
}


