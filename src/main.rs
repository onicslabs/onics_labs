#![allow(unused_imports)]
#![allow(unused_variables)]
#[macro_use] extern crate nickel;
extern crate chrono;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};
use chrono::*;

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("static"));

    server.utilize(router!(
        get "/" => |req, res| {
            let mut data = HashMap::new();
            data.insert("title", "Lobby");
            return res.render("templates/lobby.tpl", &data);
        }
    ));

    server.utilize(router!(
        get "/blog" => |req, res| {
            let mut data = HashMap::new();
            data.insert("title", "Blog");
            return res.render("templates/blog.tpl", &data);
        }
    ));

    server.utilize(router!(
        get "/contact" => |req, res| {
            let mut data = HashMap::new();
            data.insert("title", "Get in touch");
            return res.render("templates/contact.tpl", &data);
        }
    ));

    server.utilize(router!(
        get "/resources" => |req, res| {
            let mut data = HashMap::new();
            data.insert("title", "Resources");
            return res.render("templates/res.tpl", &data);
        }
    ));

    server.utilize(router!(
        get "/tutorials" => |req, res| {
            let mut data = HashMap::new();
            data.insert("title", "Tutorials");
            return res.render("templates/tut.tpl", &data);
        }
    ));

    server.listen("0.0.0.0:4201");
}
