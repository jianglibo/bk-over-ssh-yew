use crate::main_block::MainBlock;
use crate::menu_block::MainMenu;
use crate::inner_html;
use failure;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use stdweb::web::Date;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use yew::format::json::Json;
use yew::services::fetch::{FetchService, Request, Response};
use yew::services::{ConsoleService, IntervalService, Task, TimeoutService};
use yew::{html, Callback, Component, ComponentLink, Href, Html, Renderable, ShouldRender};

pub struct MyApp {
    scene: Scene,
    timeout: TimeoutService,
    interval: IntervalService,
    console: ConsoleService,
    callback_tick: Callback<()>,
    callback_done: Callback<()>,
    callback_login: Callback<Response<Json<Result<LoginResult, failure::Error>>>>,
    job: Option<Box<dyn Task>>,
    messages: Vec<&'static str>,
    standalone: Option<Box<dyn Task>>,
    fetcher: FetchService,
}

pub enum Msg {
    StartTimeout,
    StartInterval,
    Cancel,
    Done,
    Tick,
    LogTime,
    FetchResourceComplete(LoginResult),
    FetchResourceFailed,
}

#[derive(Clone, Debug, Display, EnumString, EnumIter, PartialEq)]
enum Scene {
    Login,
    UserList,
    InnerHtml,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "result")]
pub enum LoginResult {
    Success,
    Failed,
}

impl Component for MyApp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let interval = IntervalService::new();

        let mut myapp = MyApp {
            scene: Scene::UserList,
            timeout: TimeoutService::new(),
            interval,
            console: ConsoleService::new(),
            callback_tick: link.send_back(|_| Msg::Tick),
            callback_done: link.send_back(|_| Msg::Done),
            callback_login: link.send_back(
                |response: Response<Json<Result<LoginResult, failure::Error>>>| {
                    if let (meta, Json(Ok(body))) = response.into_parts() {
                        if meta.status.is_success() {
                            return Msg::FetchResourceComplete(body);
                        }
                    }
                    Msg::FetchResourceFailed
                },
            ),
            job: None,
            messages: Vec::new(),
            standalone: None,
            fetcher: FetchService::new(),
        };

        let c = link.send_back(|_| Msg::LogTime);
        let handle = myapp.interval.spawn(Duration::from_secs(3), c);

        myapp.standalone.replace(Box::new(handle));

        myapp
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => (),
            Msg::LogTime => {
                let j = json!({"foo": "bar"});
                let post_request = Request::post("/resource")
                    .header("Content-Type", "application/json")
                    .body(Json(&j))
                    .expect("Failed to build request.");

                let task = self
                    .fetcher
                    .fetch(post_request, self.callback_login.clone());
                let scene = format!("{}", self.scene);
                self.console.log(scene.as_str());
                match self.scene {
                    Scene::UserList => self.scene = Scene::Login,
                    Scene::Login => self.scene = Scene::InnerHtml,
                    Scene::InnerHtml => self.scene = Scene::UserList,
                }
            }
            _ => self.console.log("hello"),
        }
        true
    }
    fn view(&self) -> Html<Self> {
        html! {
            <div id="layout">
            // <!-- Menu toggle -->
            <a href="#menu" id="menuLink" class="menu-link">
                // <!-- Hamburger icon -->
                <span></span>
            </a>

            <div id="menu">
                <MainMenu/>
            </div>

            { self.view_scene() }
        </div>
        }
    }
}

impl MyApp {
    fn view_scene(&self) -> Html<Self> {
        match self.scene {
            Scene::UserList => html! {<MainBlock/>},
            Scene::Login => html! {<p>{"waiting for implementation."}</p>},
            Scene::InnerHtml => html!{<inner_html::Model/>},
        }
    }
}
