use super::request_otp_btn::RequestOtpBtn;
use crate::utils;
use serde::{Deserialize, Serialize};
use stdweb::web::event::{ClickEvent, IEvent, SubmitEvent};
use stdweb::web::html_element::InputElement;
use yew::format::json::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::{ConsoleService, IntervalService, Task, TimeoutService};
use yew::{html, Component, ComponentLink, Href, Html, Renderable, ShouldRender};

pub struct LoginPage {
    user_name: NodeRef,
    otp: NodeRef,
    console: ConsoleService,
    fetcher: FetchService,
    callback_login: Callback<Response<Json<Result<LoginResult, failure::Error>>>>,
    fetch_task_holder: Option<FetchTask>,
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
    RequestOtp,
    LoginSuccess,
    LoginFailed,
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        LoginPage {
            user_name: NodeRef::default(),
            otp: NodeRef::default(),
            console: ConsoleService::new(),
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
            fetcher: FetchService::new(),
            fetch_task_holder: None,
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

                let post_request = Request::post("/resource")
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
            Msg::RequestOtp => {
                js! {
                    console.log("abc");
                }
            }
            Msg::LoginSuccess => {}
            Msg::LoginFailed => {}
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="content">
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
                                <RequestOtpBtn delay_secs=180 on_request_otp=|_|Msg::RequestOtp/>
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
