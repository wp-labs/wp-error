//! Lightweight mapping from system error code (SysErrorCode) to HTTP status.
//! This module is framework-agnostic and does not depend on any HTTP stack.
use crate::SysErrorCode;

/// Map a system error code (u16) to an HTTP status code (u16).
/// Suggested contract:
/// - 404xx -> 404
/// - 422xx -> 422
/// - 204xx -> 204
/// - 499xx -> 499 (Client Closed Request) or 503; we choose 499 to distinguish
/// - 502xx -> 502
/// - 503xx -> 503
/// - 500xx -> 500
/// - else -> 500
pub fn http_status_for_sys(sys: u16) -> u16 {
    let head = sys / 100; // e.g., 42211 -> 422
    match head {
        404 => 404,
        422 => 422,
        204 => 204,
        499 => 499,
        502 => 502,
        503 => 503,
        500 => 500,
        _ => 500,
    }
}

/// Convenience: map a reason to HTTP status.
pub fn http_status_for_reason<R: SysErrorCode>(r: &R) -> u16 {
    http_status_for_sys(r.sys_code())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_status_404() {
        assert_eq!(http_status_for_sys(40401), 404);
        assert_eq!(http_status_for_sys(40402), 404);
        assert_eq!(http_status_for_sys(40411), 404);
        assert_eq!(http_status_for_sys(40412), 404);
    }

    #[test]
    fn test_http_status_422() {
        assert_eq!(http_status_for_sys(42201), 422);
        assert_eq!(http_status_for_sys(42211), 422);
        assert_eq!(http_status_for_sys(42212), 422);
        assert_eq!(http_status_for_sys(42299), 422);
    }

    #[test]
    fn test_http_status_204() {
        assert_eq!(http_status_for_sys(20401), 204);
        assert_eq!(http_status_for_sys(20402), 204);
    }

    #[test]
    fn test_http_status_499() {
        assert_eq!(http_status_for_sys(49901), 499);
        assert_eq!(http_status_for_sys(49999), 499);
    }

    #[test]
    fn test_http_status_502() {
        assert_eq!(http_status_for_sys(50201), 502);
        assert_eq!(http_status_for_sys(50209), 502);
    }

    #[test]
    fn test_http_status_503() {
        assert_eq!(http_status_for_sys(50301), 503);
        assert_eq!(http_status_for_sys(50311), 503);
        assert_eq!(http_status_for_sys(50312), 503);
    }

    #[test]
    fn test_http_status_500() {
        assert_eq!(http_status_for_sys(50001), 500);
        assert_eq!(http_status_for_sys(50009), 500);
        assert_eq!(http_status_for_sys(50041), 500);
    }

    #[test]
    fn test_http_status_default_fallback() {
        // Unknown codes should fallback to 500
        assert_eq!(http_status_for_sys(10001), 500);
        assert_eq!(http_status_for_sys(60001), 500);
        assert_eq!(http_status_for_sys(0), 500);
    }

    #[test]
    fn test_http_status_for_reason() {
        struct TestReason(u16);
        impl SysErrorCode for TestReason {
            fn sys_code(&self) -> u16 {
                self.0
            }
        }

        let r404 = TestReason(40401);
        assert_eq!(http_status_for_reason(&r404), 404);

        let r422 = TestReason(42201);
        assert_eq!(http_status_for_reason(&r422), 422);

        let r500 = TestReason(50001);
        assert_eq!(http_status_for_reason(&r500), 500);
    }
}
