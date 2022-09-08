use serde::{Deserialize, Serialize};

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

// `Model` describes our app state.
#[derive(Default)]
pub struct Model {
    pub counter: i32,
    pub new_message: String,
    pub response_html: Option<ResponseHtml>,
    pub response_data: Option<SendMessageResponseBody>,
    pub response_data_get: Option<SendMessageResponseBodyGet>,
    pub response_data_get_vec: Option<SendMessageResponseBodyGetVec>,
}
