#[macro_use] extern crate nickel;
extern crate chrono;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};
use chrono::*;

fn main() {
    let mut server = Nickel::new();

    server.get("/*", middleware! { |_, res|
        let utc: DateTime<UTC> = UTC::now();
        let mut data = HashMap::new();
        let footsig = utc.format("<em>%b</em> || Onics Labs, LLC || <em>%Y</em>").to_string(); 
        data.insert("foot-sig", footsig); 
        return res.render("assets/templates/landing.tpl", &data);
    });

    server.listen("0.0.0.0:4201");
}
