use std::fmt;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Header {
    /// Cors
    AccessControlAllowOrigin,
    Accept,
    /// How to encoding to send files
    AcceptEncoding,
    /// Languages
    AcceptLanguage,
    ///
    AcceptRanges,
    ///
    CacheControl,
    ///
    Cookie,
    /// Keep Connection
    Connection,
    /// Content Length
    ContentLength,
    /// Content Segurity Policy
    ContentSegurityPolicy,
    /// Content Type
    ContentType,
    /// Date
    Date,
    /// Cache
    ETag,
    /// Where
    Host,
    /// Last Modified
    LastModified,
    ///
    Location,
    ///
    SecWebSocketAccept,
    ///
    SecWebSocketKey,
    ///
    SecWebSocketOrigin,
    ///
    SecWebSocketProtocol,
    ///
    SecWebSocketVersion,
    /// Server name
    Server,
    /// Cookies
    SetCookie,
    /// Encoding content
    TransferEncoding,
    /// Upgrade
    Upgrade,
    ///
    UserAgent,
    /// Accept Encoding
    Vary,
    /// For Headers not implemented yet
    NONE,
}

impl Header {
    pub fn as_str(&self) -> &'static str {
        match self {
            Header::AccessControlAllowOrigin => "Access-Control-Allow-Origin",
            Header::Accept => "Accept",
            Header::AcceptEncoding => "Accept-Encoding",
            Header::AcceptLanguage => "Accept-Languages",
            Header::AcceptRanges => "Accept-Ranges",

            Header::CacheControl => "Cache-Control",
            Header::Cookie => "Cookie",
            Header::Connection => "Connection",
            Header::ContentLength => "Content-Length",
            Header::ContentSegurityPolicy => "Content-Segurity-Policy",
            Header::ContentType => "Content-Type",

            Header::Date => "Date",
            Header::ETag => "ETag",
            Header::Host => "Host",

            Header::Location => "Location",
            Header::LastModified => "Last-Modified",

            Header::SecWebSocketAccept => "Sec-WebSocket-Accept",
            Header::SecWebSocketKey => "Sec-WebSocket-Key",
            Header::SecWebSocketOrigin => "Sec-WebSocket-Origin",
            Header::SecWebSocketProtocol => "Sec-WebSocket-Protocol",
            Header::SecWebSocketVersion => "Sec-WebSocket-Version",

            Header::Server => "Server",
            Header::SetCookie => "Set-Cookie",

            Header::TransferEncoding => "Transfer-Encoding",
            Header::Upgrade => "Upgrade",
            Header::UserAgent => "User-Agent",
            Header::Vary => "Vary",

            Header::NONE => "",
        }
    }
}

pub fn from_string_to_header(header: String) -> Header {
    match header.to_lowercase().as_str() {
        "access-control-allow-origin" => Header::AccessControlAllowOrigin,
        "accept" => Header::Accept,
        "accept-encoding" => Header::AcceptEncoding,
        "accept-language" => Header::AcceptLanguage,
        "accept-ranges" => Header::AcceptRanges,

        "cache-control" => Header::CacheControl,
        "cookie" => Header::Cookie,
        "connection" => Header::Connection,
        "content-length" => Header::ContentLength,
        "content-segurity-policy" => Header::ContentSegurityPolicy,
        "content-type" => Header::ContentType,

        "date" => Header::Date,
        "etag" => Header::ETag,
        "host" => Header::Host,

        "location" => Header::Location,
        "last-modified" => Header::LastModified,

        "sec-webSocket-accept" => Header::SecWebSocketAccept,
        "sec-webSocket-key" => Header::SecWebSocketKey,
        "sec-webSocket-origin" => Header::SecWebSocketOrigin,
        "sec-webSocket-protocol" => Header::SecWebSocketProtocol,
        "sec-webSocket-version" => Header::SecWebSocketVersion,

        "server" => Header::Server,
        "set-cookie" => Header::SetCookie,

        "transfer-encoding" => Header::TransferEncoding,
        "upgrade" => Header::Upgrade,
        "user-agent" => Header::UserAgent,
        "vary" => Header::Vary,

        _ => Header::NONE,
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub enum ContentType {
    JS,
    PDF,
    JSON,
    XML,
    ZIP,
    Urlencode,

    ImgGif,
    ImgJpeg,
    ImgPng,

    FormData,

    CSS,
    CSV,
    Html,
    Text,

    MPEG,
    MP4,
    WEBM,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::JS => "application/js; charset=utf-8",
            ContentType::PDF => "application/pdf; charset=utf-8",
            ContentType::JSON => "application/json; charset=utf-8",
            ContentType::XML => "application/xml",
            ContentType::ZIP => "application/zip",
            ContentType::Urlencode => "application/x-www-form-urlencoded",

            ContentType::ImgGif => "image/gif",
            ContentType::ImgJpeg => "image/jpeg",
            ContentType::ImgPng => "image/png",

            ContentType::FormData => "multipart/form-data",

            ContentType::CSS => "text/css; charset=utf-8",
            ContentType::CSV => "text/csv; charset=utf-8",
            ContentType::Html => "text/html; charset=utf-8",
            ContentType::Text => "text/plain; charset=utf-8",

            ContentType::MPEG => "video/mpeg",
            ContentType::MP4 => "video/mp4",
            ContentType::WEBM => "video/webm",
        }
    }
}
