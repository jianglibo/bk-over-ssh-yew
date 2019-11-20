use yew::{html, Component, ComponentLink, Href, Html, Renderable, ShouldRender};
use stdweb::unstable::TryFrom;
use stdweb::web::Node;
use yew::virtual_dom::VNode;

pub const INNTER_HTML_STR: &[u8] = include_bytes!("anhtml.html");

fn get_inner_html_str() -> &'static str {
    std::str::from_utf8(INNTER_HTML_STR).expect("include_bytes should work.")
}

pub struct Model {
    pub value: i64,
}

pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { value: 0 }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        // let js_svg = js! {
        //     var div = document.createElement("div");
        //     var dd = @{get_inner_html_str().to_string()};
        //     console.log(dd);
        //     div.innerHTML = dd;
        //     console.log(div);
        //     return div;
        // };
        let node = Node::from_html(get_inner_html_str()).expect("js_svg from_html should success.");
        // eprintln!("js_svg: {:?}", js_svg);
        // let node = Node::try_from(js_svg).expect("convert js_svg");
        let vnode = VNode::VRef(node);
        eprintln!("svg: {:?}", vnode);
        vnode
    }
}
