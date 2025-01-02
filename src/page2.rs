use maud::{html, Markup};
use std::error::Error;

// fn header() -> Markup {
//     html! {
//         head {
//             meta charset="utf-8" {}
//             title { "Hello, World!" }
//             meta name="viewport" content="width=device-width, initial-scale=1" {}
//             // TODO: vendor and embed these 3rd-party with rust-embed
//             // link rel="stylesheet" href="https://unpkg.com/mvp.css"
//             // link rel="stylesheet" href="https://github.com/sir-sharkey/css-init/blob/main/dist/init.min.css" {}
//             // TODO: these two together worked the best with the janksite as of writing
//             // pure gives layout primitives and pico gives a nice default theme
//             link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css" {}
//             link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/purecss@3.0.0/build/pure-min.css" integrity="sha384-X38yfunGUhNzHpBaEBsWLO+A0HDYOQi8ufWDkZ0k9e0eXz/tH3II7uKZ9msv++Ls" crossorigin="anonymous" {}
//             // link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet"
//             script src="https://unpkg.com/htmx.org@2.0.4" {}
//             script src="https://unpkg.com/hyperscript.org@0.9.13" {}
//             (styles())
//         }
//     }
// }
//
// fn styles() -> Markup {
//     html! {
//         style {
//             r#"
//             .bordered {
//                 border: 4px solid black;
//             }
//             .red {
//                 background-color: red;
//             }
//             .button_holder {
//                 display: flex;
//                 flex-direction: row;
//             }
//             .big_box {
//                 // display: flex;
//                 width: 500px;
//                 height: 200px;
//                 // gap: 10px;
//             }
//             button.pure-button {
//                 margin: 4px;
//                 border-radius: 8px;
//             }
//             "#
//         }
//     }
// }
//
// fn body() -> Markup {
//     html! {
//         body _=r#"on every htmx:beforeSend in <button:not(.no-disable)/>
//          tell it 
//              toggle [disabled='true'] until htmx:afterOnLoad"#{
//             h1 { "Hello, World!" }
//             h2 { "heading 2" }
//             (button_box())
//         }
//     }
// }
//
// fn some_classes() -> &'static str {
//     "big_box pure-u-1 pure-u-md-1-2"
// }
//
// fn animating_box() -> Markup {
//     html! {
//         svg viewBox="0 0 10 10" xmlns="http://www.w3.org/2000/svg"
//         {
//           rect width="10" height="10"{
//             animate
//               attributeName="rx"
//               values="0;5;0"
//               dur="10s"
//               repeatCount="indefinite" {}
//           }
//         }
//     }
// }
//
// fn anti_japan() -> Markup {
//     html! {
//         svg
//             version="1.1"
//             baseProfile="full"
//             width="300"
//             height="200"
//             xmlns="http://www.w3.org/2000/svg" {
//               rect width="100%" height="100%" fill="black" {}
//               circle cx="150" cy="100" r="90" fill="blue" {}
//             }
//     }
// }
//
// fn button_box() -> Markup {
//     html! {
//         .pure-g {
//             div class=(some_classes()) style="background-color:#02e" {
//                 .button_holder id="unhide-me"{
//                     button #button .pure-button _="on click put 'Load Messages' into #set-me" { "reset" }
//                     // button class="pure-button" id="button" _="on click put 'Load Messages' into #set-me" { "reset" }
//                     // button class="pure-button" id="set-me" hx-get="/messages" { "Load Messages" }
//                     button class="pure-button" id="set-me" _="on click fetch /messages then put it into my innerHTML" { "Load Messages" }
//                 }
//                 .button_holder {
//                     button class="pure-button" _="on click toggle @disabled on #say-hello" { "Toggle Disabled State" }
//                     button class="pure-button" id="say-hello" _="on click alert('hello!')" { "Say Hello" }
//                     button class="pure-button" _="on click toggle @hidden on #unhide-me" { "Hide Me" }
//                 }
//             }
//             div class=(some_classes()) style="background-color:#0e2" {
//                 button class="pure-button" _="on click toggle .bordered on #second-button" { "Toggle Next Border" }
//                 button class="pure-button" id="second-button" _="on click toggle .red unless I match .bordered" { "Toggle My Background" }
//                 (animating_box())
//             }
//             div class=(some_classes()) style="background-color:#e20" _="on click measure me then log it" {
//                 form {
//                     input class="indeterminate" type="checkbox" _="on load set my.indeterminate to true" {}
//                     button .pure-button type="reset" _="on click set .indeterminate.indeterminate to true" { "Set Indeterminate" }
//                 }
//                 button class="pure-button" _="on click transition opacity to 0 then remove me" { "Fade & Remove" }
//                 // button class="pure-button" _="on click append 'Clicked!' to <div/>" { "Click Me" }
//                 button
//                     .pure-button
//                     hx-get="/info"
//                     hx-on--before-request="alert('Making a request!')"
//                     hx-on--after-request="alert('Done making a request!')"
//                     { "Get Info!" }
//                 (anti_japan())
//             }
//         }
//     }
// }
//
// async fn root_page() -> Markup {
//     html! {
//         (DOCTYPE)
//         html color-mode="user" {
//             (header())
//             (body())
//         }
//     }
// }
//
// async fn messages() -> Markup {
//     html! {
//         div {
//             h1 { "Messages" }
//             ul {
//                 li { "Hello, World!" }
//                 li { "Goodbye, World!" }
//             }
//         }
//     }
// }
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     dotenvy::dotenv()?;
//
//     let db_url = dotenvy::var("DATABASE_URL")?;
//     // println!("DATABASE_URL: {}", db_url);
//     //
//     // for (key, value) in env::vars() {
//     //     println!("{key}: {value}");
//     // }
//
//     // Ok(())
//     //
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&db_url)
//         .await?;
//
//     // build our application with a single route
//     let app = Router::new()
//         .route("/", get(root_page))
//         .route("/messages", get(messages));
//
//     // run our app with hyper, listening globally on port 3000
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
//
//     pool.close().await;
//
//     Ok(())
// }
//
// // fn root_router() -> Router {
// //     Router::new().route("/", get(root_page))
// // }
//
// // fn static_files() -> Router {
// //     // you can also have two `ServeDir`s nested at different paths
// //     // TODO: test if nested folders work or if commented out line is needed
// //     // let serve_dir_from_assets = ServeDir::new("assets/public/vendor");
// //     let serve_dir_from_dist = ServeDir::new("assets/public/");
// //
// //     Router::new()
// //         .nest_service("/assets", serve_dir_from_assets)
// //         .nest_service("/dist", serve_dir_from_dist)
// // }
