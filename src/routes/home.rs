use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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
    tracks: serde_json::Value,
    formats: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaInfo {
    media: Media,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    track: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    // Can work on these next:
    Format: String,
    FileSize: String,
    Duration: String,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { 
            link,
            json_filename: String::new(),
            tasks: vec![],
            tracks: json!(null),
            formats: json!(null),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {

// gotta move these guys!
        fn number_of_tracks(v: &Vec<MediaInfo>) -> serde_json::Value {
            // could potentially use an array with known length here
            let mut track_numbers = Vec::new();
            for elem in v.iter() {
                track_numbers.push(elem.media.track.len().to_string());
            };

            let mut value_counts : HashMap<String, i32> = HashMap::new();
            for item in track_numbers.iter() {
                *value_counts.entry(String::from(item)).or_insert(0) += 1;
            };

            #[derive(Deserialize)]
            let string_value_counts = json!(value_counts);
            string_value_counts
        }


        fn format_codec_types(v: &Vec<MediaInfo>) -> serde_json::Value {
            // fileData: MediaInfo { media: Media { track: [Track { Format: "Matroska", Duration: "10.000" } ...
            let mut medias = Vec::new();
            for elem in v.iter() {
                medias.push(&elem.media);
            };
            let mut tracks = Vec::new();
            for t in &medias {
                tracks.push(&t.track);
            };
            let mut ttracks = Vec::new();
            for tt in &tracks {
                // First track is General
                ttracks.push(tt[0]["Format"].to_string());
            };

            let mut value_counts : HashMap<String, i32> = HashMap::new();
            for item in ttracks.iter() {
                *value_counts.entry(String::from(item)).or_insert(0) += 1;
            };

            #[derive(Deserialize)]
            let formats = json!(value_counts);
            formats
        }


        match msg {
            Msg::SendJson(value) => {
                self.json_filename = format!("File loading . . .");
                let file = value.first().unwrap().clone();
                let callback = self.link.callback(Msg::FileLoaded);
                let j = ReaderService::default().read_file(file, callback).unwrap();
                self.tasks.push(j);
                true
            }
            Msg::FileLoaded(file) => {
                self.json_filename = format!("File loaded: {:?}", file.name);
                let content = serde_json::from_slice(&*file.content).unwrap_or(vec![]);
                let v: Vec<MediaInfo> = content;
                if v.is_empty() {
                    self.json_filename = "Err, are you sure that is MediaInfo JSON you got there?".to_string()
                } else {
                    // bring the action
                    self.tracks = number_of_tracks(&v);
                    self.formats= format_codec_types(&v);
                };
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
                         } 
                         Msg::SendJson(result)
                       });

        html! {
            <div class="app">
                <header class="app-header">
                    <h1>{"media collection viewer"}</h1>
                        <tagline>{"work in progress! - upload collection export of mediainfo.json and see charts!"}</tagline>
                        <small>{"By @ablwr: "}<a href="https://github.com/ablwr/media-collection-viewer">{"source"}</a></small>
                    </header>
                    <main>
                        <input type="file" 
                            id="jsonImport" 
                            accept="application/JSON" 
                            multiple=false 
                            onchange=callback/>
                        <p>{ &self.json_filename.to_string() }</p>
                        <button class="button" id="jsonStart">{ "Press this button to build charts" }</button>
                        // <p>{ "JSON results:" }
                        // <textarea id="result"></textarea>
                        // </p>
                        <span style="display:none;" id="chart_tracks">{ &self.tracks.to_string() }</span>
                        <span style="display:none;" id="chart_formats">{ &self.formats.to_string() }</span>
                        <div id="all_the_charts">
                            // TODO: Throw this over to the JS in a proper way
                            <div>
                                { "How many tracks are in each file?" }
                                <canvas id="tracks"></canvas>
                            </div>
                            <div>
                                { "What formats are in the collection?" }
                                <canvas id="formats"></canvas>
                            </div>                            
                        </div>
                    </main>
                <footer></footer>
            </div>
        }
    }
}

impl Home {
// fileData: MediaInfo { media: Media { track: [Track { Format: "Matroska", Duration: "10.000" } ...

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
