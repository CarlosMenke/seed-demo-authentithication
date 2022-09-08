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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageResponseBodyGet {
    pub ordinal_number: u32,
    pub text: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageResponseBodyGetVec {
    pub response: Vec<SendMessageResponseBodyGet>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseHtml {
    pub html: String,
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
    pub response_html: Option<ResponseHtml>,
    pub response_data: Option<SendMessageResponseBody>,
    pub response_data_get: Option<SendMessageResponseBodyGet>,
    pub response_data_get_vec: Option<SendMessageResponseBodyGetVec>,
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
    GetRequest,
    GetVecRequest,
    GetHtmlRequest,
    Fetched(fetch::Result<SendMessageResponseBody>),
    FetchedGet(fetch::Result<SendMessageResponseBodyGet>),
    FetchedGetVec(fetch::Result<SendMessageResponseBodyGetVec>),
    FetchedHtml(fetch::Result<ResponseHtml>),
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
        Msg::GetRequest => {
            orders
                .skip()
                .perform_cmd(async { Msg::FetchedGet(get_message().await) });
        }
        Msg::GetVecRequest => {
            orders
                .skip()
                .perform_cmd(async { Msg::FetchedGetVec(get_vec_message().await) });
        }
        Msg::GetHtmlRequest => {
            orders
                .skip()
                .perform_cmd(async { Msg::FetchedHtml(get_html().await) });
        }

        Msg::Fetched(Ok(response_data)) => {
            log!("fetched data: {:?}", &response_data);
            model.response_data = Some(response_data);
        }
        Msg::Fetched(Err(fetch_error)) => {
            log!("Example_A error:", fetch_error);
            orders.skip();
        }

        Msg::FetchedGet(Ok(response_data)) => {
            log!("fetched data: {:?}", &response_data);
            model.response_data_get = Some(response_data);
        }
        Msg::FetchedGet(Err(fetch_error)) => {
            log!("Example_A error:", fetch_error);
            orders.skip();
        }

        Msg::FetchedGetVec(Ok(response_data)) => {
            log!("fetched data: {:?}", &response_data);
            model.response_data_get_vec = Some(response_data);
        }
        Msg::FetchedGetVec(Err(fetch_error)) => {
            log!("Example_A error:", fetch_error);
            orders.skip();
        }

        Msg::FetchedHtml(Ok(response_data)) => {
            log!("fetched data: {:?}", &response_data);
            model.response_html = Some(response_data);
        }
        Msg::FetchedHtml(Err(fetch_error)) => {
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

async fn get_message() -> fetch::Result<SendMessageResponseBodyGet> {
    fetch("http://127.0.0.1:8084/test_get.json")
        .await?
        .check_status()?
        .json()
        .await
}

async fn get_vec_message() -> fetch::Result<SendMessageResponseBodyGetVec> {
    fetch("http://127.0.0.1:8084/test_get_vec.json")
        .await?
        .check_status()?
        .json()
        .await
}

async fn get_html() -> fetch::Result<ResponseHtml> {
    fetch("http://127.0.0.1:8084/test_html.html")
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
        view_message_get(&model.response_data_get),
        view_message_get_vec(&model.response_data_get_vec),
        view_message_html(&model.response_html),
        input![
            input_ev(Ev::Input, Msg::NewMessageChanged),
            attrs! {
                At::Value => model.new_message,
                At::AutoFocus => AtValue::None,
            }
        ],
        button![ev(Ev::Click, |_| Msg::SendRequest), "Send message"],
        button![ev(Ev::Click, |_| Msg::GetRequest), "Get message"],
        button![ev(Ev::Click, |_| Msg::GetVecRequest), "Get Vec message"],
        button![ev(Ev::Click, |_| Msg::GetHtmlRequest), "Get Html message"],
        raw![include_str!("../static/music_all.html")]
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

fn view_message_get(message: &Option<SendMessageResponseBodyGet>) -> Node<Msg> {
    let message = match message {
        Some(message) => message,
        None => return empty![],
    };
    div![div![format!(
        r#""{}". message: "{}""#,
        message.ordinal_number, message.text
    )],]
}

fn view_message_get_vec(message: &Option<SendMessageResponseBodyGetVec>) -> Node<Msg> {
    let message = match message {
        Some(message) => message,
        None => return empty![],
    };

    div![
        C!["response-list"],
        message.response.iter().map(|value| {
            ul![format!(
                r#""{}". message: "{}""#,
                value.ordinal_number, value.text
            )]
        })
    ]
}

fn view_message_html(message: &Option<ResponseHtml>) -> Node<Msg> {
    let message = match message {
        Some(message) => message,
        None => return empty![],
    };
    div![raw![&message.html],]
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
