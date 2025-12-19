use actix_web::{middleware::{Logger, Compress}, web, App, HttpServer, HttpResponse};
use server::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> 
{
    // locked list
    let shared_game_list: games_list::LockedList<games_list::Games> = games_list::LockedList::<games_list::Games>::new();
    //server
    HttpServer::new(move || {
            App::new()
            .app_data(web::Data::new(shared_game_list.clone()))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .route("/", web::get().to(|| async {HttpResponse::Ok().body("Works!")}))
            .route("b/server", web::post().to(execute_external))
        }
    )
    .bind("0.0.0.0:8080").unwrap_or_else(|_| {log::error!("Can not add port 8080."); std::process::exit(1);}) 
    .run()
    .await.unwrap_or_else(|_| {log::error!("Can not start server."); std::process::exit(1);});
    Ok(())
}

/*

.map(Into::into),

.map(|vec| vec.into_iter().map(Into::into).collect()),

.map(|vec| vec.into_iter().map(|inner_vec| {{inner_vec.into_iter().map(Into::into).collect()}}).collect()),

.into(),

.into_iter().map(Into::into).collect(),

.into_iter().map(|inner_vec| {{inner_vec.into_iter().map(Into::into).collect()}}).collect(),

*/

