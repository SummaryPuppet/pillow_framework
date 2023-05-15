use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusCode {
    Information(Information),
    Successfull(Successfull),
    Redirection(Redirection),
    ClientError(ClientError),
    ServerError(ServerError),
}

impl AsStr for StatusCode {
    fn as_str(&self) -> &'static str {
        match self {
            StatusCode::Information(info) => info.as_str(),
            StatusCode::Successfull(success) => success.as_str(),
            StatusCode::Redirection(redirect) => redirect.as_str(),
            StatusCode::ClientError(client) => client.as_str(),
            StatusCode::ServerError(server) => server.as_str(),
        }
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub trait AsStr {
    fn as_str(&self) -> &'static str {
        match self {
            _ => "",
        }
    }
}

/// Information Messages
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Information {
    /// This interim response indicates that the client should continue the request or ignore the response if the request is already finished.
    Continue,
    /// This code is sent in response to an Upgrade request header from the client and indicates the protocol the server is switching to.
    SwitchingProtocols,
}

impl AsStr for Information {
    fn as_str(&self) -> &'static str {
        match self {
            Information::Continue => "100 Continue",
            Information::SwitchingProtocols => "101 Switching Protocols",
        }
    }
}

/// Successfull Messages
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Successfull {
    /// The request succeeded. The result meaning of "success" depends on the HTTP method:
    ///
    /// - GET: The resource has been fetched and transmitted in the message body.
    ///
    /// - HEAD: The representation headers are included in the response without any message body.
    ///
    /// - PUT or POST: The resource describing the result of the action is transmitted in the message body.
    ///
    /// - TRACE: The message body contains the request message as received by the server.
    OK,
    /// The request succeeded, and a new resource was created as a result.
    /// This is typically the response sent after POST requests, or some PUT requests.
    Created,
    /// The request has been received but not yet acted upon.
    /// It is noncommittal, since there is no way in HTTP to later send an asynchronous response indicating the outcome of the request.
    /// It is intended for cases where another process or server handles the request, or for batch processing.
    Accepted,
    /// This response code is used when the Range header is sent from the client to request only part of a resource.
    PartialContent,
}

impl AsStr for Successfull {
    fn as_str(&self) -> &'static str {
        match self {
            Successfull::OK => "200 OK",
            Successfull::Created => "201 Created",
            Successfull::Accepted => "202 Accepted",
            Successfull::PartialContent => "206 Partial Content",
        }
    }
}

/// Redirect
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Redirection {
    /// The request has more than one possible response.
    /// The user agent or user should choose one of them.
    /// (There is no standardized way of choosing one of the responses, but HTML links to the possibilities are recommended so the user can pick.)
    MultipleChoices,
    /// The URL of the requested resource has been changed permanently. The new URL is given in the response.
    MovedPermantely,
    /// This response code means that the URI of requested resource has been changed temporarily.
    /// Further changes in the URI might be made in the future.
    /// Therefore, this same URI should be used by the client in future requests.
    Found,
    /// The server sent this response to direct the client to get the requested resource at another URI with a GET request.
    SeeOther,
    /// This is used for caching purposes.
    /// It tells the client that the response has not been modified, so the client can continue to use the same cached version of the response.
    NotModified,
    /// The server sends this response to direct the client to get the requested resource at another URI with same method that was used in the prior request.
    /// This has the same semantics as the 302 Found HTTP response code, with the exception that the user agent must not change the HTTP method used: if a POST was used in the first request, a POST must be used in the second request.
    TemporalyRedirect,
    /// This means that the resource is now permanently located at another URI, specified by the Location: HTTP Response header.
    /// This has the same semantics as the 301 Moved Permanently HTTP response code, with the exception that the user agent must not change the HTTP method used:
    /// if a POST was used in the first request, a POST must be used in the second request.
    PermanetRedirect,
}

impl AsStr for Redirection {
    fn as_str(&self) -> &'static str {
        match self {
            Redirection::MultipleChoices => "300 Multiple Choices",
            Redirection::MovedPermantely => "301 Moved Permantely",
            Redirection::Found => "302 Found",
            Redirection::SeeOther => "303 See Other",
            Redirection::NotModified => "304 Not Modified",
            Redirection::TemporalyRedirect => "307 Temporaly Redirect",
            Redirection::PermanetRedirect => "308 Permanet Redirect",
        }
    }
}

/// Client Error Messages
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientError {
    /// The server cannot or will not process the request due to something that is perceived to be a client error (e.g., malformed request syntax, invalid request message framing, or deceptive request routing).
    BadRequest,
    /// Although the HTTP standard specifies "unauthorized", semantically this response means "unauthenticated".
    /// That is, the client must authenticate itself to get the requested response.
    Unautorized,
    /// The client does not have access rights to the content; that is, it is unauthorized, so the server is refusing to give the requested resource.
    /// Unlike 401 Unauthorized, the client's identity is known to the server.
    Forbiden,
    /// The server cannot find the requested resource.
    /// In the browser, this means the URL is not recognized.
    /// In an API, this can also mean that the endpoint is valid but the resource itself does not exist.
    /// Servers may also send this response instead of 403 Forbidden to hide the existence of a resource from an unauthorized client.
    /// This response code is probably the most well known due to its frequent occurrence on the web.
    NotFound,
    /// The request method is known by the server but is not supported by the target resource.
    /// For example, an API may not allow calling DELETE to remove a resource.
    MethodNotAllowed,
    /// This response is sent when the web server, after performing server-driven content negotiation, doesn't find any content that conforms to the criteria given by the user agent.
    NotAcceptable,
    /// This response is sent on an idle connection by some servers, even without any previous request by the client.
    /// It means that the server would like to shut down this unused connection.
    /// This response is used much more since some browsers, like Chrome, Firefox 27+, or IE9, use HTTP pre-connection mechanisms to speed up surfing.
    /// Also note that some servers merely shut down the connection without sending this message.
    RequestTimeout,
}

impl AsStr for ClientError {
    fn as_str(&self) -> &'static str {
        match self {
            ClientError::BadRequest => "400 Bad Request",
            ClientError::Unautorized => "401 Unautorized",
            ClientError::Forbiden => "403 Forbiden",
            ClientError::NotFound => "404 Not Found",
            ClientError::MethodNotAllowed => "405 Method Not Allowed",
            ClientError::NotAcceptable => "406 Not Acceptable",
            ClientError::RequestTimeout => "408 Request Timeout",
        }
    }
}

/// Server Error Messages
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerError {
    /// The server has encountered a situation it does not know how to handle.
    InternalServerError,
    /// The request method is not supported by the server and cannot be handled.
    /// The only methods that servers are required to support (and therefore that must not return this code) are GET and HEAD.
    NotImplemented,
    /// This error response means that the server, while working as a gateway to get a response needed to handle the request, got an invalid response.
    BadGateway,
    /// This error response is given when the server is acting as a gateway and cannot get a response in time.
    GatewayTimeout,
}

impl AsStr for ServerError {
    fn as_str(&self) -> &'static str {
        match self {
            ServerError::InternalServerError => "500 Internal Server Error",
            ServerError::NotImplemented => "501 Not Implemented",
            ServerError::BadGateway => "502 Bad Gateway",
            ServerError::GatewayTimeout => "504 Gateway Timeout",
        }
    }
}
