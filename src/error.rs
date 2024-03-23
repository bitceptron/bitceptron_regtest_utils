use tokio::task::JoinError;

#[derive(Debug)]
pub enum RegtestUtilsError {
    InvalidPort,
    RepetitiveRegtestPortOrRpcportConfs,
    RpcClientEstablishmentError(bitcoincore_rpc::Error),
    OsCommandError(std::process::Command),
    TokioJoinError(JoinError),
}


impl From<bitcoincore_rpc::Error> for RegtestUtilsError {
    fn from(value: bitcoincore_rpc::Error) -> Self {
        RegtestUtilsError::RpcClientEstablishmentError(value)
    }
}

impl From<std::process::Command> for RegtestUtilsError {
    fn from(value: std::process::Command) -> Self {
        RegtestUtilsError::OsCommandError(value)
    }
}

impl From<JoinError> for RegtestUtilsError {
    fn from(value: JoinError) -> Self {
        RegtestUtilsError::TokioJoinError(value)
    }
}