use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use mysql::*;
use mysql::prelude::*;
use actix_files as fs;

#[derive(Serialize, Deserialize)]
struct TokenTransaction {
    address: String,
    amount: u64,
}

#[derive(Serialize)]
struct Balance {
    balance: u64,
}

fn establish_connection() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let opts = Opts::from_url(&database_url).expect("Invalid DATABASE_URL");
    Pool::new(opts).expect("Failed to create pool.")
}

async fn send_tokens(transaction: web::Json<TokenTransaction>) -> impl Responder {
    let pool = establish_connection();
    let mut conn = pool.get_conn().expect("Failed to get connection from pool");

    let result = conn.exec_drop(
        "INSERT INTO transactions (address, amount, transaction_type) VALUES (:address, :amount, 'send')",
        params! {
            "address" => &transaction.address,
            "amount" => transaction.amount,
        },
    );
    
    match result {
        Ok(_) => {
            conn.exec_drop(
                "UPDATE balance SET balance = balance - :amount",
                params! {
                    "amount" => transaction.amount,
                },
            ).expect("Failed to update balance");
            HttpResponse::Ok().json("Tokens sent")
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to send tokens"),
    }
}