use serde::Deserialize;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub enum Msg {
    CopyToClipboard,
}
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Insd {
    pub filename: String,
    pub value: String,
}

pub struct Question {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: Insd,
}

impl Component for Question {
    type Message = Msg;
    type Properties = Insd;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props: props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CopyToClipboard => unsafe {
                get_payload(
                    wasm_bindgen::JsValue::from("asf"),
                    wasm_bindgen::JsValue::from(self.props.value.clone()),
                );
                true
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        true
    }

    fn view(&self) -> Html {
        let r = self.props.filename.clone();
        let mut imagn = self.props.filename.clone();
        imagn.push_str(".png");
        imagn.insert_str(0, "./process/");
        let v: Vec<&str> = r.split('/').collect();
        let mut lik = self.props.value.clone();
        lik.insert_str(0, "http://www.google.com/search?q=");
        // lik = lik.replace(" ", "+");
        html! {
            <div class="quest">
                <div class="questionno">
                    <div class="no">{"Ques "}{v[0]}{". "}</div>
                    <button class="button" onclick=self.link.callback(|_|Msg::CopyToClipboard)><span>{"Copy Ques"}</span></button>
                    <div class="search">
                        <a href=lik target="_blank">{"Search"}</a>
                    </div>
                </div>
                <div class="question">{self.props.value.clone()}</div>
                <img src=imagn/>
            </div>
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "get_payload")]
    fn get_payload(cmd_string: JsValue, cmd2_string: JsValue);
}
