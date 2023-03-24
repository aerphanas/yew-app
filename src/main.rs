use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <iframe width="560"
                        height="315"
                        src={format!("{}", video.url)}
                        title={format!("{}", video.title)}
                        frameborder="0">
            </iframe>
        </div>
    }
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
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
            <>
                <p key={video.id} onclick={on_video_select}>
                    {format!("{}: {}", video.speaker, video.title)}
                </p>
            </>
        }   
    }).collect()
}

#[function_component(App)]
fn app() -> Html {
    let url = "https://www.youtube.com/embed/";
    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };
    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });
    let videos = vec![
        Video {
            id: 1,
            title: "Encrypted File System for Applications - Rust Zürisee Feb 2023".to_string(),
            speaker: "Stefan Schindler".to_string(),
            url: format!("{url}Lyv6ybfOSjs")
        },
        Video {
            id: 2,
            title: " OS Development - One Year with Rust - Rust Linz, November 2022 ".to_string(),
            speaker: "Bernhard Kauer".to_string(),
            url: format!("{url}uB9hdaPoUxg")
        },
        Video {
            id: 3,
            title: "Profiling Code in Rust - Rust Linz, December 2022".to_string(),
            speaker: "Vitaly Bragilevsky".to_string(),
            url: format!("{url}JRMOIE_wAFk")
        },
        Video {
            id: 4,
            title: "Nine Rules for Creating Procedural Macros in Rust - Rust Linz, December 2022".to_string(),
            speaker: "Carl Kadie".to_string(),
            url: format!("{url}DMLBBZBlKis")
        },
    ];
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <VideosList videos={videos} on_click={on_video_select.clone()} />
            </div>
            <div>
                { for details }
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}