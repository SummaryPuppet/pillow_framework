use crate::{request::Request, response::Response};

pub type MiddlewareType = Box<dyn Fn(&Request, &Response)>;

pub struct Middleware {
    pub cb: MiddlewareType,
}

impl Middleware {
    pub fn new<F>(controller: F) -> Middleware
    where
        F: Fn(&Request, &Response) + 'static,
    {
        let box_cb = Box::new(controller);

        Middleware { cb: box_cb }
    }
}

impl Middleware {
    pub fn use_middleware(&self, request: &Request, response: &Response) {
        let fun = &self.cb;

        fun(request, response);
    }
}
