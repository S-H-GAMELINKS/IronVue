extern crate iron;
extern crate staticfile;
extern crate mount;

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use std::path::Path;

fn main() {
    let mut mount = Mount::new();

    let routes = ["/", "/about", "/contact"];

    for route in &routes {
        mount.mount(route, Static::new(Path::new("static/index.html")));
    }

    mount.mount("/index.js", Static::new(Path::new("static/index.js")));

    Iron::new(mount).http(env!("PORT")).unwrap();
}