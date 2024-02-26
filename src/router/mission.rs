use json;
use json::object;
use crate::router::global;
use actix_web::{HttpResponse, HttpRequest, http::header::HeaderValue};
use crate::router::userdata;

pub fn mission(req: HttpRequest) -> HttpResponse {
    let blank_header = HeaderValue::from_static("");
    let key = req.headers().get("a6573cbe").unwrap_or(&blank_header).to_str().unwrap_or("");
    let uid = req.headers().get("aoharu-user-id").unwrap_or(&blank_header).to_str().unwrap_or("");
    let user = userdata::get_acc(key, uid);
    
    let resp = object!{
        "code": 0,
        "server_time": global::timestamp(),
        "data": {
            "mission_list": user["live_mission_list"].clone()
        }
    };
    global::send(resp)
}