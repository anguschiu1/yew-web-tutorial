mod video;
use crate::video::Video;
use crate::video::VideoDetails;
use crate::video::VideoList;
use gloo_net::http::Request;
use yew::prelude::*;

// The fetch will be blocked by CORS.To make this work, we need to use proxy setup in trunk serve, as follows:
// ❯ trunk serve --proxy-backend=https://yew.rs/blahblahblah
// You cannot use "https://yew.rs/" as backend, as it will override html.

#[function_component(App)]
fn app() -> Html {
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });
            || ()
        });
    }

    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video));
        })
    };
    let details = selected_video.as_ref().map(|video| {
        html! {
        <VideoDetails video={video.clone()} />
        }
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideoList videos={ (*videos).clone() } on_click={on_video_select.clone()} />
            </div>
            { for details } // Option<VNode> has implemented IntoIterator, hence it can be used in a for loop
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
