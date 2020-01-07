use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::prelude::*;
use stdweb::web::event::{ClickEvent, IEvent};
use yew::services::{IntervalService, Task, TimeoutService};
use std::time::Duration;

pub struct RequestOtpBtn {
    props: Props,
    timeout: TimeoutService,
    interval: IntervalService,
    job: Option<Box<dyn Task>>,
    timeout_job: Option<Box<dyn Task>>,
    callback_done: Callback<()>,
    callback_tick: Callback<()>,
    count_down: u64,
    link: ComponentLink<Self>,
}

pub enum Msg {
    RequestOtp(ClickEvent),
    TimeUp,
    Tick,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[props(required)]
    pub on_request_otp_start: Callback<()>,
    pub delay_secs: u64,
}


impl Component for RequestOtpBtn {

    type Message = Msg;
    type Properties = Props;

    fn create(mut props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        if props.delay_secs == 0 {
            props.delay_secs = 3;
        };
        RequestOtpBtn {
            props,
            timeout: TimeoutService::new(),
            interval: IntervalService::new(),
            job: None,
            timeout_job: None,
            callback_done: link.callback(|_| Msg::TimeUp),
            callback_tick: link.callback(|_| Msg::Tick),
            count_down: 0,
            link,
         }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.delay_secs == 0 {
            self.count_down = 0;
            if let Some(mut task) = self.job.take() {
                task.cancel();
            }
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestOtp(ev) => {
                ev.prevent_default();
                ev.stop_propagation();
                js! {
                    console.log(@{ev});
                }
                {
                let handle = self
                        .timeout
                        .spawn(Duration::from_secs(self.props.delay_secs), self.callback_done.clone());
                self.count_down = self.props.delay_secs;
                self.timeout_job.replace(Box::new(handle));
                }

                {
                    let handle = self
                        .interval
                        .spawn(Duration::from_secs(1), self.callback_tick.clone());
                    self.job = Some(Box::new(handle));
                }
                self.props.on_request_otp_start.emit(());
            },
            Msg::TimeUp => {
                js! {
                    console.log("timeout");
                }
                self.count_down = 0;
                if let Some(mut task) = self.job.take() {
                    task.cancel();
                }
            },
            Msg::Tick => {
                if self.count_down > 0 {
                    self.count_down -= 1;
                }
                js! {
                    console.log("ticking");
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <button disabled={self.count_down > 0} onclick=self.link.callback(|e|Msg::RequestOtp(e)) type="button" class="pure-button">{self.get_count_down()}</button>
        }
    }
}

impl RequestOtpBtn {
    fn get_count_down(&self) -> String {
        if self.count_down > 0 {
            format!("请求一个密码({})", self.count_down)
        } else {
            "请求一个密码".to_string()
        }
    }
}