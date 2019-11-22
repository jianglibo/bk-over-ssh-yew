use std::time::Duration;
use yew::prelude::*;
use yew::services::{Task, TimeoutService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[derive(PartialEq, Debug, Clone)]
pub enum MessageType {
    Info,
    Success,
    Warning,
    Danger,
}

impl Default for MessageType {
    fn default() -> MessageType {
        MessageType::Info
    }
}


pub struct MessageBox {
    props: Props,
    timeout: TimeoutService,
    timeout_job: Option<Box<dyn Task>>,
    callback_done: Callback<()>,
}

pub enum Msg {
    TimeUp,
    MessageClicked,
}

#[derive(Properties)]
pub struct Props {
    pub delay_secs: u64,
    pub message: String,
    pub mtype: MessageType,
}

impl Component for MessageBox {
    type Message = Msg;
    type Properties = Props;

    fn create(mut props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        if props.delay_secs == 0 {
            props.delay_secs = 3;
        };
        let mut mb = MessageBox {
            props,
            timeout: TimeoutService::new(),
            callback_done: link.send_back(|_| Msg::TimeUp),
            timeout_job: None,
        };

        if !mb.props.message.is_empty() {
            let handle = mb.timeout.spawn(
                Duration::from_secs(mb.props.delay_secs),
                mb.callback_done.clone(),
            );
            mb.timeout_job.replace(Box::new(handle));
        }

        mb
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.message != props.message {
            self.props = props;
            let handle = self.timeout.spawn(
                Duration::from_secs(self.props.delay_secs),
                self.callback_done.clone(),
            );
            self.timeout_job.replace(Box::new(handle));
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TimeUp => {
                self.props.message = "".to_owned();
            }
            Msg::MessageClicked => {
                self.props.message = "".to_owned();
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        if self.props.message.is_empty() {
            html! {}
        } else {
            html! {
                <aside class=self.get_classes() onclick=|_|Msg::MessageClicked>
                    <p>
                        {self.props.message.as_str()}
                    </p>
                </aside>
            }
        }
    }
}

impl MessageBox {

    fn get_classes(&self) -> String {
        match self.props.mtype {
            MessageType::Info => "alert alert-info",
            MessageType::Warning => "alert alert-warning",
            MessageType::Success => "alert alert-success",
            MessageType::Danger => "alert alert-danger",
        }
        .to_string()
    }
}
