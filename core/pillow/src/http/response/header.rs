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
