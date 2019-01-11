extern crate iron;
extern crate staticfile;
extern crate mount;

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use std::path::Path;

fn main() {
    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("static/index.html")));
    mount.mount("/index.js", Static::new(Path::new("static/index.js")));

    Iron::new(mount).http("localhost:3000").unwrap();
}