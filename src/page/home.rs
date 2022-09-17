use crate::api::requests::*;
use crate::{model::*, view::view::*};
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    model: &mut impl Orders<Msg>,
    ctx: Option<LoginMessageResponseBody>,
) -> Model {
    Model {
        base_url: url.to_base_url(),
        ctx: ctx,
        counter: 1,
        new_message: String::new(),
        login_name: String::new(),
        response_html: Some(ResponseHtml::default()),
        response_data: Some(SendMessageResponseBodyGet::default()),
        response_data_get: Some(SendMessageResponseBodyGet::default()),
        response_data_get_vec: Some(SendMessageResponseBodyGetVec::default()),
        response_admin: Some(SendMessageResponseBodyGet::default()),
    }
}

pub enum Msg {
    Increment,
    Decrement,
    NewMessageChanged(String),
    SendRequest,
    GetRequest,
    GetVecRequest,
    GetHtmlRequest,
    GetAdminRequest,

    Fetched(fetch::Result<SendMessageResponseBodyGet>),
    FetchedGet(fetch::Result<SendMessageResponseBodyGet>),
    FetchedGetVec(fetch::Result<SendMessageResponseBodyGetVec>),
    FetchedHtml(fetch::Result<ResponseHtml>),
    FetchedGetAdmin(fetch::Result<SendMessageResponseBodyGet>),
}
// ------ ------
//     Model
// ------ ------

pub struct Model {
    pub base_url: Url,
    pub ctx: Option<LoginMessageResponseBody>,

    pub counter: i32,
    pub new_message: String,
    pub login_name: String,
    pub response_html: Option<ResponseHtml>,
    pub response_data: Option<SendMessageResponseBodyGet>,
    pub response_data_get: Option<SendMessageResponseBodyGet>,
    pub response_data_get_vec: Option<SendMessageResponseBodyGetVec>,
    pub response_admin: Option<SendMessageResponseBodyGet>,
}

// ------ Frequency ------

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn root(self) -> Url {
        self.base_url()
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
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
        Msg::GetAdminRequest => {
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                log!("sended token {}", token);
                async { Msg::FetchedGetAdmin(get_admin(token).await) }
            });
        }

        Msg::Fetched(Ok(response_data)) => {
            log!("fetched data: {:?}", &response_data);
            model.response_data = Some(response_data);
        }

        Msg::FetchedGet(Ok(response_data)) => {
            log!("fetched data: {:?}", &response_data);
            model.response_data_get = Some(response_data);
        }

        Msg::FetchedGetVec(Ok(response_data)) => {
            log!("fetched data: {:?}", &response_data);
            model.response_data_get_vec = Some(response_data);
        }

        Msg::FetchedHtml(Ok(response_data)) => {
            log!("fetched data: {:?}", &response_data);
            model.response_html = Some(response_data);
        }

        Msg::FetchedGetAdmin(Ok(response_data)) => {
            log!("fetched data: {:?}", &response_data);
            model.response_admin = Some(response_data);
        }

        Msg::Fetched(Err(fetch_error))
        | Msg::FetchedGet(Err(fetch_error))
        | Msg::FetchedGetVec(Err(fetch_error))
        | Msg::FetchedHtml(Err(fetch_error))
        | Msg::FetchedGetAdmin(Err(fetch_error)) => {
            log!("Example_A error:", fetch_error);
            orders.skip();
        }
    };
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    let button_style = style!(
    St::BackgroundColor => "green", St::Margin => px(10), St::Padding => px(5), St::PaddingLeft => px(10), St::PaddingRight => px(10),  St::BorderRadius => px(15), St::Border => px(0)
    );
    let message = match model.ctx.clone() {
        Some(message) => message.token,
        None => "default".to_string(),
    };
    div![
        message,
        div![
            model.counter,
            C!["counter"],
            button![
                C!["button"],
                &button_style,
                IF!( model.counter < 3  => ev(Ev::Click, |_| Msg::Increment)),
                IF!( model.counter >= 3 => style!(St::BackgroundColor => "red")),
                "Increment",
            ],
            button![
                C!["button"],
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
        button![ev(Ev::Click, |_| Msg::GetAdminRequest), "Get Admin data"],
    ]
}
