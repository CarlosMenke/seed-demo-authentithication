use seed::{prelude::*, *};

use crate::{
    model::{LoginMessageResponseBody, SendMessageResponseBodyGet},
    view::view::*,
};

// ------ ------
//     Init
// ------ ------

pub fn init(mut url: Url, model: &mut impl Orders<Msg>) -> Model {
    Model {
        base_url: url.to_base_url(),
        counter: 0,
    }
}

pub enum Msg {
    Increment,
}
// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    counter: i32,
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
    };
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div![
        //view_token(response_login),
        //TODO fetch data here
        button![
            C!["button"],
            IF!( model.counter < 3  => ev(Ev::Click, |_| Msg::Increment)),
            IF!( model.counter >= 3 => style!(St::BackgroundColor => "red")),
            "Increment",
        ],
        model.counter,
    ]
}
