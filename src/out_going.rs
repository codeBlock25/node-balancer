use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web::http::header::HeaderMap;
use reqwest::{Method, Request, Url};
use crate::util::{OutUrl, respond};


pub(crate) async fn get_out(req: HttpRequest, data: web::Data<OutUrl>) -> impl Responder {
    let mut headermap = HeaderMap::new();
    for (key, val) in req.headers() {
        headermap.insert(key.clone(), val.clone());
    }

    let path = &req.uri().to_string()[1..];
    let uri = match query {
        "" => Url::parse(&format!("{}{}", data.url.as_str(), path)).unwrap(),
        _ => Url::parse(&format!(
            "{}{}?{}",
            data.url.as_str(),
            path,
            req.query_string()
        ))
            .unwrap(),
    };

    let client = &data.out_client.clone();
    let request = Request::new(Method::GET, uri);

    let handle = async_std::task::spawn(client.execute(request));
    let res = match handle.await {
        Ok(res) => res,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error requesting path: {}", e))
        }
    };

    respond(res).await
}