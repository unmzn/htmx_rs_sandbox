use axum::{routing::get, routing::put, Router};
use maud::{html, Markup, DOCTYPE};
use sqlx::postgres::PgPoolOptions;
// use std::env;
use std::error::Error;

fn header() -> Markup {
    html! {
        head {
            meta charset="utf-8";
            title { "Hello, World!" }
            meta name="viewport" content="width=device-width, initial-scale=1";
            // TODO: vendor and embed these 3rd-party with rust-embed
            // link rel="stylesheet" href="https://unpkg.com/mvp.css"
            // link rel="stylesheet" href="https://github.com/sir-sharkey/css-init/blob/main/dist/init.min.css";
            // TODO: these two together worked the best with the janksite as of writing
            // pure gives layout primitives and pico gives a nice default theme
            link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css";
            link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/purecss@3.0.0/build/pure-min.css" integrity="sha384-X38yfunGUhNzHpBaEBsWLO+A0HDYOQi8ufWDkZ0k9e0eXz/tH3II7uKZ9msv++Ls" crossorigin="anonymous";
            // link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet"
            script src="https://unpkg.com/htmx.org@2.0.4" {}
            script src="https://unpkg.com/hyperscript.org@0.9.13" {}
            (styles())
        }
    }
}

fn styles () -> Markup {
    html! {
        style {
            r#"
            .bordered {
                border: 4px solid black;
            }
            .red {
                background-color: red;
            }
            .button_holder {
                display: flex;
                flex-direction: row;
            }
            .big_box {
                width: 500px;
                height: 200px;
            }
            "#
        }
    }
}

fn body() -> Markup {
    html! {
        body {
            h1 { "Hello, World!" }
            h2 { "heading 2" }
            (button_box())
        }
    }
}

fn some_classes() -> &'static str {
    "big_box pure-u-1 pure-u-md-1-2"
}

fn button_box() -> Markup {
    html! {
        .pure-g {
            div class=(some_classes()) style="background-color:#02e" {
                .button_holder {
                    // button id="button" _="on click transition opacity to 0 then remove closest parent <div/>" { "Click me!" }
                    button id="button" _="on click transition opacity to 0 then toggle @hidden on closest parent <div/>" { "Click me!" }
                    button hx-put="/messages" { "Load Messages" }
                }
                .button_holder {
                    button _="on click toggle @disabled on #say-hello" { "Toggle Disabled State" }
                    button button id="say-hello" _="on click alert('hello!')" { "Say Hello" }
                }
            }
            div class=(some_classes()) style="background-color:#0e2" {
                button _="on click toggle .bordered on #second-button" { "Toggle Next Border" }
                button id="second-button" _="on click toggle .red unless I match .bordered" { "Toggle My Background" }
            }
        }
    }
}

async fn root_page() -> Markup {
    html! {
        (DOCTYPE)
        html color-mode="user" {
            (header())
            (body())
        }
    }
}

async fn messages() -> Markup {
    html! {
        div {
            h1 { "Messages" }
            ul {
                li { "Hello, World!" }
                li { "Goodbye, World!" }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    dotenvy::dotenv()?;

    let db_url = dotenvy::var("DATABASE_URL")?;
    // println!("DATABASE_URL: {}", db_url);
    //
    // for (key, value) in env::vars() {
    //     println!("{key}: {value}");
    // }

    // Ok(())
    //
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await?;

    // build our application with a single route
    let app = Router::new().route("/", get(root_page))
        .route("/messages", put(messages));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
