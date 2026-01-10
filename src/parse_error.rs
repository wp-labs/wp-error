use derive_more::From;
use orion_error::ErrorCode;
use orion_error::StructError;
use orion_error::UvsDataFrom;
use orion_error::UvsReason;
use orion_error::UvsResFrom;
use serde::Serialize;
use thiserror::Error;

/*
fn translate_position(input: &[u8], index: usize) -> (usize, usize) {
    if input.is_empty() {
        return (0, index);
    }

    let safe_index = index.min(input.len() - 1);
    let column_offset = index - safe_index;
    let index = safe_index;

    let nl = input[0..index]
        .iter()
        .rev()
        .enumerate()
        .find(|(_, b)| **b == b'\n')
        .map(|(nl, _)| index - nl - 1);
    let line_start = match nl {
        Some(nl) => nl + 1,
        None => 0,
    };
    let line = input[0..line_start].iter().filter(|b| **b == b'\n').count();

    let column = std::str::from_utf8(&input[line_start..=index])
        .map(|s| s.chars().count() - 1)
        .unwrap_or_else(|_| index - line_start);
    let column = column + column_offset;

    (line, column)
}
*/

#[derive(Error, Debug, Clone, PartialEq, Serialize, From)]
pub enum OMLCodeReason {
    #[error("{0}")]
    Syntax(String),
    #[from(skip)]
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Uvs(UvsReason),
}
impl ErrorCode for OMLCodeReason {
    fn error_code(&self) -> i32 {
        crate::codes::SysErrorCode::sys_code(self) as i32
    }
}

pub type OMLCodeError = StructError<OMLCodeReason>;

pub type OMLCodeResult<T> = Result<T, OMLCodeError>;

#[derive(Error, Debug, PartialEq)]
pub enum DataErrKind {
    #[error("format error : {0}\n{1:?} ")]
    FormatError(String, Option<String>),
    #[error("not complete")]
    NotComplete,
    #[error("no parse data: {0}")]
    UnParse(String),

    #[error("less data")]
    LessData,
    #[error("empty data")]
    EmptyData,
    #[error("struct less : {0}")]
    LessStc(String),
    #[error("define less : {0}")]
    LessDef(String),
}
impl From<DataErrKind> for OMLCodeReason {
    fn from(value: DataErrKind) -> Self {
        OMLCodeReason::from(UvsReason::from_data(format!("{}", value), None))
    }
}
pub type OmlCodeResult<T> = Result<T, OMLCodeError>;

// ParseError<&str, ContextError<StrContext>>

impl From<OMLCodeReason> for UvsReason {
    fn from(value: OMLCodeReason) -> Self {
        UvsReason::from_res(value.to_string())
    }
}
