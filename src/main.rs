#![allow(unused_imports)]
#![allow(unused_variables)]
#[macro_use] extern crate nickel;
extern crate chrono;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};
use chrono::*;

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("src/static"));

    server.utilize(router!(
        get "/" => |req, res| {
            let utc: DateTime<UTC> = UTC::now();
            let mut data = HashMap::new();
            let ftsig = "<em>%b</em> || Onics Labs, LLC || <em>%Y</em>"; 
            data.insert("foot-sig", utc.format(ftsig).to_string()); 
            return res.render("templates/landing.tpl", &data);
        }
    ));
    
    server.utilize(router!(
        get "/boot" => |req, res| {
            let utc: DateTime<UTC> = UTC::now();
            let mut data = HashMap::new();
            let ftsig = "<em>%b</em> || Onics Labs, LLC || <em>%Y</em>"; 
            data.insert("foot-sig", utc.format(ftsig).to_string()); 
            return res.render("templates/boot.tpl", &data);
        }
    ));

    server.listen("0.0.0.0:4201");
}
