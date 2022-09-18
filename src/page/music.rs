use seed::{prelude::*, *};

use crate::view::view::*;

const DEPTH1: &str = "1";
const DEPTH2: &str = "2";
const DEPTH3: &str = "3";

// ------ ------
//     Init
// ------ ------

pub fn init(mut url: Url, model: &mut impl Orders<Msg>) -> Model {
    let base_url = url.to_base_url();
    let depth_init = match url.remaining_path_parts().as_slice() {
        [DEPTH1] => Depth::Depth1,
        [DEPTH2] => Depth::Depth2,
        [DEPTH3] => Depth::Depth3,
        [] => {
            Urls::new(&base_url).default().go_and_replace();
            Depth::Depth1
        }
        _ => Depth::Depth1,
    };
    Model {
        base_url: base_url,
        depth: depth_init,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    depth: Depth,
}

// ------ Frequency ------

enum Depth {
    Depth1,
    Depth2,
    Depth3,
}

pub enum Msg {}
// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn root(self) -> Url {
        self.base_url()
    }
    pub fn default(self) -> Url {
        self.depth1()
    }
    pub fn depth1(self) -> Url {
        self.base_url().add_path_part(DEPTH1)
    }
    pub fn depth2(self) -> Url {
        self.base_url().add_path_part(DEPTH2)
    }
    pub fn depth3(self) -> Url {
        self.base_url().add_path_part(DEPTH3)
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {}
// ------ ------
//     View
// ------ ------

pub fn view<Ms>(model: &Model) -> Node<Ms> {
    let (depth, link, view_music) = match &model.depth {
        Depth::Depth1 => (
            DEPTH1,
            a![
                "Switch to depth 2",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depth2()
                }
            ],
            view_music_depth_1(),
        ),
        Depth::Depth2 => (
            DEPTH2,
            a![
                "Switch to depth 3",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depth3()
                }
            ],
            view_music_depth_2(),
        ),
        Depth::Depth3 => (
            DEPTH3,
            a![
                "Switch to depth 1",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depth1()
                }
            ],
            view_music_depth_3(),
        ),
    };

    div![
        "This is the depth: ",
        depth,
        div![format!("Hello! This is your {} report.", depth,), link,],
        view_music,
    ]
}
