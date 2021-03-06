//extern crate awf_codegen;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate derive_new;
extern crate awf_help;
use actix_web::{dev, web, App, Error, FromRequest, HttpRequest, HttpServer, Responder};
use actix_web::error::ErrorBadRequest;
//use h010::config;
//use awf_codegen::route;
use awf_help::{config, route, route_res, ServiceFactory};
use serde::Serialize;
use std::sync::{Arc, Mutex};



#[derive(Serialize)]
struct MyObj {
    name: String,
}
//use awf_help::ServiceFactory;
#[route(GET, "/")]
fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    let hello = req.headers().get("hello").unwrap().to_str().unwrap();
    //format!("Hello {}!", &name)
    //web::json::
    web::Json([MyObj {
        name: hello.to_string(),
    }])
    .with_header("x-version", "1.2.3")
}

#[route(POST, "/")]
fn greet2(req: HttpRequest) -> String {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("post Hello {}!", &name)
}

#[route(POST, "/1123/{name}")]
fn greet3(req: HttpRequest, name: web::Path<String>) -> String {
    format!("post Hello {}!", &name)
}
struct Hello{
    //#[new(value = "hello".to_string())]
    //name: String,
}

#[route_res("/api/auth")]
impl Hello {
    fn get(req: HttpRequest) -> String {
        format!("get Hello !")
    }
    fn post(req: HttpRequest) -> String {
        format!("post Hello !")
    }
}
/*
impl FromRequest for Hello {
    type Error = Error;
    type Future =Result<Self, Self::Error>; //Self;//Result<Self, Self::Error>;
    type Config = ();
    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        //return Ok(MyObj {
        //    name: "1234".to_string(),               
        //});
        //return Ok(Hello{name: "1234".to_string()});
        //let x = mapRes.lock().unwrap().get("Hello").unwrap();
        //let h =x.downcast_ref::<&Hello>().unwrap();
        //return h; //Ok(h);
        return Ok(Hello{})
        //return Err(ErrorBadRequest("kkkk"));
    }
}*/

fn main() {
//mapRes.lock().unwrap().insert("hello",Arc::new(Hello::new("test".to_string())));
    HttpServer::new(|| App::new().configure(config))
        .bind("127.0.0.1:8100")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
    //println!("Hello, world!");
}
