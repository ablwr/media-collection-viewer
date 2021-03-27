use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use yew::{html, ChangeData, Component, ComponentLink, Html, ShouldRender};
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(inline_js = "export function sendJson(a) { return a; }")]
extern "C" {
    fn sendJson(a: String) -> String;
}

pub enum Msg {
    SendJson(Vec<File>),
    FileLoaded(FileData),
}

#[wasm_bindgen]
pub struct Home {
    link: ComponentLink<Self>,
    tasks: Vec<ReaderTask>,
    json_filename: String,
    chart_tracks: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct MediaInfo {
    media: Media,
}

#[derive(Debug, Serialize, Deserialize)]
struct Media {
    track: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Track {
    // Can work on these next:
    // Format: String,
    // Duration: String,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { 
            link,
            json_filename: String::new(),
            tasks: vec![],
            chart_tracks: json!(null),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {


        fn number_of_tracks(v: Vec<MediaInfo>) -> serde_json::Value {

            let mut chart_tracks = json!({ "labels": [], "data": []});
            // could potentially use an array with known length here
            let mut track_numbers = Vec::new();
            for elem in v.iter() {
                track_numbers.push(elem.media.track.len().to_string());
            }

            let mut value_counts : HashMap<String, i32> = HashMap::new();
            for item in track_numbers.iter() {
                *value_counts.entry(String::from(item)).or_insert(0) += 1;
            };

            #[derive(Deserialize)]
            let string_value_counts = json!(value_counts);
            string_value_counts
        }

        match msg {
            // TODO handle error for malformed mediainfo
            Msg::SendJson(value) => {
                let file = value.first().unwrap().clone();
                let callback = self.link.callback(Msg::FileLoaded);
                let j = ReaderService::default().read_file(file, callback).unwrap();
                log::info!("{:?}", &j);
                self.tasks.push(j);
                true
            }
            Msg::FileLoaded(file) => {
                self.json_filename = file.name;
                let v: Vec<MediaInfo> = serde_json::from_reader(&*file.content).unwrap();
                self.chart_tracks = number_of_tracks(v);
                // log::info!("{:?}", &self.chart_tracks);
                true
            }
        }
    }

    fn view(&self) -> Html {

        let callback = self.link.callback(move |value| {
                        let mut result = Vec::new();
                        if let ChangeData::Files(files) = value {
                            let files = js_sys::try_iter(&files)
                                .unwrap().unwrap()
                                .map(|v| File::from(v.unwrap()));
                            result.extend(files);
                        } Msg::SendJson(result)});

        html! {
            <div class="app">
                <header class="app-header">
                    <h1>{"media collection viewer"}</h1>
                    <tagline>{"work in progress! - upload collection export of mediainfo.json and see charts!"}</tagline>
                    <small>{"By @ablwr: "}<a href="https://github.com/ablwr/media-collection-viewer">{"source"}</a></small>
                    <input type="file" 
                        id="jsonImport" 
                        accept="application/JSON" 
                        multiple=false 
                        onchange=callback/>
                    <p>{ "Filename appears here when loaded: " }{ &self.json_filename.to_string() }</p>
                    <button id="jsonStart">{ "Press this button for charts" }</button>
                    // <p>{ "JSON results:" }
                    // <textarea id="result"></textarea>
                    // </p>
                    <div style="max-width:400px">
                        { "How many tracks are in each file?" }
                        // TODO: Throw this over to the JS in a proper way
                        <span style="display:none;" id="chart_tracks">{ &self.chart_tracks.to_string() }</span>
                        <canvas id="myChart" width="400" height="400"></canvas>
                    </div>
                </header>
                <footer></footer>
            </div>
        }
    }
}

impl Home {
// fileData: MediaInfo { media: Media { track: [Track { Format: "Matroska", Duration: "10.000" } ...

    // fn format_codec_types() {
    //   println!("what kinds of formats?");
    //           for t in elem.media.track.iter() {
    //               log::info!("fileData: {:?}", t.Format);
    //           }
    // }

    // fn audio_codec_types() {
    //   println!("what kinds of audio codecs?");
    // }

    // fn video_codec_types() {
    //   println!("what kinds of video codecs?");
    // }

    // fn dimensions() {
    //   println!("What are the dimensions (paired width+height)?");
    // }

    // fn color_spaces() {
    //   println!("What are the color spaces?");
    // }

    // fn durations() {
    //  // Need to loop through every Media/General
    //  // then establish a range for durations 
    //  // then establish any outliers
    //   println!("what kind of durations?");
    // }
}

fn main() {
    yew::start_app::<Home>();
}
