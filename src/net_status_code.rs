use std::collections::HashMap;

pub enum NetCode {
    // Continue /** RFC 7231, 6.2.1 */
    Continue = 100,
    // SwitchingProtocols /** RFC 7231, 6.2.2 */
    SwitchingProtocols = 101,
    // Processing /** RFC 2518, 10.1 */
    Processing = 102,
    // EarlyHints /** RFC 8297 **/
    EarlyHints = 103,
    // OK /** RFC 7231, 6.3.1 */
    OK = 200,
    // Created /** RFC 7231, 6.3.2 */
    Created = 201,
    // Accepted /** RFC 7231, 6.3.3 */
    Accepted = 202,
    // NonAuthoritativeInfo /** RFC 7231, 6.3.4 */
    NonAuthoritativeInfo = 203,
    // NoContent /** RFC 7231, 6.3.5 */
    NoContent = 204,
    // ResetContent /** RFC 7231, 6.3.6 */
    ResetContent = 205,
    // PartialContent /** RFC 7233, 4.1 */
    PartialContent = 206,
    // MultiStatus /** RFC 4918, 11.1 */
    MultiStatus = 207,
    // AlreadyReported /** RFC 5842, 7.1 */
    AlreadyReported = 208,
    // IMUsed /** RFC 3229, 10.4.1 */
    IMUsed = 226,
    // MultipleChoices /** RFC 7231, 6.4.1 */
    MultipleChoices = 300,
    // MovedPermanently /** RFC 7231, 6.4.2 */
    MovedPermanently = 301,
    // Found /** RFC 7231, 6.4.3 */
    Found = 302,
    // SeeOther /** RFC 7231, 6.4.4 */
    SeeOther = 303,
    // NotModified /** RFC 7232, 4.1 */
    NotModified = 304,
    // UseProxy /** RFC 7231, 6.4.5 */
    UseProxy = 305,
    // TemporaryRedirect /** RFC 7231, 6.4.7 */
    TemporaryRedirect = 307,
    // PermanentRedirect /** RFC 7538, 3 */
    PermanentRedirect = 308,
    // BadRequest /** RFC 7231, 6.5.1 */
    BadRequest = 400,
    // Unauthorized /** RFC 7235, 3.1 */
    Unauthorized = 401,
    // PaymentRequired /** RFC 7231, 6.5.2 */
    PaymentRequired = 402,
    // Forbidden /** RFC 7231, 6.5.3 */
    Forbidden = 403,
    // NotFound /** RFC 7231, 6.5.4 */
    NotFound = 404,
    // MethodNotAllowed /** RFC 7231, 6.5.5 */
    MethodNotAllowed = 405,
    // NotAcceptable /** RFC 7231, 6.5.6 */
    NotAcceptable = 406,
    // ProxyAuthRequired /** RFC 7235, 3.2 */
    ProxyAuthRequired = 407,
    // RequestTimeout /** RFC 7231, 6.5.7 */
    RequestTimeout = 408,
    // Conflict /** RFC 7231, 6.5.8 */
    Conflict = 409,
    // Gone /** RFC 7231, 6.5.9 */
    Gone = 410,
    // LengthRequired /** RFC 7231, 6.5.10 */
    LengthRequired = 411,
    // PreconditionFailed /** RFC 7232, 4.2 */
    PreconditionFailed = 412,
    // RequestEntityTooLarge /** RFC 7231, 6.5.11 */
    RequestEntityTooLarge = 413,
    // RequestURITooLong /** RFC 7231, 6.5.12 */
    RequestURITooLong = 414,
    // UnsupportedMediaType /** RFC 7231, 6.5.13 */
    UnsupportedMediaType = 415,
    // RequestedRangeNotSatisfiable /** RFC 7233, 4.4 */
    RequestedRangeNotSatisfiable = 416,
    // ExpectationFailed /** RFC 7231, 6.5.14 */
    ExpectationFailed = 417,
    // Teapot /** RFC 7168, 2.3.3 */
    Teapot = 418,
    // MisdirectedRequest /** RFC 7540, 9.1.2 */
    MisdirectedRequest = 421,
    // UnprocessableEntity /** RFC 4918, 11.2 */
    UnprocessableEntity = 422,
    // Locked /** RFC 4918, 11.3 */
    Locked = 423,
    // FailedDependency /** RFC 4918, 11.4 */
    FailedDependency = 424,
    // TooEarly /** RFC 8470, 5.2 */
    TooEarly = 425,
    // UpgradeRequired /** RFC 7231, 6.5.15 */
    UpgradeRequired = 426,
    // PreconditionRequired /** RFC 6585, 3 */
    PreconditionRequired = 428,
    // TooManyRequests /** RFC 6585, 4 */
    TooManyRequests = 429,
    // RequestHeaderFieldsTooLarge /** RFC 6585, 5 */
    RequestHeaderFieldsTooLarge = 431,
    // UnavailableForLegalReasons /** RFC 7725, 3 */
    UnavailableForLegalReasons = 451,

