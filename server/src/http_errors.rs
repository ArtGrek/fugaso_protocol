use actix_web;

pub fn http_responses_error(a_error_type: &str, a_message: &str, a_error_code: &str) -> actix_web::error::Error {
    let l_error_body = serde_json::json!({"a_message": a_message,",errorCode": a_error_code});
    match a_error_type {
        "BadRequest" => actix_web::error::ErrorBadRequest(l_error_body).into(),
        "ErrorUnauthorized" => actix_web::error::ErrorUnauthorized(l_error_body).into(),
        "ErrorPaymentRequired" => actix_web::error::ErrorPaymentRequired(l_error_body).into(),
        "ErrorForbidden" => actix_web::error::ErrorForbidden(l_error_body).into(),
        "ErrorNotFound" => actix_web::error::ErrorNotFound(l_error_body).into(),
        "ErrorMethodNotAllowed" => actix_web::error::ErrorMethodNotAllowed(l_error_body).into(),
        "ErrorNotAcceptable" => actix_web::error::ErrorNotAcceptable(l_error_body).into(),
        "ErrorProxyAuthenticationRequired" => actix_web::error::ErrorProxyAuthenticationRequired(l_error_body).into(),
        "ErrorRequestTimeout" => actix_web::error::ErrorRequestTimeout(l_error_body).into(),
        "ErrorConflict" => actix_web::error::ErrorConflict(l_error_body).into(),
        "ErrorGone" => actix_web::error::ErrorGone(l_error_body).into(),
        "ErrorLengthRequired" => actix_web::error::ErrorLengthRequired(l_error_body).into(),
        "ErrorPreconditionFailed" => actix_web::error::ErrorPreconditionFailed(l_error_body).into(),
        "ErrorPayloadTooLarge" => actix_web::error::ErrorPayloadTooLarge(l_error_body).into(),
        "ErrorUriTooLong" => actix_web::error::ErrorUriTooLong(l_error_body).into(),
        "ErrorUnsupportedMediaType" => actix_web::error::ErrorUnsupportedMediaType(l_error_body).into(),
        "ErrorRangeNotSatisfiable" => actix_web::error::ErrorRangeNotSatisfiable(l_error_body).into(),
        "ErrorExpectationFailed" => actix_web::error::ErrorExpectationFailed(l_error_body).into(),
        "ErrorMisdirectedRequest" => actix_web::error::ErrorMisdirectedRequest(l_error_body).into(),
        "ErrorUnprocessableEntity" => actix_web::error::ErrorUnprocessableEntity(l_error_body).into(),
        "ErrorLocked" => actix_web::error::ErrorLocked(l_error_body).into(),
        "ErrorFailedDependency" => actix_web::error::ErrorFailedDependency(l_error_body).into(),
        "ErrorUpgradeRequired" => actix_web::error::ErrorUpgradeRequired(l_error_body).into(),
        "ErrorPreconditionRequired" => actix_web::error::ErrorPreconditionRequired(l_error_body).into(),
        "ErrorTooManyRequests" => actix_web::error::ErrorTooManyRequests(l_error_body).into(),
        "ErrorRequestHeaderFieldsTooLarge" => actix_web::error::ErrorRequestHeaderFieldsTooLarge(l_error_body).into(),
        "ErrorUnavailableForLegalReasons" => actix_web::error::ErrorUnavailableForLegalReasons(l_error_body).into(),
        "ErrorInternalServerError" => actix_web::error::ErrorInternalServerError(l_error_body).into(),
        "ErrorNotImplemented" => actix_web::error::ErrorNotImplemented(l_error_body).into(),
        "ErrorBadGateway" => actix_web::error::ErrorBadGateway(l_error_body).into(),
        "ErrorServiceUnavailable" => actix_web::error::ErrorServiceUnavailable(l_error_body).into(),
        "ErrorGatewayTimeout" => actix_web::error::ErrorGatewayTimeout(l_error_body).into(),
        "ErrorHttpVersionNotSupported" => actix_web::error::ErrorHttpVersionNotSupported(l_error_body).into(),
        "ErrorVariantAlsoNegotiates" => actix_web::error::ErrorVariantAlsoNegotiates(l_error_body).into(),
        "ErrorInsufficientStorage" => actix_web::error::ErrorInsufficientStorage(l_error_body).into(),
        "ErrorLoopDetected" => actix_web::error::ErrorLoopDetected(l_error_body).into(),
        "ErrorNotExtended" => actix_web::error::ErrorNotExtended(l_error_body).into(),
        "ErrorNetworkAuthenticationRequired" => actix_web::error::ErrorNetworkAuthenticationRequired(l_error_body).into(),
        _ => actix_web::error::ErrorInternalServerError (l_error_body).into()
    }
}

