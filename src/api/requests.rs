use crate::api::get_api_url;
use crate::model::*;
use seed::prelude::*;

pub async fn send_message(new_message: String) -> fetch::Result<SendMessageResponseBody> {
    fetch(
        Request::new(get_api_url(String::from("test/post.json")))
            .method(Method::Post)
            //.mode(web_sys::RequestMode::NoCors)
            .json(&SendMessageRequestBody { text: new_message })?,
    )
    .await?
    .check_status()?
    .json()
    .await
}

pub async fn get_message() -> fetch::Result<SendMessageResponseBodyGet> {
    Request::new(get_api_url(String::from("test/get.json")))
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn get_vec_message() -> fetch::Result<SendMessageResponseBodyGetVec> {
    fetch(get_api_url(String::from("test/get_vec.json")))
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn get_html() -> fetch::Result<ResponseHtml> {
    fetch(get_api_url(String::from("test/html.html")))
        .await?
        .check_status()?
        .json()
        .await
}
