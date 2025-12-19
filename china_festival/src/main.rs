pub mod game;
pub mod loger;
pub mod object_list;
pub mod utils;
pub mod api;


use actix_files::NamedFile;
use std::str;
use std::path::Path;
use actix_web::{middleware, web, App, HttpServer, HttpRequest, HttpResponse, Result, Error};
use actix_web::middleware::Compress;
use std::sync::{Arc, Mutex};
use serde_json::Value;
use uuid::Uuid;
//use sqlx::postgres::PgPoolOptions;
//use sea_orm::Database;
//use std::env;

async fn handle_post() -> HttpResponse {
    HttpResponse::Ok()
    .append_header(("access-control-allow-headers", "DNT,X-CustomHeader,Keep-Alive,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type"))
    .append_header(("access-control-allow-methods", "GET, POST, PUT, OPTIONS, HEAD"))
    .append_header(("access-control-allow-origin", "https://example.com"))
    .append_header(("access-control-max-age", "1728000"))
    .append_header(("alt-svc", "h3=\":8080\"; ma=86400"))
    .append_header(("cache-control", "no-store, no-cache, must-revalidate, max-age=0"))
    .append_header(("content-security-policy", "default-src 'self'; img-src 'self' data:; script-src 'self'; style-src 'self'; frame-ancestors 'none';"))
    .append_header(("strict-transport-security", "max-age=31536000; includeSubDomains; preload"))
    .append_header(("content-type", "text/html; charset=utf-8"))
    .append_header(("date", chrono::Utc::now().to_rfc2822()))
    .finish()
}

