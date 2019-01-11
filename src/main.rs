extern crate iron;
extern crate staticfile;
extern crate mount;

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use std::path::Path;
use std::env;

fn get_server_port() -> u16 {
    env::var("PORT").ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000)
}

fn main() {
    let mut mount = Mount::new();

    let routes = ["/", "/about", "/contact"];

    for route in &routes {
        mount.mount(route, Static::new(Path::new("static/index.html")));
    }

    mount.mount("/index.js", Static::new(Path::new("static/index.js")));

    Iron::new(mount).http(("0.0.0.0", get_server_port())).unwrap();
}