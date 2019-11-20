use yew::{html, Component, ComponentLink, Href, Html, Renderable, ShouldRender};
use yew::prelude::*;
pub struct MainMenu {
    props: Props,
}

pub enum Msg {
    A(String)
}

#[derive(Properties)]
pub struct Props {
    pub hide: bool,
    #[props(required)]
    pub name: String,
}


impl Component for MainMenu {

    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        MainMenu { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::A(s) => {js!{
                alert(@{s});
            };},
        }
        true
    }
    fn view(&self) -> Html<Self> {
        html! {
            <div class="pure-menu" display={if self.props.hide { "none" } else {"block"}}>
                <a class="pure-menu-heading" href="#">{"Company"}</a>

                <ul class="pure-menu-list">
                    <li class="pure-menu-item"><a href="#" class="pure-menu-link">{"Home"}</a></li>
                    <li class="pure-menu-item"><a href="#" class="pure-menu-link">{"About"}</a></li>

                    <li class="pure-menu-item menu-item-divided
                            pure-menu-selected">
                        <a href="#" class="pure-menu-link">{"Services"}</a>
                    </li>

                    <li class="pure-menu-item"><a href="#" class="pure-menu-link" onclick=|_|Msg::A("abc".to_string())>{"Contact"}</a></li>
                </ul>
            </div>
        }
    }

}
