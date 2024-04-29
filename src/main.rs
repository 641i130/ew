mod encryption;
mod router;
use actix_web::{
    post,
    get,
    HttpResponse,
    HttpRequest,
    web,
    dev::Service,
    http::header::ContentType
};

#[post("/v1.0/auth/initialize")]
async fn gree_init(req: HttpRequest, body: String) -> HttpResponse { router::gree::initialize(req, body) }

#[get("/v1.0/auth/x_uid")]
async fn gree_uid(req: HttpRequest) -> HttpResponse { router::gree::uid(req) }

#[get("/v1.0/payment/productlist")]
async fn gree_payment(req: HttpRequest) -> HttpResponse { router::gree::payment(req) }

#[get("/v1.0/payment/subscription/productlist")]
async fn gree_payment_sub(req: HttpRequest) -> HttpResponse { router::gree::payment(req) }

#[get("/v1.0/payment/ticket/status")]
async fn gree_payment_ticket(req: HttpRequest) -> HttpResponse { router::gree::payment_ticket(req) }

#[get("/v1.0/moderate/keywordlist")]
async fn gree_moderate_keyword(req: HttpRequest) -> HttpResponse { router::gree::moderate_keyword(req) }

#[post("/v1.0/moderate/filtering/commit")]
async fn gree_moderate_commit(req: HttpRequest, body: String) -> HttpResponse { router::gree::moderate_commit(req, body) }

#[post("/v1.0/auth/authorize")]
async fn gree_authorize(req: HttpRequest, body: String) -> HttpResponse { router::gree::authorize(req, body) }

#[get("/v1.0/migration/code")]
async fn gree_migration_code(req: HttpRequest) -> HttpResponse { router::gree::migration_code(req) }

#[post("/v1.0/migration/code/verify")]
async fn gree_migration_verify(req: HttpRequest, body: String) -> HttpResponse { router::gree::migration_verify(req, body) }

#[post("/v1.0/migration/password/register")]
async fn gree_migration_password_register(req: HttpRequest, body: String) -> HttpResponse { router::gree::migration_password_register(req, body) }

#[get("/v1.0/payment/balance")]
async fn gree_balance(req: HttpRequest) -> HttpResponse { router::gree::balance(req) }

#[post("/v1.0/migration")]
async fn gree_migration(req: HttpRequest, body: String) -> HttpResponse { router::gree::migration(req, body) }

#[post("/api/debug/error")]
async fn debug_error(req: HttpRequest, body: String) -> HttpResponse { router::debug::error(req, body) }

#[post("/api/start")]
async fn start_start(req: HttpRequest, body: String) -> HttpResponse { router::start::start(req, body) }

#[post("/api/start/assetHash")]
async fn start_assethash(req: HttpRequest, body: String) -> HttpResponse { router::start::asset_hash(req, body) }

#[post("/api/dummy/login")]
async fn dummy_login(req: HttpRequest, body: String) -> HttpResponse { router::login::dummy(req, body) }

#[get("/api/user")]
async fn user(req: HttpRequest) -> HttpResponse { router::user::user(req) }

#[post("/api/user")]
async fn user_post(req: HttpRequest, body: String) -> HttpResponse { router::user::user_post(req, body) }

#[post("/api/chat/home")]
async fn chat_home(req: HttpRequest, body: String) -> HttpResponse { router::chat::home(req, body) }

#[post("/api/chat/talk/start")]
async fn chat_start(req: HttpRequest, body: String) -> HttpResponse { router::chat::start(req, body) }

#[post("/api/chat/talk/end")]
async fn chat_end(req: HttpRequest, body: String) -> HttpResponse { router::chat::end(req, body) }

#[post("/api/story/read")]
async fn story_read(req: HttpRequest, body: String) -> HttpResponse { router::story::read(req, body) }

#[post("/api/user/initialize")]
async fn user_initialize(req: HttpRequest, body: String) -> HttpResponse { router::user::initialize(req, body) }

#[post("/api/user/detail")]
async fn user_detail(req: HttpRequest, body: String) -> HttpResponse { router::user::detail(req, body) }

#[get("/api/gift")]
async fn gift_get(req: HttpRequest) -> HttpResponse { router::home::gift_get(req) }

#[post("/api/gift")]
async fn gift(req: HttpRequest, body: String) -> HttpResponse { router::user::gift(req, body) }

#[post("/api/deck")]
async fn user_deck(req: HttpRequest, body: String) -> HttpResponse { router::user::deck(req, body) }

