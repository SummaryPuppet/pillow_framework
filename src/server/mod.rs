use async_std::{io::ReadExt, net::TcpListener};
use futures::{AsyncWriteExt, StreamExt};

use crate::http::request::Request;
use crate::http::response::Response;
use crate::routing::routes::Routes;

pub async fn server_listen(port: String, routes: &Routes) {
    let listener = TcpListener::bind(port).await.unwrap();

    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let mut stream = stream.unwrap();

            let mut buffer = [0; 1024];

            stream.read(&mut buffer).await.unwrap();

            let mut request = Request::new(&buffer);

            let res = match request.method.as_str() {
                "GET" => &routes.get,
                "POST" => &routes.post,
                "PUT" => &routes.put,
                "DELETE" => &routes.delete,
                _ => &routes.get,
            };

            let mut r = String::new();

            match res.iter().position(|route| route.url == request.path) {
                Some(index) => {
                    let route = &res[index];
                    r = route.use_action(request, Response::new());
                }
                None => {
                    let routes_params: Vec<_> =
                        res.iter().filter(|route| route.has_parameters()).collect();

                    for route in routes_params {
                        let path: Vec<_> = route.regex_complete.split(&route.url).collect();

                        let path_param: Vec<_> =
                            route.regex_words.find_iter(&request.path).collect();
                        let request_param = path_param[1].as_str();

                        if request.path.starts_with(path[0]) {
                            request.add_param(
                                route.get_parameters()[0].to_owned(),
                                request_param.to_string(),
                            );

                            r = route.use_action(request, Response::new());

                            break;
                        }
                    }
                }
            }

            stream.write_all(r.as_bytes()).await.unwrap();
            stream.flush().await.unwrap();
        })
        .await
}
