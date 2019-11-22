use super::message_box::{MessageBox, MessageType};
use super::request_otp_btn::RequestOtpBtn;
use crate::utils;
use serde::{Deserialize, Serialize};
use stdweb::web::event::{IEvent, SubmitEvent};
use stdweb::web::html_element::InputElement;
use yew::format::json::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct LoginPage {
    user_name: NodeRef,
    otp: NodeRef,
    fetcher: FetchService,
    callback_login: Callback<Response<Json<Result<LoginResult, failure::Error>>>>,
    callback_request_otp: Callback<Response<Result<String, failure::Error>>>,
    fetch_task_holder: Option<FetchTask>,
    message: (&'static str, MessageType),
    props: Props,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "result")]
pub enum LoginResult {
    Success,
    Failed,
}

#[derive(Serialize)]
pub struct LoginFormData {
    email_or_mobile: String,
    otp: String,
}

pub enum Msg {
    FormAboutSubmit(SubmitEvent),
    RequestOtpStart,
    RequestOtpFailed,
    RequestOtpSuccess,
    LoginSuccess,
    LoginFailed,
}

#[derive(Properties)]
pub struct Props {
    pub login_url: String,
    pub otp_url: String,
    pub btn_disable_delay: u64,
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = Props;

    fn create(mut props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        if props.login_url.is_empty() {
            props.login_url = "/login-ajax".to_owned();
        }
        if props.otp_url.is_empty() {
            props.otp_url = "/otp".to_owned();
        }
        if props.btn_disable_delay == 0 {
            props.btn_disable_delay = 180;
        }
        LoginPage {
            user_name: NodeRef::default(),
            otp: NodeRef::default(),
            callback_login: link.send_back(
                |response: Response<Json<Result<LoginResult, failure::Error>>>| {
                    if let (meta, Json(Ok(body))) = response.into_parts() {
                        if meta.status.is_success() {
                            if let LoginResult::Success = body {
                                return Msg::LoginSuccess;
                            }
                        }
                    }
                    Msg::LoginFailed
                },
            ),
            callback_request_otp: link.send_back(
                |response: Response<Result<String, failure::Error>>| {
                    js! {
                        console.log("yyyyy");
                    }
                    if let (meta, Ok(_body)) = response.into_parts() {
                        if meta.status.is_success() {
                            return Msg::RequestOtpSuccess;
                        }
                    }
                    Msg::RequestOtpFailed
                },
            ),
            fetcher: FetchService::new(),
            fetch_task_holder: None,
            message: ("", MessageType::Info),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FormAboutSubmit(se) => {
                se.prevent_default();
                let email_or_mobile = self
                    .user_name
                    .try_into::<InputElement>()
                    .expect("user_name should be a input element.")
                    .raw_value();
                let otp = self
                    .otp
                    .try_into::<InputElement>()
                    .expect("otp should be a input element.")
                    .raw_value();
                let data = LoginFormData {
                    email_or_mobile,
                    otp,
                };

                let post_request = Request::post(self.props.login_url.as_str())
                    .header("Content-Type", "application/json")
                    .body(data)
                    .expect("Failed to build request.");

                let post_request =
                    utils::serialize(post_request).expect("should serialize request.");
                let task = self
                    .fetcher
                    .fetch(post_request, self.callback_login.clone());
                self.fetch_task_holder.replace(task);
            }
            Msg::RequestOtpStart => {
                let post_request = Request::post(self.props.otp_url.as_str())
                    .header("Content-Type", "application/json")
                    .body(utils::get_empty_body())
                    .expect("Failed to build request.");

                let task = self
                    .fetcher
                    .fetch(post_request, self.callback_request_otp.clone());
                self.fetch_task_holder.replace(task);
            }
            Msg::LoginSuccess => {
                
            }
            Msg::LoginFailed => {
                self.message = ("密码错误，请确保输入了最新且没有过期的密码", MessageType::Danger);
            }
            Msg::RequestOtpSuccess => {
                self.message = ("密码发送成功，请检查您的邮箱或者手机。", MessageType::Success);
            }
            Msg::RequestOtpFailed => {
                self.message = ("密码发送成功失败，请稍候尝试。", MessageType::Warning);
                self.props.btn_disable_delay = 0;
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="content">
                <MessageBox message=self.message.0 mtype=self.message.1.clone() delay_secs=5/>
                <form class="pure-form pure-form-aligned" onsubmit= |e|Msg::FormAboutSubmit(e)>
                    <fieldset>

                        <div class="pure-control-group">
                            <label for="email">{"邮件地址"}</label>
                            <input ref=self.user_name.clone() name="email_or_mobile" id="email" type="email" placeholder="Email Address"/>
                            <span class="pure-form-message-inline">{"用来接收OTP"}</span>
                        </div>

                        <div class="pure-control-group">
                            <label for="password">{"OTP(一次性密码)"}</label>
                            <input ref=self.otp.clone() id="password" name="otp" type="password" placeholder="Password"/>
                            <span class="pure-form-message-inline">
                                <RequestOtpBtn delay_secs=self.props.btn_disable_delay on_request_otp_start=|_|Msg::RequestOtpStart/>
                            </span>
                        </div>
                            <div class="pure-controls">
                            <button type="submit" class="pure-button pure-button-primary">{"发送"}</button>
                        </div>
                    </fieldset>
                </form>
            </div>
        }
    }
}
