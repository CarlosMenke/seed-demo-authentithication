// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

mod api;
mod model;
mod page;
mod view;

use crate::model::*;
use api::requests::*;
use view::view::*;

// ------ ------
//     Init
// ------ ------

fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Model {
    log!("Base URL {:?}", url);
    orders.subscribe(Msg::UrlChanged);
    orders
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url.clone()));
    Model::new(url, orders.clone_base_path())
}

pub enum Msg {
    Increment,
    Decrement,

    UrlChanged(subs::UrlChanged),
    GoToUrl(Url),

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

// ------ ------
//    Update
// ------ ------

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,

        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            model.page_id = match url.next_path_part() {
                None => Some(PageId::Home),
                //TODO get music from impl for URLs
                Some("Music") => {
                    page::music::init(url, &mut model.music_model).map(|_| PageId::Music)
                }
                Some(_) => None,
            };
        }
        //TODO check if needed
        Msg::GoToUrl(url) => {
            orders.request_url(url);
        }

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

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    let button_style =
        style!(St::BackgroundColor => "green", St::Margin => "10px", St::BorderRadius=> "5px");
    div![
        header(&model.base_url),
        match model.page_id {
            Some(PageId::Home) => div!["Welcome home!"],
            Some(PageId::Music) => {
                page::music::view(
                    model.music_model.as_ref().expect("admin model"),
                    &model.counter,
                )
            }
            None => div!["404 Page not found"],
        },
        div![
            "This is a counter: ",
            model.counter,
            C!["counter"],
            button![
                &button_style,
                IF!( model.counter < 3  => ev(Ev::Click, |_| Msg::Increment)),
                IF!( model.counter >= 3 => style!(St::BackgroundColor => "red")),
                "Increment",
            ],
            button![
                &button_style,
                IF!( model.counter > 1  => ev(Ev::Click, |_| Msg::Decrement)),
                IF!( model.counter == 1 => style!(St::BackgroundColor => "red")),
                "Decrement",
            ],
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
        //view_url(&model),
    ]
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
