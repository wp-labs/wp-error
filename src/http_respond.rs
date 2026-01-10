//! Helpers to build consistent error responses (JSON/text) without
//! coupling to any specific web framework.
use crate::{SysErrorCode, http_status_for_reason};
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct ErrorResponse<'a> {
    pub status: u16,
    pub sys_code: u16,
    pub tag: &'a str,
    pub message: String,
}

pub fn build_error_response<'a, R: SysErrorCode + Display>(r: &'a R) -> ErrorResponse<'a> {
    ErrorResponse {
        status: http_status_for_reason(r),
        sys_code: r.sys_code(),
        tag: r.sys_tag(),
        message: r.to_string(),
    }
}

pub fn error_response_json<R: SysErrorCode + Display>(r: &R) -> (u16, String) {
    let er = build_error_response(r);
    (
        er.status,
        serde_json::to_string(&er).unwrap_or_else(|_| {
            format!(
                "{{\"status\":{},\"sys_code\":{},\"tag\":\"{}\",\"message\":\"{}\"}}",
                er.status,
                er.sys_code,
                er.tag,
                er.message.replace('"', "'")
            )
        }),
    )
}

pub fn error_response_text<R: SysErrorCode + Display>(r: &R) -> (u16, String) {
    let er = build_error_response(r);
    (
        er.status,
        format!("[{}] {}: {}", er.tag, er.sys_code, er.message),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    struct TestError {
        code: u16,
        tag: &'static str,
        msg: String,
    }

    impl SysErrorCode for TestError {
        fn sys_code(&self) -> u16 {
            self.code
        }
        fn sys_tag(&self) -> &'static str {
            self.tag
        }
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.msg)
        }
    }

    #[test]
    fn test_build_error_response() {
        let err = TestError {
            code: 42201,
            tag: "test.tag",
            msg: "test message".into(),
        };
        let resp = build_error_response(&err);
        assert_eq!(resp.status, 422);
        assert_eq!(resp.sys_code, 42201);
        assert_eq!(resp.tag, "test.tag");
        assert_eq!(resp.message, "test message");
    }

    #[test]
    fn test_error_response_json() {
        let err = TestError {
            code: 40401,
            tag: "conf.core",
            msg: "not found".into(),
        };
        let (status, json) = error_response_json(&err);
        assert_eq!(status, 404);
        assert!(json.contains("\"status\":404"));
        assert!(json.contains("\"sys_code\":40401"));
        assert!(json.contains("\"tag\":\"conf.core\""));
        assert!(json.contains("\"message\":\"not found\""));
    }

    #[test]
    fn test_error_response_json_with_quotes() {
        let err = TestError {
            code: 50001,
            tag: "test",
            msg: "error with \"quotes\"".into(),
        };
        let (status, json) = error_response_json(&err);
        assert_eq!(status, 500);
        // Should properly escape quotes in JSON
        assert!(json.contains("error with"));
    }

    #[test]
    fn test_error_response_text() {
        let err = TestError {
            code: 42211,
            tag: "parse.oml",
            msg: "syntax error".into(),
        };
        let (status, text) = error_response_text(&err);
        assert_eq!(status, 422);
        assert_eq!(text, "[parse.oml] 42211: syntax error");
    }

    #[test]
    fn test_error_response_text_format() {
        let err = TestError {
            code: 50201,
            tag: "source",
            msg: "supplier failed".into(),
        };
        let (status, text) = error_response_text(&err);
        assert_eq!(status, 502);
        assert!(text.starts_with("[source]"));
        assert!(text.contains("50201"));
        assert!(text.contains("supplier failed"));
    }

    #[test]
    fn test_error_response_serialize() {
        let resp = ErrorResponse {
            status: 404,
            sys_code: 40401,
            tag: "test",
            message: "not found".into(),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"status\":404"));
        assert!(json.contains("\"sys_code\":40401"));
    }
}
