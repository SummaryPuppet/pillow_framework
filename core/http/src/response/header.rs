use std::fmt;

#[derive(PartialEq, Eq, Hash)]
pub enum Header {
    /// Cors
    AccessControlAllowOrigin,
    /// How to encoding to send files
    AcceptEncoding,
    /// Languages
    AcceptLanguages,
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
    /// Last Modified
    LastModified,
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
    /// Accept Encoding
    Vary,
}

impl Header {
    pub fn as_str(&self) -> &'static str {
        match self {
            Header::AccessControlAllowOrigin => "Access-Control-Allow-Origin",
            Header::AcceptEncoding => "Accept-Encoding",
            Header::AcceptLanguages => "Accept-Languages",
            Header::AcceptRanges => "Accept-Ranges",

            Header::CacheControl => "Cache-Control",
            Header::Cookie => "Cookie",
            Header::Connection => "Connection",
            Header::ContentLength => "Content-Length",
            Header::ContentSegurityPolicy => "Content-Segurity-Policy",
            Header::ContentType => "Content-Type",

            Header::Date => "Date",
            Header::ETag => "ETag",
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
            Header::Vary => "Vary",
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

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
