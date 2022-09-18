#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

mod api;
mod model;
mod page;
mod view;

use crate::model::*;
use api::requests::*;
use std::rc::Rc;
use view::view::*;

const MUSIC: &str = "Music";
const ADMIN: &str = "Admin";
// ------ ------
//     Init
// ------ ------

fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Model {
    log!("Base URL {:?}", url);
    orders.subscribe(Msg::UrlChanged);
    orders
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url.clone()));
    Model {
        base_url: url.to_base_url(),
        page: Page::init(url, orders, &None),
        ctx: None,
    }
}

// ------ ------
//     Model
// ------ ------

//TODO cleanup
pub struct Model {
    pub base_url: Url,
    pub page: Page,
    pub ctx: Option<LoginMessageResponseBody>,
}

pub enum Page {
    Home(page::home::Model),
    Music(page::music::Model),
    Admin(page::admin::Model),
    NotFound,
}
impl Page {
    fn init(
        mut url: Url,
        orders: &mut impl Orders<Msg>,
        ctx: &Option<LoginMessageResponseBody>,
    ) -> Self {
        //match url.remaining_path_parts().as_slice() {
        match url.next_path_part() {
            Some(ADMIN) => Self::Admin(page::admin::init(url, &mut orders.proxy(Msg::AdminMsg))),
            Some(MUSIC) => Self::Music(page::music::init(url, &mut orders.proxy(Msg::MusicMsg))),
            None => Self::Home(page::home::init(
                url,
                &mut orders.proxy(Msg::HomeMsg),
                ctx.clone(),
            )),
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url()
    }
    fn admin(self) -> Url {
        self.base_url().add_path_part(ADMIN)
    }
    fn music(self) -> page::music::Urls<'a> {
        //self.base_url().add_path_part(MUSIC);
        page::music::Urls::new(self.base_url().add_path_part(MUSIC))
    }
}

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    GoToUrl(Url),

    GetLoginRequest,
    FetchedLogin(fetch::Result<LoginMessageResponseBody>),

    MusicMsg(page::music::Msg),
    AdminMsg(page::admin::Msg),
    HomeMsg(page::home::Msg),
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url, orders, &model.ctx),
        //TODO check if needed
        Msg::GoToUrl(url) => {
            orders.request_url(url);
        }

        Msg::GetLoginRequest => {
            orders.skip().perform_cmd({
                let name = "Carlos".to_string();
                async { Msg::FetchedLogin(get_login(name).await) }
            });
        }
        Msg::FetchedLogin(Ok(mut response_data)) => {
            log!("fetched data: {:?}", &response_data);
            model.ctx = Some(response_data);
        }

        Msg::FetchedLogin(Err(fetch_error)) => {
            log!("Example_A error:", fetch_error);
            orders.skip();
        }

        // ------- Page
        Msg::AdminMsg(msg) => {
            if let Page::Admin(model) = &mut model.page {
                page::admin::update(msg, model, &mut orders.proxy(Msg::AdminMsg))
            }
        }
        Msg::HomeMsg(msg) => {
            if let Page::Home(model) = &mut model.page {
                page::home::update(msg, model, &mut orders.proxy(Msg::HomeMsg))
            }
        }
        Msg::MusicMsg(msg) => {
            if let Page::Music(model) = &mut model.page {
                page::music::update(msg, model, &mut orders.proxy(Msg::MusicMsg))
            }
        }
    };
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    let base = LoginMessageResponseBody::default();
    div![
        &model.ctx.as_ref().unwrap_or(&base).token,
        header(&model.base_url),
        button![ev(Ev::Click, |_| Msg::GetLoginRequest), "Get Login message"],
        match &model.page {
            Page::Home(model) => page::home::view(&model).map_msg(Msg::HomeMsg),
            Page::Admin(model) => page::admin::view(&model).map_msg(Msg::AdminMsg),
            Page::Music(model) => page::music::view(&model).map_msg(Msg::MusicMsg),
            Page::NotFound => page::not_found::view(),
        }
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
