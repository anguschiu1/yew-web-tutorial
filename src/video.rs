use serde::Deserialize;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Deserialize)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideoListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>,
}

#[function_component(VideoList)]
// A function component takes only one argument which defines its "props" (short for "properties")
// VideoListProps { videos } desctructures the videos property from VideoListProps
pub fn video_list(VideoListProps { videos, on_click }: &VideoListProps) -> Html {
    let on_click = on_click.clone();
    videos
      .iter()
      .map(|video| {
          let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| on_click.emit(video.clone()))
          };

          html! {
            <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
          }
      })
      .collect::<Html>()
}

#[derive(Clone, Properties, PartialEq)]
pub struct VideosDetailsProps {
    pub video: Video,
}

#[function_component(VideoDetails)]
pub fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}