async fn serve_china_festival(file_name: &str) -> Result<NamedFile> {
    let file = NamedFile::open(file_name).map_err(|e| {
        actix_web::error::ErrorInternalServerError(serde_json::json!({
            "message": e.to_string(),
            "errorCode": "ERR_INTERNAL_ERROR"
        }))
    })?;
    Ok(file)
}
async fn dynamic_file_handler(req: HttpRequest, req_path: web::Path<String>) -> Result<HttpResponse> {
    let file_path = format!("../data/ChinaFestival/{}", req_path);
    if Path::new(&file_path).exists() {
        let named_file = NamedFile::open(&file_path)?.use_last_modified(true);
        let content_type = match Path::new(&file_path).extension().and_then(|ext| ext.to_str()) {
            Some("js") => "application/javascript",
            Some("css") => "text/css",
            Some("html") => "text/html; charset=utf-8",
            Some("json") => "application/json",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            _ => "application/octet-stream", 
        };
        let mut response = named_file.into_response(&req);
        response.headers_mut().insert(
            actix_web::http::header::CONTENT_TYPE,
            content_type.parse().unwrap(),
        );
        Ok(response)
    } else {Err(actix_web::error::ErrorNotFound(serde_json::json!({"message": "File not found","path": req_path.into_inner()})))}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> 
{
    loger::init_logs();
    //let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {log::error!("Can not load DATABASE_URL.");std::process::exit(1);});
    //let db_connection = Database::connect(&database_url).await.unwrap_or_else(|_| {log::error!("Can not connect to database.");std::process::exit(1);});
    //let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {log::error!("Can not load DATABASE_URL."); std::process::exit(1);});
    //let db_pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await.unwrap_or_else(|_| {log::error!("Can not connect to database."); std::process::exit(1);});
    let shared_game_list = object_list::ObjectsList::<game::models::model::Game>::new();
    HttpServer::new(move || {
            App::new()
            .app_data(web::Data::new(shared_game_list.clone())) 
            //.app_data(web::Data::new(db_pool.clone()))
            //.app_data(web::Data::new(db_connection.clone()))
            .wrap(middleware::Logger::default())
            .wrap(Compress::default())
            .route("/", web::get().to(|| async {HttpResponse::Ok().body("Works!")}))
            .route("/play/china_festival", web::get().to(|| async {serve_china_festival("../data/ChinaFestival/china_festival.html").await}))
            .route("/bng-site-eu/cdn_domain/", web::get().to(|| async {actix_web::HttpResponse::Ok().json(serde_json::json!({"country": "DE", "domains": ["//localhost:8080"]}))}))
            .route("/static/games/cdn_measure.png", web::head().to(|| async {serve_china_festival("../data/ChinaFestival/cdn_measure.png").await}))
            .route("/bng-site-eu/log/china_festival/desktop/measure_checker/demo/", web::post().to(|| async {actix_web::HttpResponse::Ok().json(serde_json::json!({}))}))
            .route("/bng-site-eu/log/", web::post().to(|| async {actix_web::HttpResponse::Ok().json(serde_json::json!({}))}))
            .route("/bng-site-eu/snt/", web::post().to(handle_post))
            .route("/bng-site-eu/gs/china_festival/desktop/QUEUE/demo/", web::post().to(execute))
            .route("/bng-site-eu/gs/china_festival/mobile/QUEUE/demo/", web::post().to(execute))
            .route("/{tail:.*}", web::get().to(dynamic_file_handler))
        }
    )
    .bind("0.0.0.0:8080").unwrap_or_else(|_| {log::error!("Can not add port 8080."); std::process::exit(1);}) 
    //.bind_openssl("0.0.0.0:8443", ssl_acceptor).unwrap_or_else(|_| {log::error!("Can not add ssl port 8443."); std::process::exit(1);}) 
    .run()
    .await.unwrap_or_else(|_| {log::error!("Can not start server."); std::process::exit(1);});
    Ok(())
}

pub async fn execute(a_http_req: HttpRequest, a_body: web::Bytes, a_shared_game_list: web::Data<object_list::ObjectsList<game::models::model::Game>>) -> Result<HttpResponse, Error> {
    let response;
    let l_command = a_http_req.query_string().split('&').find_map(|pair| {let mut parts = pair.split('='); if parts.next() == Some("gsc") {parts.next()} else {None}});

    let json: Value = serde_json::from_slice(&a_body).map_err(|e| utils::err_http_responses("BadRequest", &e.to_string(), "ERR_INVALID_BODY"))?;
    let l_game_key: String;
    if l_command == Some("login") {l_game_key = Uuid::new_v5(&Uuid::new_v5(&Uuid::parse_str("0797cb7a-1452-4407-a561-bf4f0f4fb2b1").unwrap(),  "china_festival".as_bytes()), json.get("token").and_then(|value| value.as_str()).ok_or(utils::err_http_responses("BadRequest", "Token is missing.", "ERR_INVALID_TOKEN"))?.as_bytes()).to_string();}
    else {l_game_key = (json.get("session_id").and_then(|value| value.as_str()).ok_or(utils::err_http_responses("BadRequest", "Token is missing.", "ERR_INVALID_TOKEN"))?).to_string()}
    
    let arc_mutex_game_option: Option<Arc<Mutex<game::models::model::Game>>> = {a_shared_game_list.items.lock().map_err(|e| utils::err_http_responses("ErrorLocked", &e.to_string(), "ERR_RESOURCE_LOCKED"))?.get(&l_game_key).cloned()};
    if let Some(arc_mutex_game) = arc_mutex_game_option {
        let mut l_lock_game = arc_mutex_game.lock().map_err(|e| utils::err_http_responses("ErrorLocked", &e.to_string(), "ERR_RESOURCE_LOCKED"))?;
        response = game::actions::execute(l_command, str::from_utf8(&a_body)?.to_string(), &mut *l_lock_game, None).await.map_err(|e|  utils::err_http_responses("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?
    } 
    else {
        if l_command == Some("login") {
            let mut l_game = game::models::model::Game::new(); 
            l_game.session_id = l_game_key.clone();
            l_game.modes = ["auto".to_string(), "play".to_string()].into_iter().map(Into::into).collect();
            response = game::actions::execute(l_command, str::from_utf8(&a_body)?.to_string(), &mut l_game, None).await.map_err(|e|  utils::err_http_responses("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?;
            a_shared_game_list.items.lock().map_err(|e| utils::err_http_responses("ErrorInternalServerError", &e.to_string(), "ERR_INTERNAL_ERROR"))?.insert(l_game.session_id.clone(),Arc::new(Mutex::new(l_game.clone())));
        } else {return Err(utils::err_http_responses("ErrorUnauthorized", "Session expired. Please log in again.", "ERR_UNAUTHORIZED"))}
    }
    Ok(HttpResponse::Ok().json(response))
}

#[cfg(test)]
mod tests {
    use crate::game;
    use serde_json; 
    use serde_json::json;
    use serde_json::Value;
    use std::fs::File;
    use std::io::BufReader;
    use std::thread::sleep;
    use pretty_assertions::assert_eq;
    use indicatif::ProgressBar;
    use std::time::Duration;
    
    fn replace_bs_v_zero(json: &mut Value) {
        if let Some(bs_v) = json.get_mut("bs_values") {
            if let Value::Array(ref mut outer_array) = bs_v {
                for row in outer_array.iter_mut() {
                    if let Value::Array(ref mut inner_array) = row {
                        for cell in inner_array.iter_mut() {
                            if let Value::Number(ref n) = cell {
                                if let Some(f) = n.as_f64() {
                                    if f.fract() == 0.0 {
                                        // Преобразуем значение в i64 и заменяем ячейку
                                        let int_val = f as i64;
                                        *cell = Value::Number(serde_json::Number::from(int_val));
                                    }}}}}}}}
        if let Some(bs_v) = json.get_mut("bs_v") {
            if let Value::Array(ref mut outer_array) = bs_v {
                for row in outer_array.iter_mut() {
                    if let Value::Array(ref mut inner_array) = row {
                        for cell in inner_array.iter_mut() {
                            if let Value::Number(ref n) = cell {
                                if let Some(f) = n.as_f64() {
                                    if f.fract() == 0.0 {
                                        // Преобразуем значение в i64 и заменяем ячейку
                                        let int_val = f as i64;
                                        *cell = Value::Number(serde_json::Number::from(int_val));
                                    }}}}}}}}
        if let Some(bs_v) = json.get_mut("orig_bs_v") {
            if let Value::Array(ref mut outer_array) = bs_v {
                for row in outer_array.iter_mut() {
                    if let Value::Array(ref mut inner_array) = row {
                        for cell in inner_array.iter_mut() {
                            if let Value::Number(ref n) = cell {
                                if let Some(f) = n.as_f64() {
                                    if f.fract() == 0.0 {
                                        // Преобразуем значение в i64 и заменяем ячейку
                                        let int_val = f as i64;
                                        *cell = Value::Number(serde_json::Number::from(int_val));
                                    }}}}}}}}
        if let Some(boost_values) = json.get_mut("boost_values") {
            if let Value::Array(ref mut arr) = boost_values {
                for item in arr.iter_mut() {
                    if let Value::Object(ref mut obj) = item {
                        if let Some(bs_v_value) = obj.get_mut("bs_v") {
                            if let Value::Number(ref n) = bs_v_value {
                                if let Some(f) = n.as_f64() {
                                    if f.fract() == 0.0 {
                                        // Преобразуем значение в целое число, если дробная часть равна 0
                                        let int_val = f as i64;
                                        *bs_v_value = Value::Number(serde_json::Number::from(int_val));
                                    }}}}}}}}
        if let Some(boost_values) = json.get_mut("collect_values") {
            if let Value::Array(ref mut arr) = boost_values {
                for item in arr.iter_mut() {
                    if let Value::Object(ref mut obj) = item {
                        if let Some(bs_v_value) = obj.get_mut("bs_v") {
                            if let Value::Number(ref n) = bs_v_value {
                                if let Some(f) = n.as_f64() {
                                    if f.fract() == 0.0 {
                                        // Преобразуем значение в целое число, если дробная часть равна 0
                                        let int_val = f as i64;
                                        *bs_v_value = Value::Number(serde_json::Number::from(int_val));
                                    }}}}}}}}
        if let Some(boost_values) = json.get_mut("mystery_values") {
            if let Value::Array(ref mut arr) = boost_values {
                for item in arr.iter_mut() {
                    if let Value::Object(ref mut obj) = item {
                        if let Some(bs_v_value) = obj.get_mut("bs_v") {
                            if let Value::Number(ref n) = bs_v_value {
                                if let Some(f) = n.as_f64() {
                                    if f.fract() == 0.0 {
                                        // Преобразуем значение в целое число, если дробная часть равна 0
                                        let int_val = f as i64;
                                        *bs_v_value = Value::Number(serde_json::Number::from(int_val));
                                    }}}}}}}}
                                    
    }

    fn remove_bac_field(value: &mut Value) {
        if let Value::Object(ref mut map) = value {
            if let Some(context) = map.get_mut("context") {
                if let Value::Object(ref mut ctx_map) = context {
                    if let Some(spins) = ctx_map.get_mut("spins") {
                        if let Value::Object(ref mut spins_map) = spins {
                            spins_map.remove("bac");
                        }
                    }
                    if let Some(bonus) = ctx_map.get_mut("bonus") {
                        if let Value::Object(ref mut bonus_map) = bonus {
                            bonus_map.remove("bac");
                        }
                    }
                }
            }
        }
    }

    #[actix_web::test]
    async fn test_tranzactions() {
        use std::fs;
        let data_dir = "../data/tests";
        for entry_result in fs::read_dir(data_dir).unwrap() {
            let entry = entry_result.unwrap();
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                println!("test: {:?}", path.file_name().unwrap());

                let mut l_game = game::models::model::Game::new(); 
                l_game.session_id = "b1395ade829a4bf381ddc6fdc3386efe".to_string();
                l_game.modes = ["auto".to_string(), "play".to_string()].into_iter().map(Into::into).collect();
                l_game.context.round_finished = true;

                let file = File::open(path).unwrap();
                let reader = BufReader::new(file);
                let json: Value = serde_json::from_reader(reader).unwrap();
                let transactions = json.as_array().unwrap();
                let pb = ProgressBar::new((transactions.len()) as u64);
                let mut l_position = 0;
                let mut actual_response: Value = json!(null);
                for transaction in transactions {

                    let mut expected_response = transaction.get("out").unwrap().clone();
                    if let Some(request) = transaction.get("in") {
                        if let Some(request_command) = request.get("command").and_then(Value::as_str) {
                            actual_response = game::actions::execute(Some(request_command), request.to_string(), &mut l_game, Some(expected_response.clone())).await.map_err(|e|  e).expect("REASON");
                        }
                    }
                    sleep(Duration::from_millis(10));

                    replace_bs_v_zero(expected_response.get_mut("context").and_then(|context| context.get_mut("bonus")).unwrap_or(&mut json!({})));
                    replace_bs_v_zero(actual_response.get_mut("context").and_then(|context| context.get_mut("bonus")).unwrap_or(&mut json!({})));
                    
                    remove_bac_field(&mut expected_response);
                    remove_bac_field(&mut actual_response);

                    if let Some(collect_values) = expected_response.get_mut("context").and_then(|context| context.get_mut("spins")).and_then(|bonus| bonus.get_mut("bonus_mechanic")).unwrap_or(&mut json!([])).as_array_mut() {
                        collect_values.sort_by(|a, b| {let a_num = a.as_i64().unwrap_or(0);let b_num = b.as_i64().unwrap_or(0);a_num.cmp(&b_num)});
                    }
                    if let Some(collect_values) = actual_response.get_mut("context").and_then(|context| context.get_mut("spins")).and_then(|bonus| bonus.get_mut("bonus_mechanic")).unwrap_or(&mut json!([])).as_array_mut() {
                        collect_values.sort_by(|a, b| {let a_num = a.as_i64().unwrap_or(0);let b_num = b.as_i64().unwrap_or(0);a_num.cmp(&b_num)});
                    }

                    if let Some(collect_values) = expected_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("bonus_mechanic")).unwrap_or(&mut json!([])).as_array_mut() {
                        collect_values.sort_by(|a, b| {let a_num = a.as_i64().unwrap_or(0);let b_num = b.as_i64().unwrap_or(0);a_num.cmp(&b_num)});
                    }
                    if let Some(collect_values) = actual_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("bonus_mechanic")).unwrap_or(&mut json!([])).as_array_mut() {
                        collect_values.sort_by(|a, b| {let a_num = a.as_i64().unwrap_or(0);let b_num = b.as_i64().unwrap_or(0);a_num.cmp(&b_num)});
                    }

                    if let Some(collect_values) = expected_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("collect_values")).unwrap_or(&mut json!([])).as_array_mut() {
                        collect_values.sort_by(|a, b| {let a_bs = a.get("bs_v").and_then(Value::as_f64).unwrap_or(0.0); let b_bs = b.get("bs_v").and_then(Value::as_f64).unwrap_or(0.0); b_bs.partial_cmp(&a_bs).unwrap_or(std::cmp::Ordering::Equal)} );
                    }
                    if let Some(collect_values) = actual_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("collect_values")).unwrap_or(&mut json!([])).as_array_mut() {
                        collect_values.sort_by(|a, b| {let a_bs = a.get("bs_v").and_then(Value::as_f64).unwrap_or(0.0); let b_bs = b.get("bs_v").and_then(Value::as_f64).unwrap_or(0.0); b_bs.partial_cmp(&a_bs).unwrap_or(std::cmp::Ordering::Equal)} );
                    }

                    if let Some(copy_new_bs) = expected_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("copy_new_bs")).unwrap_or(&mut json!([])).as_array_mut() {
                        copy_new_bs.sort_by(|a, b| {let a0 = a.get(0).and_then(Value::as_i64).unwrap_or(0);let b0 = b.get(0).and_then(Value::as_i64).unwrap_or(0);let a1 = a.get(1).and_then(Value::as_i64).unwrap_or(0);let b1 = b.get(1).and_then(Value::as_i64).unwrap_or(0);a0.cmp(&b0).then(a1.cmp(&b1))} );
                    }
                    if let Some(copy_new_bs) = actual_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("copy_new_bs")).unwrap_or(&mut json!([])).as_array_mut() {
                        copy_new_bs.sort_by(|a, b| {let a0 = a.get(0).and_then(Value::as_i64).unwrap_or(0);let b0 = b.get(0).and_then(Value::as_i64).unwrap_or(0);let a1 = a.get(1).and_then(Value::as_i64).unwrap_or(0);let b1 = b.get(1).and_then(Value::as_i64).unwrap_or(0);a0.cmp(&b0).then(a1.cmp(&b1))} );
                    }
                    if let Some(new_bs) = expected_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("new_bs")).unwrap_or(&mut json!([])).as_array_mut() {
                        new_bs.sort_by(|a, b| {let a0 = a.get(0).and_then(Value::as_i64).unwrap_or(0);let b0 = b.get(0).and_then(Value::as_i64).unwrap_or(0);let a1 = a.get(1).and_then(Value::as_i64).unwrap_or(0);let b1 = b.get(1).and_then(Value::as_i64).unwrap_or(0);a0.cmp(&b0).then(a1.cmp(&b1))} );
                    }
                    if let Some(new_bs) = actual_response.get_mut("context").and_then(|context| context.get_mut("bonus")).and_then(|bonus| bonus.get_mut("new_bs")).unwrap_or(&mut json!([])).as_array_mut() {
                        new_bs.sort_by(|a, b| {let a0 = a.get(0).and_then(Value::as_i64).unwrap_or(0);let b0 = b.get(0).and_then(Value::as_i64).unwrap_or(0);let a1 = a.get(1).and_then(Value::as_i64).unwrap_or(0);let b1 = b.get(1).and_then(Value::as_i64).unwrap_or(0);a0.cmp(&b0).then(a1.cmp(&b1))} );
                    }

                    if transaction.get("in").unwrap_or(&mut json!({})).get("command").and_then(Value::as_str) == Some("play") {assert_eq!(actual_response.get_mut("context"), expected_response.get_mut("context"));}
                    l_position += 1;
                    pb.set_position(l_position);
                }
                pb.finish();
            }
        }
    }
}

//00-no_win.json
//01-win.json
//02-buy_bonus.json
//03-jackpots.json
//10-boost.json
//11-boost_after_double.json
//12-boost_after_collect.json
//14-boost_after_double_after_collect.json
//15-boost_after_collect_after_double.json
//16-boost_jackpot.json
//20-double.json
//21-double_after_boost.json
//22-double_after_collect.json
//24-double_after_collect_after_boost.json
//25-double_after_boost_after_collect.json
//26-double_jackpot.json
//30-collect.json
//31-collect_after_double.json
//32-collect_after_boost.json
//34-collect_after_double_after_boost.json
//35-collect_after_boost_after_double.json
//36-collect_jackpot.json
//40-boost_and_double.json
//41-boost_and_double_after_collect.json
//42-double_and_collect.json
//43-double_and_collect_after_boost.json
//44-collect_and_boost.json
//45-collect_and_boost_after_double.json
//46-boost_and_double_and_collect.json