use async_std::io::{Read, Write};
use async_std::{io::ReadExt, net::TcpListener};
use colored::Colorize;
use futures::{AsyncWriteExt, StreamExt};

use crate::http::http_methods::HttpMethods;
use crate::http::request::Request;
use crate::routing::routes::Routes;

pub(crate) mod mock_test;

pub async fn server_listen(port: String, routes: &Routes) {
    let listener: TcpListener = match TcpListener::bind(port).await {
        Ok(listener) => listener,
        Err(error) => panic!("{} {}", "Pillow-TcpListener".red(), error),
    };

    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();

            handle_connection(stream, &routes).await;
        })
        .await
}

async fn handle_connection(mut stream: impl Read + Write + Unpin, routes: &Routes) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer).await {
        Ok(_) => {}
        Err(error) => panic!("{} {}", "Pillow-TcpStream".red(), error),
    };

    let mut request = Request::new(&buffer);

    let res = match request.method {
        HttpMethods::GET => &routes.get,
        HttpMethods::POST => &routes.post,
        HttpMethods::PUT => &routes.put,
        HttpMethods::DELETE => &routes.delete,
    };

    let mut r = String::new();

    match res.iter().position(|route| route.url == request.path) {
        Some(index) => {
            let route = &res[index];
            r = route.controller.use_action(request);
        }
        None => {
            let routes_params: Vec<_> = res.iter().filter(|route| route.has_parameters()).collect();

            for route in routes_params {
                let path: Vec<_> = route.regex_complete.split(&route.url).collect();

                let path_param: Vec<_> = route.regex_words.find_iter(&request.path).collect();
                let request_param = path_param[1].as_str();

                if request.path.starts_with(path[0]) {
                    request.add_param(
                        route.get_parameters()[0].to_owned(),
                        request_param.to_string(),
                    );

                    r = route.controller.use_action(request);

                    break;
                }
            }
        }
    }

    stream.write_all(r.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