#[get("/api/purchase")]
async fn purchase(req: HttpRequest) -> HttpResponse { router::purchase::purchase(req) }

#[post("/api/tutorial")]
async fn tutorial(req: HttpRequest, body: String) -> HttpResponse { router::tutorial::tutorial(req, body) }

#[post("/api/friend")]
async fn friend(req: HttpRequest, body: String) -> HttpResponse { router::friend::friend(req, body) }

#[get("/api/friend/ids")]
async fn friend_ids(req: HttpRequest) -> HttpResponse { router::friend::ids(req) }

#[post("/api/friend/search")]
async fn friend_search(req: HttpRequest, body: String) -> HttpResponse { router::friend::search(req, body) }

#[post("/api/friend/search/recommend")]
async fn friend_recommend(req: HttpRequest, body: String) -> HttpResponse { router::friend::recommend(req, body) }

#[post("/api/friend/request")]
async fn friend_request(req: HttpRequest, body: String) -> HttpResponse { router::friend::request(req, body) }

#[post("/api/friend/request/approve")]
async fn friend_approve(req: HttpRequest, body: String) -> HttpResponse { router::friend::approve(req, body) }

#[post("/api/friend/request/cancel")]
async fn friend_cancel(req: HttpRequest, body: String) -> HttpResponse { router::friend::cancel(req, body) }

#[post("/api/friend/delete")]
async fn friend_delete(req: HttpRequest, body: String) -> HttpResponse { router::friend::delete(req, body) }

#[post("/api/live/guest")]
async fn live_guest(req: HttpRequest, body: String) -> HttpResponse { router::live::guest(req, body) }

#[post("/api/live/mission")]
async fn live_mission(req: HttpRequest, body: String) -> HttpResponse { router::live::mission(req, body) }

#[post("/api/live/ranking")]
async fn live_ranking(req: HttpRequest, body: String) -> HttpResponse { router::live::ranking(req, body) }

#[post("/api/event")]
async fn event(req: HttpRequest, body: String) -> HttpResponse { router::event::event(req, body) }

#[post("/api/live/start")]
async fn live_start(req: HttpRequest, body: String) -> HttpResponse { router::live::start(req, body) }

#[post("/api/live/end")]
async fn live_end(req: HttpRequest, body: String) -> HttpResponse { router::live::end(req, body) }

#[post("/api/live/retire")]
async fn live_retire(req: HttpRequest, body: String) -> HttpResponse { router::live::retire(req, body) }

#[get("/api/live/clearRate")]
async fn live_clearrate(req: HttpRequest) -> HttpResponse { router::clear_rate::clearrate(req) }

#[post("/api/live/continue")]
async fn live_continue(req: HttpRequest, body: String) -> HttpResponse { router::live::continuee(req, body) }

#[get("/api/mission")]
async fn mission(req: HttpRequest) -> HttpResponse { router::mission::mission(req) }

#[post("/api/mission/clear")]
async fn mission_clear(req: HttpRequest, body: String) -> HttpResponse { router::mission::clear(req, body) }

#[get("/api/mission/receive")]
async fn mission_receive(req: HttpRequest, body: String) -> HttpResponse { router::mission::receive(req, body) }

#[get("/api/home")]
async fn home(req: HttpRequest) -> HttpResponse { router::home::home(req) }

#[get("/api/home/preset")]
async fn preset_get(req: HttpRequest) -> HttpResponse { router::home::preset_get(req) }

#[post("/api/home/preset")]
async fn preset(req: HttpRequest, body: String) -> HttpResponse { router::home::preset(req, body) }

#[post("/api/lottery/get_tutorial")]
async fn lottery_tutorial(req: HttpRequest, body: String) -> HttpResponse { router::lottery::tutorial(req, body) }

#[get("/api/lottery")]
async fn lottery(req: HttpRequest) -> HttpResponse { router::lottery::lottery(req) }

#[post("/api/lottery")]
async fn lottery_post(req: HttpRequest, body: String) -> HttpResponse { router::lottery::lottery_post(req, body) }

#[post("/api/login_bonus")]
async fn login_bonus(req: HttpRequest, body: String) -> HttpResponse { router::login::bonus(req, body) }

#[get("/api/notice/reward")]
async fn reward(req: HttpRequest) -> HttpResponse { router::notice::reward(req) }

