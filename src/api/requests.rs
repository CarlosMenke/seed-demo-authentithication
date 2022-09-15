use crate::api::get_api_url;
use crate::model::*;
use seed::prelude::*;

pub async fn send_message(new_message: String) -> fetch::Result<SendMessageResponseBodyGet> {
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

pub async fn get_login(name: String) -> fetch::Result<LoginMessageResponseBody> {
    fetch(
        Request::new(get_api_url(String::from("test/login.json")))
            .method(Method::Post)
            .json(&LoginMessageRequestBody {
                username: name,
                permissions: Vec::from(["ROLE_ADMIN".to_string()]),
            })?,
    )
    .await?
    .check_status()?
    .json()
    .await
}

pub async fn get_admin(token: String) -> fetch::Result<SendMessageResponseBodyGet> {
    Request::new(get_api_url(String::from("test/auth/admin.json")))
        .header(Header::bearer(token))
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}