// ErrorBadRequest                      - 400 - ERR_UNKNOWN_REQUEST
// ErrorUnauthorized                    - 401 - ERR_UNAUTHORIZED
// ErrorPaymentRequired                 - 402 - ERR_PAYMENT_REQUIRED
// ErrorForbidden                       - 403 - ERR_FORBIDDEN
// ErrorNotFound                        - 404 - ERR_RESOURCE_NOT_FOUND
// ErrorMethodNotAllowed                - 405 - ERR_METHOD_NOT_ALLOWED
// ErrorNotAcceptable                   - 406 - ERR_NOT_ACCEPTABLE
// ErrorProxyAuthenticationRequired     - 407 - ERR_PROXY_AUTHENTICATION_REQUIRED
// ErrorRequestTimeout                  - 408 - ERR_REQUEST_TIMEOUT
// ErrorConflict                        - 409 - ERR_CONFLICT
// ErrorGone                            - 410 - ERR_RESOURCE_GONE
// ErrorLengthRequired                  - 411 - ERR_LENGTH_REQUIRED
// ErrorPreconditionFailed              - 412 - ERR_PRECONDITION_FAILED
// ErrorPayloadTooLarge                 - 413 - ERR_PAYLOAD_TOO_LARGE
// ErrorUriTooLong                      - 414 - ERR_URI_TOO_LONG
// ErrorUnsupportedMediaType            - 415 - ERR_UNSUPPORTED_MEDIA_TYPE
// ErrorRangeNotSatisfiable             - 416 - ERR_RANGE_NOT_SATISFIABLE
// ErrorExpectationFailed               - 417 - ERR_EXPECTATION_FAILED
// ErrorMisdirectedRequest              - 421 - ERR_MISDIRECTED_REQUEST
// ErrorUnprocessableEntity             - 422 - ERR_UNPROCESSABLE_ENTITY
// ErrorLocked                          - 423 - ERR_RESOURCE_LOCKED
// ErrorFailedDependency                - 424 - ERR_FAILED_DEPENDENCY
// ErrorUpgradeRequired                 - 426 - ERR_UPGRADE_REQUIRED
// ErrorPreconditionRequired            - 428 - ERR_PRECONDITION_REQUIRED
// ErrorTooManyRequests                 - 429 - ERR_TOO_MANY_REQUESTS
// ErrorRequestHeaderFieldsTooLarge     - 431 - ERR_REQUEST_HEADER_FIELDS_TOO_LARGE
// ErrorUnavailableForLegalReasons      - 451 - ERR_UNAVAILABLE_FOR_LEGAL_REASONS
// ErrorInternalServerError             - 500 - ERR_INTERNAL_ERROR
// ErrorNotImplemented                  - 501 - ERR_NOT_IMPLEMENTED
// ErrorBadGateway                      - 502 - ERR_BAD_GATEWAY
// ErrorServiceUnavailable              - 503 - ERR_SERVICE_UNAVAILABLE
// ErrorGatewayTimeout                  - 504 - ERR_GATEWAY_TIMEOUT
// ErrorHttpVersionNotSupported         - 505 - ERR_HTTP_VERSION_NOT_SUPPORTED
// ErrorVariantAlsoNegotiates           - 506 - ERR_VARIANT_ALSO_NEGOTIATES
// ErrorInsufficientStorage             - 507 - ERR_INSUFFICIENT_STORAGE
// ErrorLoopDetected                    - 508 - ERR_LOOP_DETECTED
// ErrorNotExtended                     - 510 - ERR_NOT_EXTENDED
// ErrorNetworkAuthenticationRequired   - 511 - ERR_NETWORK_AUTHENTICATION_REQUIRED
// ErrorWebServerUnknown                - 520 - ERR_WEB_SERVER_UNKNOWN
// ErrorWebServerDown                   - 521 - ERR_WEB_SERVER_DOWN
// ErrorConnectionTimedOut              - 522 - ERR_CONNECTION_TIMED_OUT
// ErrorOriginUnreachable               - 523 - ERR_ORIGIN_UNREACHABLE
// ErrorTimeoutOccurred                 - 524 - ERR_TIMEOUT_OCCURRED

