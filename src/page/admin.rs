use seed::{prelude::*, *};

use crate::{
    model::{LoginMessageResponseBody, SendMessageResponseBodyGet},
    view::view::*,
};

// ------ ------
//     Init
// ------ ------

pub fn init(mut url: Url, model: &mut Option<Model>) -> Option<()> {
    let model = model.get_or_insert_with(|| Model {
        base_url: url.to_base_url(),
    });
    Some(())
}

pub enum Msg {}
// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
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

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {};
}

// ------ ------
//     View
// ------ ------

pub fn view<Ms>(model: &Model, response_login: &Option<LoginMessageResponseBody>) -> Node<Ms> {
    div![
        view_token(response_login),
        //TODO fetch data here
    ]
}