#[post("/api/user/getmigrationcode")]
async fn getmigrationcode(req: HttpRequest, body: String) -> HttpResponse { router::user::get_migration_code(req, body) }

#[post("/api/user/registerpassword")]
async fn registerpassword(req: HttpRequest, body: String) -> HttpResponse { router::user::register_password(req, body) }

#[post("/api/user/migration")]
async fn migration(req: HttpRequest, body: String) -> HttpResponse { router::user::migration(req, body) }

#[post("/api/user/gglrequestmigrationcode")]
async fn gglrequestmigrationcode(req: HttpRequest, body: String) -> HttpResponse { router::user::request_migration_code(req, body) }

#[post("/api/user/gglverifymigrationcode")]
async fn gglverifymigrationcode(req: HttpRequest, body: String) -> HttpResponse { router::user::verify_migration_code(req, body) }

#[post("/api/serial_code")]
async fn serial_code(req: HttpRequest, body: String) -> HttpResponse { router::serial_code::serial_code(req, body) }

#[get("/api/serial_code/events")]
async fn serial_code_events(req: HttpRequest) -> HttpResponse { router::serial_code::events(req) }

#[get("/api/album/sif")]
async fn sif_album(req: HttpRequest) -> HttpResponse { router::user::sif(req) }

#[get("/web/announcement")]
async fn announcement(req: HttpRequest) -> HttpResponse { router::web::announcement(req) }

#[get("/api/home/announcement")]
async fn announcement_api(req: HttpRequest) -> HttpResponse { router::user::announcement(req) }

#[post("/api/card/reinforce")]
async fn card_reinforce(req: HttpRequest, body: String) -> HttpResponse { router::card::reinforce(req, body) }

#[post("/api/card/skill/reinforce")]
async fn card_skill_reinforce(req: HttpRequest, body: String) -> HttpResponse { router::card::skill_reinforce(req, body) }

#[post("/api/card/evolve")]
async fn card_evolve(req: HttpRequest, body: String) -> HttpResponse { router::card::evolve(req, body) }

#[get("/api/shop")]
async fn shop(req: HttpRequest) -> HttpResponse { router::shop::shop(req) }

#[post("/api/shop/buy")]
async fn shop_buy(req: HttpRequest, body: String) -> HttpResponse { router::shop::buy(req, body) }


#[post("/api/webui/login")]
async fn webui_login(req: HttpRequest, body: String) -> HttpResponse { router::webui::login(req, body) }

#[post("/api/webui/startLoginbonus")]
async fn webui_start_loginbonus(req: HttpRequest, body: String) -> HttpResponse { router::webui::start_loginbonus(req, body) }

#[get("/api/webui/userInfo")]
async fn webui_user(req: HttpRequest) -> HttpResponse { router::webui::user(req) }

#[get("/webui/logout")]
async fn webui_logout(req: HttpRequest) -> HttpResponse { router::webui::logout(req) }

#[post("/api/webui/import")]
async fn webui_import(req: HttpRequest, body: String) -> HttpResponse { router::webui::import(req, body) }

fn unhandled(req: HttpRequest) -> HttpResponse {
    router::webui::main(req)
}
#[get("/index.css")]
async fn css(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(ContentType(mime::TEXT_CSS))
        .body(include_str!("../webui/dist/index.css"))
}
#[get("/index.js")]
async fn js(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(ContentType(mime::APPLICATION_JAVASCRIPT_UTF_8))
        .body(include_str!("../webui/dist/index.js"))
}


