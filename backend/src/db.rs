// backend/src/db.rs
use mysql::*;
use mysql::prelude::*;

// Initialize and return a connection pool
pub fn init_pool() -> Pool {
    let url = "mysql://root:Latterdaysaints1401!@localhost:3306/greeting_app";
    let opts = Opts::from_url(url).expect("Invalid DB URL");
    Pool::new(opts).expect("Failed to create pool")
}
