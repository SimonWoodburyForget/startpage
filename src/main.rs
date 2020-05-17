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
    use std::net::SocketAddr;
    pub fn make(key: char, addr: impl Into<SocketAddr>) -> String {
        include_str!("./userscript.js").replace(
            ";;KEY_URL;;",
            &format!(
                "const KEY = \"{}\"; const URL = \"http://{}\";",
                key,
                addr.into()
            ),
        )
    }
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8005);
    use dirs_next::config_dir;
    let _conf = config_dir().map(|mut path| {
        path.push("startpage");
        path
    });
    let routes = warp::path("startpage.user.js")
        .map(move || userscript::make(' ', addr))
        .or(warp::any().map(|| "404"));
    warp::serve(routes).run(addr).await;
}
