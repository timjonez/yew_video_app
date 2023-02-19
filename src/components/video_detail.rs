use yew::prelude::*;
use crate::models::Video;

#[derive(Properties, PartialEq)]
pub struct VideoDetailsProps {
    pub video: Video
}

#[function_component(VideoDetails)]
pub fn video_details(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
    let url_arr = video.url.split(".be/").collect::<Vec<&str>>();
    let url = url_arr.last().unwrap();
    html! {
        <div>
            <h3><a href={ video.url.clone() }> { video.title.clone() }</a></h3>
            <iframe width={"560"} height={"315"} src={ format!("https://www.youtube.com/embed/{}", url)} ></iframe>
        </div>
    }
}
