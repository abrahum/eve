use bytes::Bytes;

pub type EveResult<T> = Result<T, EveError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EveError {
    // dedoce income packet errors
    DecodeIncomeUnkownFlag(i32),
    DecodeIncomeInvalid(Bytes),
    // parse sso frame errors
    SsoSessionExpired,
    SsoPacketDropped,
    SsoUnsuccessRetCode(i32),
    // Jce Errors
    JceError(jcers::JceError),
}

impl EveError {
    pub(crate) fn err<T>(self) -> EveResult<T> {
        Err(self)
    }
}
