extern crate iron;
extern crate staticfile;
extern crate mount;

// This example serves the docs from target/doc/staticfile at /doc/
//
// Run `cargo doc && cargo run --example doc_server`, then
// point your browser to http://127.0.0.1:3000/doc/

use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

fn main() {
    let mut mount = Mount::new();

    // Serve the shared JS/CSS at /public/src/
    mount.mount("/", Static::new(Path::new("public/javascript/")));
    mount.mount("/", Static::new(Path::new("public/")));
    // Serve the static file docs at /doc/
    mount.mount("/home/", Static::new(Path::new("target/doc/staticfile/")));

    println!("Web server running on http://localhost:3000/home/");

    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
