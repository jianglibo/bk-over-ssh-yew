use crate::main_block::MainBlock;
use crate::menu_block::MainMenu;
use std::time::Duration;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use yew::services::{ConsoleService, IntervalService, Task, TimeoutService};
use yew::{html, Callback, Component, ComponentLink, Href, Html, Renderable, ShouldRender};
use stdweb::web::Date;

pub struct MyApp {
    scene: Scene,
    timeout: TimeoutService,
    interval: IntervalService,
    console: ConsoleService,
    callback_tick: Callback<()>,
    callback_done: Callback<()>,
    job: Option<Box<dyn Task>>,
    messages: Vec<&'static str>,
    standalone: Option<Box<dyn Task>>,
}

pub enum Msg {
    StartTimeout,
    StartInterval,
    Cancel,
    Done,
    Tick,
    LogTime,
}

#[derive(Clone, Debug, Display, EnumString, EnumIter, PartialEq)]
enum Scene {
    Login,
    UserList,
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
            job: None,
            messages: Vec::new(),
            standalone: None,
        };

        let c = link.send_back(|_| Msg::LogTime);
        let handle = myapp.interval.spawn(Duration::from_secs(10), c);

        myapp.standalone.replace(Box::new(handle));

        myapp
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => (),
            Msg::LogTime => {
                let scene = format!("{}", self.scene);
                self.console.log(scene.as_str());
                match self.scene {
                    Scene::UserList => self.scene = Scene::Login,
                    Scene::Login => self.scene = Scene::UserList,
                }
            },
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

            <MainBlock/>
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
        }
    }
}
