mod question;
use anyhow::Error;
use serde::Deserialize;

// use std::error::Error;
use question::Question;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

use yew::services::ConsoleService;

#[derive(Deserialize, Debug)]
struct Data {
    value: Vec<Insd>,
}
#[derive(Deserialize, Debug)]
struct Insd {
    filename: String,
    value: String,
}

enum Msg {
    Initialize,
    FetchResourceComplete(Data),
    FetchResourceFailed,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    data: Data,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
}
impl Model {
    fn view_data(&self) -> Html {
        self.data
            .value
            .iter()
            .enumerate()
            .map(|(i, e)| {
                html! {
                    <Question filename=e.filename.clone() value=e.value.replace("\n\n", "\n") />
                    // <div class="quest">
                    //     <div class="questionno">
                    //     <div class="no">{"Ques "}{e.filename.clone()}{". "}</div>
                    //     </div>
                    //     <div class="question">{e.value.clone()}</div>
                    // </div>
                }
            })
            .collect::<Html>()
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            data: Data { value: Vec::new() },
            link,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Initialize => {
                let get_request =
                    Request::get("https://divergent-anonymous.github.io/process/questionlist.json")
                        .body(Nothing)
                        .expect("could not build the request");

                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Data, Error>>>| {
                            if let (meta, Json(Ok(body))) = response.into_parts() {
                                if meta.status.is_success() {
                                    return Msg::FetchResourceComplete(body);
                                }
                            } else {
                            }
                            Msg::FetchResourceFailed
                        });

                let task = FetchService::fetch(get_request, callback).expect("sdf");
                self.fetch_task = Some(task);
                true
            }
            Msg::FetchResourceComplete(e) => {
                self.data = e;
                true
            }
            Msg::FetchResourceFailed => true,
            _ => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        true
    }
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::Initialize);
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Divergent"} </h1>
                <ol>
                    <li>{"Page will only Update in Incognito Mode"}</li>
                    <li>{"Website have 20sec latency for updates"}</li>
                </ol>
                <h3>{"Add your answers in SpreedSheet"}</h3>
                <a target="_blank" href="https://github.com/divergent-anonymous/divergent-anonymous.github.io/blob/main/process/all.txt">{"To Get Raw Text File (Updates Quickly with in 3 sec) Github Link for file"}</a>
                <p></p>
                {self.view_data()}
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
//             unsafe {
//                 get_payload(wasm_bindgen::JsValue::from("asf"), wasm_bindgen::JsValue::from("asf"));
//                 // get_payload(js_sys::JsString::from("don"), js_sys::JsString::from("don"));
//             }
