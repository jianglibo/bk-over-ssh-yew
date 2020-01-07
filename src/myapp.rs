use crate::pages;
use crate::menu_block::MainMenu;
use crate::Scene;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct MyApp {
    scene: Scene,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Scene(Scene),
}

impl Component for MyApp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        MyApp {
            scene: Scene::Home,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Scene(scene) => {
                self.scene = scene;
                let s = self.scene.clone().to_string();
                js!{console.log("**" + @{s});};
                
            },
        }
        true
    }
    fn view(&self) -> Html {
        html! {
            <div id="layout">
            // <!-- Menu toggle -->
            <a href="#menu" id="menuLink" class="menu-link">
                // <!-- Hamburger icon -->
                <span></span>
            </a>

            <div id="menu">
                // maybe one time setting from parent to child.
                <MainMenu active_scene={self.scene.clone()} on_menu_clicked=self.link.callback(|scene|Msg::Scene(scene))/>
            </div>
            <div id="main">
            <div class="header">
                <h1>{self.get_title()}</h1>
                <h2>{self.get_sub_title()}</h2>
            </div>
                { self.view_scene() }
            </div>
        </div>
        }
    }
}

impl MyApp {
    fn view_scene(&self) -> Html {
        match self.scene {
            Scene::Home => html! {<pages::HomePage/>},
            Scene::Login => html! {<pages::LoginPage/>},
            Scene::Accounts => html! {<pages::AccountsPage/>},
        }
    }

    fn get_title(&self) -> String {
        match self.scene {
            Scene::Home => "通过SSH备份文件",
            Scene::Login => "登录系统",
            Scene::Accounts => "备份账号",
        }.into()
    }

    fn get_sub_title(&self) -> String {
        match self.scene {
            Scene::Home => "把你的服务器上的重要文件通过SSH备份到这里。",
            Scene::Login => "不用注册，使用一次性密码登录系统",
            Scene::Accounts => "备份账号列表。",
        }.into()
    }
}