    // InternalServerError /** RFC 7231, 6.6.1 */
    InternalServerError = 500,
    // NotImplemented /** RFC 7231, 6.6.2 */
    NotImplemented = 501,
    // BadGateway /** RFC 7231, 6.6.3 */
    BadGateway = 502,
    // ServiceUnavailable /** RFC 7231, 6.6.4 */
    ServiceUnavailable = 503,
    // GatewayTimeout /** RFC 7231, 6.6.5 */
    GatewayTimeout = 504,
    // HTTPVersionNotSupported /** RFC 7231, 6.6.6 */
    HTTPVersionNotSupported = 505,
    // VariantAlsoNegotiates /** RFC 2295, 8.1 */
    VariantAlsoNegotiates = 506,
    // InsufficientStorage /** RFC 4918, 11.5 */
    InsufficientStorage = 507,
    // LoopDetected /** RFC 5842, 7.2 */
    LoopDetected = 508,
    // NotExtended /** RFC 2774, 7 */
    NotExtended = 510,
    // NetworkAuthenticationRequired /** RFC 6585, 6 */
    NetworkAuthenticationRequired = 511,
}

pub fn compute_status_text() -> HashMap<NetCode, String> {
    let mut status_text: HashMap<NetCode, String> = HashMap::new();

    status_text.insert(NetCode::Continue, "Continue".to_string());
    status_text.insert(NetCode::SwitchingProtocols, "Switching Protocols".to_string());
    status_text.insert(NetCode::Processing, "Processing".to_string());
    status_text.insert(NetCode::EarlyHints, "Early Hints".to_string());
    status_text.insert(NetCode::OK, "OK".to_string());
    status_text.insert(NetCode::Created, "Created".to_string());
    status_text.insert(NetCode::Accepted, "Accepted".to_string());
    status_text.insert(NetCode::NonAuthoritativeInfo, "Non-Authoritative Information".to_string());
    status_text.insert(NetCode::NoContent, "No Content".to_string());
    status_text.insert(NetCode::ResetContent, "Reset Content".to_string());
    status_text.insert(NetCode::PartialContent, "Partial Content".to_string());
    status_text.insert(NetCode::MultiStatus, "Multi Status".to_string());
    status_text.insert(NetCode::AlreadyReported, "Already Reported".to_string());
    status_text.insert(NetCode::IMUsed, "IM Used".to_string());
    status_text.insert(NetCode::MultipleChoices, "Multiple Choices".to_string());
    status_text.insert(NetCode::MovedPermanently, "Moved Permanently".to_string());
    status_text.insert(NetCode::Found, "Found".to_string());
    status_text.insert(NetCode::SeeOther, "See Other".to_string());
    status_text.insert(NetCode::NotModified, "Not Modified".to_string());
    status_text.insert(NetCode::UseProxy, "Use Proxy".to_string());
    status_text.insert(NetCode::TemporaryRedirect, "Temporary Redirect".to_string());
    status_text.insert(NetCode::PermanentRedirect, "Permanent Redirect".to_string());
    status_text.insert(NetCode::BadRequest, "Bad Request".to_string());
    status_text.insert(NetCode::Unauthorized, "Unauthorized".to_string());
    status_text.insert(NetCode::PaymentRequired, "Payment Required".to_string());
    status_text.insert(NetCode::Forbidden, "Forbidden".to_string());
    status_text.insert(NetCode::NotFound, "Not Found".to_string());
    status_text.insert(NetCode::MethodNotAllowed, "Method Not Allowed".to_string());
    status_text.insert(NetCode::NotAcceptable, "Not Acceptable".to_string());
    status_text.insert(NetCode::ProxyAuthRequired, "Proxy Auth Required".to_string());
    status_text.insert(NetCode::RequestTimeout, "Request Timeout".to_string());
    status_text.insert(NetCode::Conflict, "Conflict".to_string());
    status_text.insert(NetCode::Gone, "Gone".to_string());
    status_text.insert(NetCode::LengthRequired, "Length Required".to_string());
    status_text.insert(NetCode::PreconditionFailed, "Precondition Failed".to_string());
    status_text.insert(NetCode::RequestEntityTooLarge, "Request Entity Too Large".to_string());
    status_text.insert(NetCode::RequestURITooLong, "Request URI Too Long".to_string());
    status_text.insert(NetCode::UnsupportedMediaType, "Unsupported Media Type".to_string());
    status_text.insert(NetCode::RequestedRangeNotSatisfiable, "Requested Range Not Satisfiable".to_string());
    status_text.insert(NetCode::ExpectationFailed, "Expectation Failed".to_string());
    status_text.insert(NetCode::Teapot, "Teapot".to_string());
    status_text.insert(NetCode::MisdirectedRequest, "Misdirected Request".to_string());
    status_text.insert(NetCode::UnprocessableEntity, "Unprocessable Entity".to_string());
    status_text.insert(NetCode::Locked, "Locked".to_string());
    status_text.insert(NetCode::FailedDependency, "Failed Dependency".to_string());
    status_text.insert(NetCode::TooEarly, "Too Early".to_string());
    status_text.insert(NetCode::UpgradeRequired, "Upgrade Required".to_string());
    status_text.insert(NetCode::PreconditionRequired, "Precondition Required".to_string());
    status_text.insert(NetCode::TooManyRequests, "Too Many Requests".to_string());
    status_text.insert(NetCode::RequestHeaderFieldsTooLarge, "Request Header Fields Too Large".to_string());
    status_text.insert(NetCode::UnavailableForLegalReasons, "Unavailable For Legal Reasons".to_string());
    status_text.insert(NetCode::InternalServerError, "Internal ServerError".to_string());
    status_text.insert(NetCode::NotImplemented, "Not Implemented".to_string());
    status_text.insert(NetCode::BadGateway, "Bad Gateway".to_string());
    status_text.insert(NetCode::ServiceUnavailable, "Service Unavailable".to_string());
    status_text.insert(NetCode::GatewayTimeout, "Gateway Timeout".to_string());
    status_text.insert(NetCode::HTTPVersionNotSupported, "HTTP Version Not Supported".to_string());
    status_text.insert(NetCode::VariantAlsoNegotiates, "Variant Also Negotiates".to_string());
    status_text.insert(NetCode::InsufficientStorage, "Insufficient Storage".to_string());
    status_text.insert(NetCode::LoopDetected, "Loop Detected".to_string());
    status_text.insert(NetCode::NotExtended, "Not Extended".to_string());
    status_text.insert(NetCode::NetworkAuthenticationRequired, "Network Authentication Required".to_string());

    status_text
}
