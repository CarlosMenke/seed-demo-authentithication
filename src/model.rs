use crate::page;
use seed::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

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
//
// ------ ------
//     Model
// ------ ------

pub struct Model {
    pub base_path: Rc<[String]>,
    pub initial_url: Url,
    pub next_path_part: Option<String>,
    pub remaining_path_parts: Vec<String>,
    pub base_url: Url,
    pub page_id: Option<PageId>,
    pub music_model: Option<page::music::Model>,

    pub counter: i32,
    pub new_message: String,
    pub response_html: Option<ResponseHtml>,
    pub response_data: Option<SendMessageResponseBody>,
    pub response_data_get: Option<SendMessageResponseBodyGet>,
    pub response_data_get_vec: Option<SendMessageResponseBodyGetVec>,
}

impl Model {
    pub fn new(mut url: Url, base_path: Rc<[String]>) -> Self {
        log!(&url);
        log!(url.to_string());
        log!("_______________________________");

        Self {
            base_path,
            initial_url: url.clone(),
            base_url: url.to_base_url(),
            next_path_part: url.next_path_part().map(ToOwned::to_owned),
            remaining_path_parts: url
                .remaining_path_parts()
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
            page_id: None,
            music_model: None,
            counter: 0,
            new_message: "".to_string(),
            response_html: Some(ResponseHtml::default()),
            response_data: Some(SendMessageResponseBody::default()),
            response_data_get: Some(SendMessageResponseBodyGet::default()),
            response_data_get_vec: Some(SendMessageResponseBodyGetVec::default()),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PageId {
    Home,
    Music,
}
impl PageId {
    pub fn name(self) -> String {
        match self {
            PageId::Home => "Home".to_string(),
            PageId::Music => "Music".to_string(),
        }
    }
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn music_urls(self) -> page::music::Urls<'a> {
        //TODO replace Music with constant
        page::music::Urls::new(self.base_url().add_path_part(PageId::Music.name()))
    }
}
