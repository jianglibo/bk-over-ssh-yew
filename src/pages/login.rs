use super::message_box::{MessageBox, MessageType};
use super::request_otp_btn::RequestOtpBtn;
use crate::utils;
use serde::{Deserialize, Serialize};
use stdweb::web::event::{IEvent, SubmitEvent};
use stdweb::web::html_element::InputElement;
use yew::format::json::Json;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct LoginPage {
    user_name: NodeRef,
    otp: NodeRef,
    fetcher: FetchService,
    callback_login: Callback<Response<Json<Result<LoginResult, failure::Error>>>>,
    callback_request_otp: Callback<Response<Json<Result<LoginResult, failure::Error>>>>,
    fetch_task_holder: Option<FetchTask>,
    message: (String, MessageType),
    props: Props,
    console: ConsoleService,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "result")]
pub enum LoginResult {
    Success,
    Failed,
    OtpError,
    OtpSendFailed,
    OtpCreateUserFailed(String),
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
    LoginOtpError,
    LoginOtpSendFailed,
    LoginOtpCreateUserFailed(String),
}

#[derive(Properties)]
pub struct Props {
    pub login_url: String,
    pub otp_url: String,
    pub btn_disable_delay: u64,
}

impl LoginPage {
    fn get_login_form_data(&self) -> LoginFormData {
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
        LoginFormData {
            email_or_mobile,
            otp,
        }
    }
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
                            match body {
                                LoginResult::Success => Msg::LoginSuccess,
                                LoginResult::Failed => Msg::LoginFailed,
                                LoginResult::OtpError => Msg::LoginOtpError,
                                _ => panic!("Unexpected response: {:?}", body),
                            }
                        } else {
                            Msg::LoginFailed
                        }
                    } else {
                        Msg::LoginFailed
                    }
                },
            ),
            callback_request_otp: link.send_back(
                |response: Response<Json<Result<LoginResult, failure::Error>>>| {
                    if let (meta, Json(Ok(body))) = response.into_parts() {
                        if meta.status.is_success() {
                            match body {
                                LoginResult::Success => Msg::RequestOtpSuccess,
                                LoginResult::OtpSendFailed => Msg::LoginOtpSendFailed,
                                LoginResult::OtpCreateUserFailed(email) => {
                                    Msg::LoginOtpCreateUserFailed(email)
                                }
                                _ => panic!("Unexpected response: {:?}", body),
                            }
                        } else {
                            Msg::RequestOtpFailed
                        }
                    } else {
                        Msg::RequestOtpFailed
                    }
                },
            ),
            fetcher: FetchService::new(),
            fetch_task_holder: None,
            message: ("".to_string(), MessageType::Info),
            console: ConsoleService::new(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FormAboutSubmit(se) => {
                se.prevent_default();
                let data = self.get_login_form_data();

                if data.email_or_mobile.trim().is_empty() {
                    self.message = ("请输入邮件地址。".to_string(), MessageType::Danger);
                } else if data.otp.trim().is_empty() {
                    self.message = ("请输入一次性密码。".to_string(), MessageType::Danger);
                } else {
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
            }
            Msg::RequestOtpStart => {
                let form_data = self.get_login_form_data();
                if form_data.email_or_mobile.len() < 6
                    || form_data.email_or_mobile.find('@').is_none()
                {
                    self.message = (
                        "请输入接受一次性密码的邮箱地址！".to_string(),
                        MessageType::Danger,
                    );
                    self.props.btn_disable_delay = 0;
                } else {
                    let body = utils::seriablizable_body(&form_data);
                    self.console.info(body.as_ref().unwrap());
                    let post_request = Request::post(self.props.otp_url.as_str())
                        .header("Content-Type", "application/json")
                        .body(body)
                        .expect("Failed to build request.");

                    let task = self
                        .fetcher
                        .fetch(post_request, self.callback_request_otp.clone());
                    self.fetch_task_holder.replace(task);
                }
            }
            Msg::LoginSuccess => {}
            Msg::LoginOtpSendFailed => {
                self.message = ("密码发送过程出错。".to_string(), MessageType::Danger);
            }
            Msg::LoginOtpCreateUserFailed(email) => {
                self.message = (format!("无法创建用户: {}", email), MessageType::Danger);
            }
            Msg::LoginOtpError => {
                self.message = ("一次性密码错误。".to_string(), MessageType::Danger);
            }
            Msg::LoginFailed => {
                self.message = (
                    "密码错误，请确保输入了最新且没有过期的密码".to_string(),
                    MessageType::Danger,
                );
            }
            Msg::RequestOtpSuccess => {
                self.message = (
                    "密码发送成功，请检查您的邮箱或者手机。".to_string(),
                    MessageType::Success,
                );
            }
            Msg::RequestOtpFailed => {
                self.message = (
                    "密码发送失败，请稍候尝试。".to_string(),
                    MessageType::Warning,
                );
                self.props.btn_disable_delay = 0;
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="content">
                <MessageBox message=self.message.0.as_str() mtype=self.message.1.clone() delay_secs=5/>
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
