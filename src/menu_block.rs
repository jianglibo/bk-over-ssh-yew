use crate::Scene;
use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct MainMenu {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    MenuItemClicked(Scene),
}

#[derive(Properties, Clone)]
pub struct Props {
    #[props(required)]
    pub on_menu_clicked: Callback<Scene>,
    pub active_scene: Scene,
}

impl MainMenu {
    fn get_item_classes(&self, scene: &Scene, divided: bool) -> String {
        let c = if &self.props.active_scene == scene {
            "pure-menu-item pure-menu-selected"
        } else {
            "pure-menu-item"
        };
        if divided {
            format!("{} {}", c, "menu-item-divided")
        } else {
            c.to_string()
        }
    }

    fn menu_item(&self, scene: Scene, divided: bool) -> Html {
        let c = self.get_item_classes(&scene, divided);
        let cloned = scene.clone();
        html! {
            <li class={c}>
                <a href="#" class="pure-menu-link" onclick=self.link.callback(move |_|Msg::MenuItemClicked(scene.clone()))>{cloned}</a>
            </li>
        }
    }
}

impl Component for MainMenu {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MainMenu { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MenuItemClicked(scene) => {
                self.props.on_menu_clicked.emit(scene);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        js! {
            console.log("props:" + @{props.active_scene.to_string()});
        }
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="pure-menu">
                <a class="pure-menu-heading" href="#">{"Company"}</a>
                <ul class="pure-menu-list">
                    {self.menu_item(Scene::Home, true)}
                    {self.menu_item(Scene::Login, false)}
                    {self.menu_item(Scene::Accounts, false)}
                </ul>
            </div>
        }
    }
}
