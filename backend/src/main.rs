extern crate iron;
extern crate staticfile;
extern crate mount;

// Simple static file server written in rust and eventually
// Elm too!
//
// Run `cargo run` then
// Navigate browser to http://127.0.0.1:3000/home/

use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

fn main() {
    let mut mount = Mount::new();

    // Serve the shared JS/CSS at /public/javascript/
    mount.mount("/", Static::new(Path::new("frontend/public/javascript/main.js")));

    // Serve the static file docs at /home/
    mount.mount("/home/", Static::new(Path::new("frontend/public/index.html")));

    // Log
    println!("Web server running on http://localhost:3000/home/");

    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
