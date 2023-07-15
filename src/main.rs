use actix_web::{get, App, HttpRequest,HttpResponse, HttpServer, Responder, middleware::{Logger, self}, web};
mod read_config;
mod authentication_token;
use serde::Deserialize;
use serde_json::Value;
use chrono::{Duration, Utc};
// mod authentication_token;
use authentication_token::{AuthenticationToken, Claims};
use jsonwebtoken::{encode, EncodingKey, Header};
mod mongodb_helper;


async fn ping() -> impl Responder {
    "Ping request received"
}

#[derive(Deserialize)]
struct Customer {
    customerId: String,
    apiKey: String,
}
async fn generate_token(cust: web::Query<Customer>, req: HttpRequest) -> HttpResponse {
    let conf = read_config::read_config();
    let conf_json = conf["token"].clone();
    if cust.customerId == conf_json["username"] && cust.apiKey == conf_json["password"] {
        let claims = Claims {
            customerid: cust.customerId.to_string().parse::<usize>().unwrap(),
            exp: (Utc::now() + Duration::days(conf_json["expiry_days"].as_i64().unwrap())).timestamp() as usize,
        };
        // let header = Header::new(Algorithm::HS512);
        let token: String = encode(
            &Header::default(),
            // &header,
            &claims,
            // &EncodingKey::from_secret("seaker-secret".as_ref()),
            &EncodingKey::from_secret(conf_json["secret"].as_str().unwrap().as_ref()),
        )
        .unwrap();
        HttpResponse::Ok().body(token)
    } else {
        println!("{:?},{:?},{:?},{:?}",cust.customerId, conf_json["username"],cust.apiKey, conf_json["password"]);
        HttpResponse::Ok().body("Invalid Credentials".to_string())
    }

}

#[derive(Deserialize)]
struct MongodbData {
    database: String,
    collection: String,
    filter: String,
}
async fn mongodb_data(    tele: web::Query<MongodbData>,
    _auth_token: AuthenticationToken,
    req: HttpRequest,
) -> impl Responder {
    let data = mongodb_helper::get_mongodb_data(
        tele.database.to_string(),
        tele.collection.to_string(),
        tele.filter.to_string(),
    )
    .await;
    match data {
        Ok(data) => {
            HttpResponse::Ok()
                .json(data)
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Compress::default())
        .wrap(Logger::new(" IP=%{r}a Request=%r Response=%s Response_Size(bytes)=%b Reefer=%{Referer}i %{User-Agent}i Response_Time=%T"))
        .route("/test/ping", web::get().to(ping))
        .route("/test/token", web::post().to(generate_token))
        .route("/test/mongodb", web::get().to(mongodb_data))
    })
    .bind(("localhost",5006))?
    .run()
    .await
}