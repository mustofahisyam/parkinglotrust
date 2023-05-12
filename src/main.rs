use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::handlers::{entry_point, create_block_handler, initiate_database_handler, get_availabilities_by_vehicle_type, 
    do_parking_handler, checkout_handler};

pub mod handlers;

#[get("/")]
async fn hello() -> impl Responder {
    let resp: String =  entry_point();
    HttpResponse::Ok().body(resp)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[get("/block/{vehicle_type}/availabilities")]
async fn get_availabilities_vehicle_type(v_type: web::Path<String>) -> impl Responder {
    let resp = get_availabilities_by_vehicle_type(&v_type);

    let resp = match resp {
        Ok(resp) => resp,
        Err(_) => panic!()
    };

    HttpResponse::Ok().body(resp)
}


#[get("/initiate_db")]
async fn initiate_db() -> impl Responder {
    let resp: String = initiate_database_handler();
    HttpResponse::Ok().body(resp)
}

#[post("/block")]
async fn block(req_body: String) -> impl Responder {
    let block = create_block_handler(&req_body);

    let resp = match block {
        Ok(resp) => resp,
        Err(_) => panic!()
    };
    HttpResponse::Ok().body(resp)
}

#[post("/enter")]
async fn enter(req_body: String) -> impl Responder {
    let enter = do_parking_handler(&req_body);

    let resp = match enter {
        Ok(resp) => resp,
        Err(_) => panic!()
    };
    HttpResponse::Ok().body(resp)
}

#[get("/checkout/{vehicle_id}")]
async fn checkout(v_id: web::Path<String>) -> impl Responder {
    let co = checkout_handler(&v_id);

    let resp = match co {
        Ok(resp) => resp,
        Err(_) => panic!()
    };
    HttpResponse::Ok().body(resp)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(initiate_db)
            .service(block)
            .service(get_availabilities_vehicle_type)
            .service(enter)
            .service(checkout)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
