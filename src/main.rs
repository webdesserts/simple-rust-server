extern crate iron;
extern crate rustc_serialize;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use rustc_serialize::{json, Encodable};
use urlencoded::{UrlEncodedQuery, UrlDecodingError};
use urlencoded::UrlDecodingError::{EmptyQuery, BodyError};

fn main() {
    Iron::new(params_to_json).http("localhost:3000").unwrap();
}

fn params_to_json(req: &mut Request) -> IronResult<Response> {
    // Extract the decoded data as hashmap, using the UrlEncodedQuery plugin.
    return match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => send_as_json(hashmap),
        Err(e) => send_error(e)
    };
}

fn send_as_json<T: Encodable>(encodable: &T) -> IronResult<Response> {
    return Ok(Response::with((status::Ok, json::encode(encodable).unwrap())))
}

fn send_error(error: UrlDecodingError) -> IronResult<Response> {
    match error {
        EmptyQuery => Ok(Response::with((status::BadRequest, "Expected Parameters"))),
        BodyError(..) => Ok(Response::with((status::BadRequest, "fix yo body!")))
    }
}


