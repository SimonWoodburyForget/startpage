use warp::Filter;

mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Group(String);

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Name(String);

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Url(String);

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Config {
        pub groups: Vec<(Group, Name, Url)>,
        pub links: Vec<(Name, Url)>,
    }
}

mod userscript {
    use std::fmt::Display;
    const USER_SCRIPT: &str = include_str!("./userscript.js");
    pub fn main<A: Display, B: Display>(key: A, url: B) -> String {
        let key_url = format!("const KEY = \"{}\"; const URL = {};", key, url);
        USER_SCRIPT.replace(";;KEY_URL;;", &key_url)
    }
}

#[tokio::main]
async fn main() {
    use dirs_next::config_dir;
    let _conf = config_dir().map(|mut path| {
        path.push("startpage");
        path
    });

    let routes = {
        warp::path("startpage.user.js")
            .map(|| userscript::main(" ", "http://localhost:8005"))
            .or(warp::any().map(|| "404"))
    };

    warp::serve(routes).run(([127, 0, 0, 1], 8005)).await;
}
