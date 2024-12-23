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