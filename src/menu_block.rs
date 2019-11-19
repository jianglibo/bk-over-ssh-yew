use yew::{html, Component, ComponentLink, Href, Html, Renderable, ShouldRender};
pub struct MainMenu {

}

pub enum Msg {
    A
}

impl Component for MainMenu {

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MainMenu { }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::A => (),
        }
        true
    }
    fn view(&self) -> Html<Self> {
        html! {
            <div class="pure-menu">
                <a class="pure-menu-heading" href="#">{"Company"}</a>

                <ul class="pure-menu-list">
                    <li class="pure-menu-item"><a href="#" class="pure-menu-link">{"Home"}</a></li>
                    <li class="pure-menu-item"><a href="#" class="pure-menu-link">{"About"}</a></li>

                    <li class="pure-menu-item menu-item-divided
                            pure-menu-selected">
                        <a href="#" class="pure-menu-link">{"Services"}</a>
                    </li>

                    <li class="pure-menu-item"><a href="#" class="pure-menu-link">{"Contact"}</a></li>
                </ul>
            </div>
        }
    }

}
