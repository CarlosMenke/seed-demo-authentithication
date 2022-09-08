// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageRequestBody {
    pub text: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageResponseBody {
    pub ordinal_number: u32,
    pub text: String,
}
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
#[derive(Default)]
struct Model {
    counter: i32,
    pub new_message: String,
    pub response_data: Option<SendMessageResponseBody>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    password: String,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive()]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
    Decrement,
    NewMessageChanged(String),
    SendRequest,
    Fetched(fetch::Result<SendMessageResponseBody>),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,

        Msg::NewMessageChanged(message) => {
            model.new_message = message;
        }
        Msg::SendRequest => {
            orders.skip().perform_cmd({
                let message = model.new_message.clone();
                async { Msg::Fetched(send_message(message).await) }
            });
        }

        Msg::Fetched(Ok(response_data)) => {
            log!("fetched data: {:?}", &response_data);
            model.response_data = Some(response_data);
        }

        Msg::Fetched(Err(fetch_error)) => {
            log!("Example_A error:", fetch_error);
            orders.skip();
        }
    };
}

async fn send_message(new_message: String) -> fetch::Result<SendMessageResponseBody> {
    fetch(
        Request::new("http://127.0.0.1:8084/test_post.json")
            .method(Method::Post)
            //.mode(web_sys::RequestMode::NoCors)
            .json(&SendMessageRequestBody { text: new_message })?,
    )
    .await?
    .check_status()?
    .json()
    .await
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    let button_style =
        style!(St::BackgroundColor => "green", St::Margin => "10px", St::BorderRadius=> "5px");
    div![
        "This is a counter: ",
        model.counter,
        C!["counter"],
        button![
            ev(Ev::Click, |_| Msg::Increment),
            "Increment",
            &button_style
        ],
        button![
            &button_style,
            IF!( model.counter > 0  => ev(Ev::Click, |_| Msg::Decrement)),
            IF!( model.counter == 0 => style!(St::BackgroundColor => "red")),
            "Decrement",
        ],
        view_message(&model.response_data),
        input![
            input_ev(Ev::Input, Msg::NewMessageChanged),
            attrs! {
                At::Value => model.new_message,
                At::AutoFocus => AtValue::None,
            }
        ],
        button![ev(Ev::Click, |_| Msg::SendRequest), "Send message"],
    ]
}

fn view_message(message: &Option<SendMessageResponseBody>) -> Node<Msg> {
    let message = match message {
        Some(message) => message,
        None => return empty![],
    };
    div![div![format!(
        r#""{}". message: "{}""#,
        message.ordinal_number, message.text
    )],]
}
// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
