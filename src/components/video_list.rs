use yew::prelude::*;
use crate::models::Video;

#[derive(Properties, PartialEq)]
pub struct VideoListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>
}

#[function_component(Videoslist)]
pub fn videos_list(VideoListProps { videos, on_click }: &VideoListProps) -> Html {
    let on_click = on_click.clone();

    videos.iter().map(|video| {
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone())
            })
        };

        html! {
            <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }).collect()
}