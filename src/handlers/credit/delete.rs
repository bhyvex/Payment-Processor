use iron::prelude::*;
use iron::{headers, status};
use iron::Handler;
use diesel::types::structs::data_types::PgTimestamp;
use rustc_serialize::json::{ToJson};
use urlencoded::{UrlDecodingError, UrlEncodedQuery};
use router::{Router};
use bodyparser::Json;
use models::credit;
use uri_params::{self, URIParams};
use std::collections::HashMap;

pub struct Delete;

impl Handler for Delete {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let params = get_params(req);

        let id = credit::delete_from_params(params);

        let mut response = Response::new().set(((status::Ok), id.to_json().to_string()));
        response.headers.set(headers::ContentType::json());

        Ok(response)
    }
}
fn get_params(req: &mut Request) -> credit::CreditQueryParams {
    let credit_id = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("").to_string();

    let json_body = req.get::<Json>();
    let mut user_id: Option<i64> = None;
    match json_body {
        Ok(Some(json_body)) => user_id = json_body.find("user_id").unwrap().as_i64(),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    };
    let mut params = URIParams::new();
    params.insert("id".to_string(), vec![credit_id]);
    params.insert("user_id".to_string(), vec![user_id.unwrap().to_string() ]);
    match credit::CreditQueryParams::from_delete_request(&params) {
        Ok(query_params) => query_params,
        Err(err) => credit::CreditQueryParams { 
            id: Some(-1),
            user_id: Some(-1),
            created_date_greater_than: None,
            created_date_less_than: None
        }
    }
}
