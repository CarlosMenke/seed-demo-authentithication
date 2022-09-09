use crate::model::*;
use crate::Msg;
use seed::{prelude::*, *};

pub fn view_music() -> Vec<Node<Msg>> {
    raw![include_str!("../../static/music_all.html")]
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

fn view_url(model: &Model) -> Node<Msg> {
    ol![
        li![
            button![
                "Go to '/ui/a/b/c?x=1?#hash'` and reload the page",
                ev(Ev::Click, |_| {
                    Url::new()
                        .set_path(&["ui", "a", "b", "c"])
                        .set_search(UrlSearch::new(vec![
                            ("x", vec!["1"])
                        ]))
                        .set_hash("hash")
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
