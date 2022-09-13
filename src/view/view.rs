use crate::model::*;
use crate::Msg;
use seed::{prelude::*, *};

pub fn view_music_depth_1<Ms>() -> Vec<Node<Ms>> {
    raw![include_str!("../../static/music_depth_1.html")]
}
pub fn view_music_depth_2<Ms>() -> Vec<Node<Ms>> {
    raw![include_str!("../../static/music_depth_2.html")]
}
pub fn view_music_depth_3<Ms>() -> Vec<Node<Ms>> {
    raw![include_str!("../../static/music_depth_3.html")]
}

pub fn view_message(message: &Option<SendMessageResponseBodyGet>) -> Node<Msg> {
    let message = match message {
        Some(message) => message,
        None => return empty![],
    };
    div![div![format!(
        r#""{}". message: "{}""#,
        message.ordinal_number, message.text
    )],]
}

pub fn view_message_get(message: &Option<SendMessageResponseBodyGet>) -> Node<Msg> {
    let message = match message {
        Some(message) => message,
        None => return empty![],
    };
    div![div![format!(
        r#""{}". message: "{}""#,
        message.ordinal_number, message.text
    )],]
}

pub fn view_message_get_vec(message: &Option<SendMessageResponseBodyGetVec>) -> Node<Msg> {
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

pub fn view_message_html(message: &Option<ResponseHtml>) -> Node<Msg> {
    let message = match message {
        Some(message) => message,
        None => return empty![],
    };
    div![raw![&message.html],]
}

pub fn view_url(model: &Model) -> Node<Msg> {
    ol![
        li![
            button![
                "Go to '/music/depth1'` and reload the page",
                ev(Ev::Click, |_| {
                    Url::new()
                        .set_path(&["music"])
                        //.set_search(UrlSearch::new(vec![
                            //("depth", vec!["1"])
                        //]))
                        //.set_hash("hash")
                        .go_and_load();
                })
            ],
        ],
        li![
            format!("Base path ...... \"{}\"  ......  (comment out `base` element in `index.html`, refresh the page and watch changes)", &model.base_path.join("/")),
        ],
        li![
            format!("Initial Url ...... \"{}\"", &model.initial_url),
        ],
        li![
            format!("Base Url ...... \"{}\"  ......  (the path part is the most important here)", &model.base_url),
        ],
        li![
            format!("Next path part ...... \"{:?}\"", &model.next_path_part),
        ],
        li![
            format!("Remaining path parts ...... \"{:?}\"", &model.remaining_path_parts),
        ],
        li![
            button![
                "Go to '/' and don't trigger `UrlChanged`",
                ev(Ev::Click, |_| {
                    Url::new().go_and_push();
                })
            ],
        ],
        li![
            button![
                "Go back",
                ev(Ev::Click, |_| {
                    Url::go_back(1);
                })
            ],
        ],
        li![
            button![
                "Go to '/' and trigger `UrlChanged` (simulate `<a>` link click)",
                ev(Ev::Click, |_| Msg::GoToUrl(Url::new()))
            ],
        ],
        li![
            button![
                "Go to 'https://example.com'",
                ev(Ev::Click, |_| {
                    Url::go_and_load_with_str("https://example.com");
                })
            ],
        ],
    ]
}

pub fn header(base_url: &Url) -> Node<Msg> {
    ul![
        li![a![
            attrs! { At::Href => Urls::new(base_url).home() },
            "Home",
        ]],
        li![a![
            attrs! { At::Href => Urls::new(base_url).music_urls().root() },
            "Music",
        ]],
        li![a![
            attrs! { At::Href => Urls::new(base_url).admin_urls().root() },
            "Admin",
        ]],
    ]
}

pub fn view_token<Ms>(message: &Option<LoginMessageResponseBody>) -> Node<Ms> {
    let message = match message {
        Some(message) => message,
        None => return empty![],
    };
    div!["The toekn is: ", &message.token]
}
