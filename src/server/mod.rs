use async_std::{io::ReadExt, net::TcpListener};
use futures::{AsyncWriteExt, StreamExt};

use crate::http::{response::Response, routes::Routes};

pub async fn server_listen(port: String, routes: &Routes) {
    let listener = TcpListener::bind(port).await.unwrap();

    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let mut stream = stream.unwrap();

            let mut buffer = [0; 1024];
            let mut headers = [httparse::EMPTY_HEADER; 16];

            stream.read(&mut buffer).await.unwrap();

            let mut request = httparse::Request::new(&mut headers);
            let _res = request.parse(&buffer);

            let res = match request.method.unwrap() {
                "GET" => &routes.get,
                "POST" => &routes.post,
                "PUT" => &routes.put,
                "DELETE" => &routes.delete,
                _ => &routes.get,
            };

            match res.get(request.path.unwrap()) {
                Some(res) => {
                    let r = res(request, Response::new());
                    stream.write_all(r.as_bytes()).await.unwrap();
                }
                None => {}
            }
        })
        .await
}
