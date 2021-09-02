// #[macro_use] extern crate diesel;
// extern crate dotenv;

// // use crate::diesel;
// use diesel::prelude::*;

// mod database;
// mod schema;
// mod models;

// use database::establish_connection;
// use models::FibEntry;
// use schema::fib_entries;


// fn get_fib_enteries() -> i32 {

//     let connection = establish_connection();  
 
//     let fib_enteries = fib_entries::table
//         .order(fib_entries::columns::input_number.asc())
//         .load::<FibEntry>(&connection)
//         .unwrap();
    
//     // let mut buffer = Vec::new();
//     let mut counter = 0;

//     for i in fib_enteries {
//         println!("{}", i.calculated_number.unwrap());
//     }
//     return counter
//  }


// fn main() {
//     let _ = get_fib_enteries();
//     println!("this is running");
// }