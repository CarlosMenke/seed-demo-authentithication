use crate::page;
use seed::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageRequestBody {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoginMessageRequestBody {
    pub username: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoginMessageResponseBody {
    pub username: String,
    pub permissions: Vec<String>,
    pub token: String,
}

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
    pub admin_model: Option<page::admin::Model>,

    pub counter: i32,
    pub new_message: String,
    pub login_name: String,
    pub response_html: Option<ResponseHtml>,
    pub response_data: Option<SendMessageResponseBodyGet>,
    pub response_data_get: Option<SendMessageResponseBodyGet>,
    pub response_data_get_vec: Option<SendMessageResponseBodyGetVec>,
    pub response_login: Option<LoginMessageResponseBody>,
    pub response_admin: Option<SendMessageResponseBodyGet>,
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
            admin_model: None,
            counter: 1,
            new_message: "".to_string(),
            login_name: "".to_string(),
            response_html: Some(ResponseHtml::default()),
            response_data: Some(SendMessageResponseBodyGet::default()),
            response_data_get: Some(SendMessageResponseBodyGet::default()),
            response_data_get_vec: Some(SendMessageResponseBodyGetVec::default()),
            response_login: Some(LoginMessageResponseBody::default()),
            response_admin: Some(SendMessageResponseBodyGet::default()),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PageId {
    Home,
    Music,
    Admin,
}
impl PageId {
    pub fn name(self) -> String {
        match self {
            PageId::Home => "Home".to_string(),
            PageId::Music => "Music".to_string(),
            PageId::Admin => "Admin".to_string(),
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
    pub fn admin_urls(self) -> page::admin::Urls<'a> {
        //TODO replace Music with constant
        page::admin::Urls::new(self.base_url().add_path_part(PageId::Admin.name()))
    }
}
