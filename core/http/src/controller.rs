use super::{request::Request, response::Response};

type ControllerBoxType = Box<dyn Fn(Request, Response) -> String + Sync + Send + 'static>;

pub trait ControllerT {
    fn controller(&self, _: Request, _: Response) -> String;
}

/// Http Controller
pub struct Controller {
    call_back: ControllerBoxType,
}

impl Controller {
    /// Returns new Controller
    pub fn new<F>(cb: F) -> Controller
    where
        F: Fn(Request, Response) -> String + Sync + Send + 'static,
    {
        let action = Box::new(cb);

        Controller { call_back: action }
    }
}

impl Controller {
    /// Returns CallBack clouse function
    pub fn get_action(&self) -> &ControllerBoxType {
        &self.call_back
    }

    /// Use CallBack function
    ///
    /// # Arguments
    ///
    /// * `request` - Request
    pub fn use_action(&self, request: Request) -> String {
        let cb = &self.get_action();

        cb(request, Response::new())
    }
}
