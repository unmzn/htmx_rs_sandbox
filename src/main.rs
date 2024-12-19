use axum::{routing::get, routing::put, Router};
use maud::{html, Markup, DOCTYPE};

fn header() -> Markup {
    html! {
        head {
            meta charset="utf-8";
            title { "Hello, World!" }
            meta name="viewport" content="width=device-width, initial-scale=1";
            script src="https://unpkg.com/htmx.org@2.0.4" {}
            script src="https://unpkg.com/hyperscript.org@0.9.13" {}
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

fn button_box() -> Markup {
    html! {
        div style="background-color:#02e;width:500px;height:500px;" {
            button id="button" _="on click transition opacity to 0 then remove closest parent <div/>" { "Click me!" }
            button hx-put="/messages" { "Load Messages" }
        }
        div style="background-color:#e20;width:500px;height:500px;" {
            button _="on click toggle .bordered on #second-button" { "Toggle Next Border" }
            button id="second-button" _="on click toggle .red unless I match .bordered" { "Toggle My Background" }
        }
    }
}

async fn root_page() -> Markup {
    html! {
        (DOCTYPE)
        html {
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
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(root_page))
        .route("/messages", put(messages));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
