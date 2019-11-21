use yew::{html, Component, ComponentLink, Href, Html, Renderable, ShouldRender};
use yew::prelude::*;
use yew::services::{ConsoleService, IntervalService, Task, TimeoutService};
use stdweb::web::html_element::InputElement;
use stdweb::web::event::{ClickEvent, IEvent, SubmitEvent};
use super::request_otp_btn::RequestOtpBtn;

pub struct LoginPage {
    user_name: NodeRef,
    otp: NodeRef,
    console: ConsoleService,
}

pub enum Msg {
    FormAboutSubmit(SubmitEvent),
    RequestOtp,
}

impl Component for LoginPage {

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        LoginPage {
            user_name: NodeRef::default(),
            otp: NodeRef::default(),
            console: ConsoleService::new(),
         }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FormAboutSubmit(se) => {
                se.prevent_default();
                let user_name = self.user_name.try_into::<InputElement>().expect("it's a input element.");
                self.console.log(&format!("{}", user_name.raw_value()));
            },
            Msg::RequestOtp => {
                js! {
                    console.log("abc");
                }
            },
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

