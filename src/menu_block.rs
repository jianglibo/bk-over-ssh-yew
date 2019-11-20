use yew::{html, Component, ComponentLink, Href, Html, Renderable, ShouldRender};
use yew::prelude::*;
use yew::html::Children;
use crate::Scene;

pub struct MainMenu {
    props: Props,
}

pub enum Msg {
    MenuItemClicked(Scene),
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub on_menu_clicked: Callback<Scene>,
    pub active_scene: Scene,
}

impl MainMenu {
    fn get_item_classes(&self, scene: Scene) -> String {
        let s = scene.clone().to_string();
        let s1 = self.props.active_scene.clone().to_string();
        js!{console.log("--" + @{s});};
        js!{console.log("++" + @{s1});};
        let c = if self.props.active_scene == scene {
            "pure-menu-item pure-menu-selected"
        } else {
            "pure-menu-item"
        };
        c.to_string()
    }
}


impl Component for MainMenu {

    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        MainMenu { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MenuItemClicked(scene) => {
                self.props.active_scene = scene.clone();
                self.props.on_menu_clicked.emit(scene);
            }
        }
        true
    }
    fn view(&self) -> Html<Self> {
        html! {
            <div class="pure-menu">
                <a class="pure-menu-heading" href="#">{"Company"}</a>

                <ul class="pure-menu-list">
                    <li class="pure-menu-item">
                        <a href="#" class="pure-menu-link">{"Home"}</a>
                    </li>
                    <li class="pure-menu-item">
                        <a href="#" class={self.get_item_classes(Scene::InnerHtml)} onclick=|_|Msg::MenuItemClicked(Scene::InnerHtml)>{Scene::InnerHtml}</a>
                    </li>
                    // menu-item-divided
                    <li class="pure-menu-item">
                        <a href="#" class={self.get_item_classes(Scene::Home)} onclick=|_|Msg::MenuItemClicked(Scene::Home)>{Scene::Home}</a>
                    </li>

                    <li class="pure-menu-item">
                        <a href="#" class={self.get_item_classes(Scene::Login)} onclick=|_|Msg::MenuItemClicked(Scene::Login)>{Scene::Login}</a>
                    </li>
                </ul>
            </div>
        }
    }

}
