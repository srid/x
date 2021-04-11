#![recursion_limit = "10000"]
use yew::services::fetch::{Request, Response};
use yew::{format::Nothing, services::FetchService};
use yew::{prelude::*, services::fetch::FetchTask};
use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
enum AppRoute {
    #[to = "/test"]
    Test,
    #[to = "/"]
    Index,
}

enum Msg {
    LoadInfo,
    SetInfo(Result<String, anyhow::Error>),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    msg: String,
    fetch_task: Option<FetchTask>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            msg: "...".to_string(),
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoadInfo => {
                // Playing with network fetch ...
                let url = "https://www.reddit.com/r/TheMotte/top.json";
                let request = Request::get(url)
                    .body(Nothing)
                    .expect("Could not build that request");
                let callback = self
                    .link
                    .callback(|rsp: Response<_>| Msg::SetInfo(rsp.into_body()));
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::SetInfo(response) => {
                match response {
                    Ok(s) => self.msg = s,
                    Err(error) => self.msg = error.to_string(),
                };
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::LoadInfo)
        }
    }

    fn view(&self) -> Html {
        html! {
            <body>
                <div class="container mx-auto">
                    <Router<AppRoute, ()>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Test => html!{
                                <RouterAnchor<AppRoute>
                                    route=AppRoute::Index
                                    >
                                    { "Index" }
                                </RouterAnchor<AppRoute>>
                                },
                                AppRoute::Index => html! {
                                <div>
                                    <RouterAnchor<AppRoute>
                                        route=AppRoute::Test
                                        >
                                        { "Test" }
                                    </RouterAnchor<AppRoute>>
                                </div>
                                }
                            }
                        })
                    />

                    <div class="border-t-1">
                        <button
                            class="border-2 rounded p-2 bg-purple-200"
                            onclick=self.link.callback(|_|
                            Msg::LoadInfo)>{ "Refresh" }</button>
                        <p>{ self.value }</p>
                        <div class="monospace overflow">
                            { self.msg.clone() }
                        </div>
                    </div>

                </div>
            </body>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
