use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use math_api;

#[get("/factorial/{n}")]
async fn factorial(req: HttpRequest) -> impl Responder {
    let number = req.match_info().get("n");
    //TODO validate if the number is a natural number.
    match number {
        Some(n) => {
            let value = math_api::factorial(n.parse::<u8>().unwrap());
            HttpResponse::Ok().body(format!("{}", value))
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

#[get("/fibonacci/{n}")]
async fn fibonacci(req: HttpRequest) -> impl Responder {
    let number = req.match_info().get("n");
    //TODO validate if the number is a natural number.
    match number {
        Some(n) => {
            let value = math_api::fibonacci(n.parse::<u8>().unwrap());
            HttpResponse::Ok().body(format!("{}", value))
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

#[get("/health")]
async fn index() -> impl Responder {
    "OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running Math API server.");
    
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(fibonacci)
            .service(factorial)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(format!("0.0.0.0:8000"))?
    .run()
    .await
}