async fn log_unknown_request(req: HttpRequest, body: String) -> HttpResponse {
    if !req.path().starts_with("/api") {
        return unhandled(req);
    }
    if body != String::new() {
        println!("{}", encryption::decrypt_packet(&body).unwrap());
    }
    println!("Unhandled request: {}", req.path());
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    let rv = HttpServer::new(|| App::new()
        .wrap_fn(|req, srv| {
            println!("Request: {}", req.path());
            srv.call(req)
        })
        .service(serial_code)
        .service(shop)
        .service(shop_buy)
        .service(css)
        .service(js)
        .service(webui_start_loginbonus)
        .service(webui_import)
        .service(webui_logout)
        .service(webui_user)
        .service(webui_login)
        .service(card_evolve)
        .service(card_skill_reinforce)
        .service(card_reinforce)
        .service(announcement_api)
        .service(announcement)
        .service(sif_album)
        .service(preset)
        .service(preset_get)
        .service(gree_init)
        .service(gree_uid)
        .service(gree_authorize)
        .service(gree_payment)
        .service(gree_payment_ticket)
        .service(gree_payment_sub)
        .service(gree_moderate_keyword)
        .service(gree_moderate_commit)
        .service(gree_migration_code)
        .service(gree_migration_verify)
        .service(gree_migration_password_register)
        .service(gree_balance)
        .service(gree_migration)
        .service(debug_error)
        .service(login_bonus)
        .service(reward)
        .service(live_continue)
        .service(live_guest)
        .service(live_mission)
        .service(live_ranking)
        .service(live_clearrate)
        .service(live_start)
        .service(live_end)
        .service(live_retire)
        .service(chat_home)
        .service(chat_end)
        .service(chat_start)
        .service(story_read)
        .service(event)
        .service(purchase)
        .service(user_initialize)
        .service(start_start)
        .service(tutorial)
        .service(lottery_tutorial)
        .service(lottery_post)
        .service(lottery)
        .service(friend)
        .service(friend_search)
        .service(friend_recommend)
        .service(friend_ids)
        .service(friend_request)
        .service(friend_approve)
        .service(friend_cancel)
        .service(friend_delete)
        .service(mission)
        .service(mission_clear)
        .service(mission_receive)
        .service(home)
        .service(start_assethash)
        .service(user)
        .service(user_post)
        .service(user_deck)
        .service(user_detail)
        .service(dummy_login)
        .service(getmigrationcode)
        .service(registerpassword)
        .service(migration)
        .service(gglrequestmigrationcode)
        .service(gglverifymigrationcode)
        .service(serial_code_events)
        .service(gift)
        .service(gift_get)
        .default_service(web::route().to(log_unknown_request)))
        .bind(("0.0.0.0", 8080))?
        .run();
    println!("Server started: http://127.0.0.1:{}", 8080);
    rv.await
}



/*
fn main() {
    let base64_input = "MX2tzmKTxY7EsV46rYFZuAfxeY0tPHuZ0etG15WsK1MAzs/U0WUXE4bJZINrEvCxqqUbvCYxhDtXp3HoeH/zDXtnW183aF/aYycmUW3aAF6zyio4/PJoqFl7EGET37ruotoQ9Teof2PXpXraF94diw==";
    match decrypt_packet(base64_input) {
        Ok(decrypted_json) => {
            // Process the decrypted JSON
            println!("Decrypted JSON: {}", decrypted_json);
        }
        Err(err) => {
            eprintln!("Error decrypting packet: {}", err);
        }
    }
}

*/

/*

async fn make_post_request(url: &str, body: &str, headers: &HeaderMap) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut response = client
        .post(url)
        .body(body.to_string());
    
    for (name, value) in headers.iter() {
        if name == "Accept-Encoding" {continue;};
        if name == "host" {
            response = response.header("host", "api-sif2.lovelive-sif2.com");
            continue;
        };
        println!("{}: {}", name, value.to_str().unwrap());
        response = response.header(name, value.to_str().unwrap());
    }
    
    let response_body = response.send().await?.text().await?;
    
    Ok(response_body)
}

async fn make_get_request(url: &str, headers: &HeaderMap) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut response = client.get(url);
    for (name, value) in headers.iter() {
        if name == "Accept-Encoding" {continue;};
        if name == "host" {
            response = response.header("host", "api.app.lovelive-sif2.bushimo.jp");
            continue;
        };
        response = response.header(name, value.to_str().unwrap());
    }
    let response_body = response.send().await?.text().await?;
    Ok(response_body)
}

async fn log_unknown_request(req: HttpRequest, body: String) -> HttpResponse {
    if body != String::new() {
        println!("req: {}", encryption::decrypt_packet(&body).unwrap_or(String::new()));
        let resp = make_post_request(&format!("https://api-sif2.lovelive-sif2.com{}", req.path()), &body, req.headers()).await.unwrap();
        
        //println!("Unhandled request: {} {}", req.path(), body);
        println!("resp: {}", encryption::decrypt_packet(&resp).unwrap_or(String::new()));
        HttpResponse::Ok().body(resp)
    } else {
        let resp = make_get_request(&format!("https://api-sif2.lovelive-sif2.com{}", req.path()), req.headers()).await.unwrap();
        
        //println!("Unhandled request: {} {}", req.path(), body);
        println!("resp: {}", encryption::decrypt_packet(&resp).unwrap_or(String::new()));
        HttpResponse::Ok().body(resp)
        
    }
}*/
