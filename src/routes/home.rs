use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use yew::{html, ChangeData, Component, ComponentLink, Html, ShouldRender};
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};

use crate::routes::about::About;
use crate::components::charts::Charts;

pub enum Msg {
    SendJson(Vec<File>),
    FileLoaded(FileData),
}

pub struct Home {
    link: ComponentLink<Self>,
    tasks: Vec<ReaderTask>,
    json_filename: String,
    tracks: serde_json::Value,
    formats: serde_json::Value,
    color_spaces: serde_json::Value,
    dimensions: serde_json::Value,
    audio_codecs: serde_json::Value,
    video_codecs: serde_json::Value,
    audio_bitdepths: serde_json::Value,
    video_bitdepths: serde_json::Value,
    video_standards: serde_json::Value,
    chroma_subsamplings: serde_json::Value,
    file_extensions: serde_json::Value,
    longest_shortest: String,
}

#[derive(Serialize, Deserialize)]
pub struct MediaInfo {
    media: Media,
}

#[derive(Serialize, Deserialize)]
pub struct Media {
    track: Vec<Value>,
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
            color_spaces: json!(null),
            dimensions: json!(null),
            audio_codecs: json!(null),
            video_codecs: json!(null),
            audio_bitdepths: json!(null),
            video_bitdepths: json!(null),
            video_standards: json!(null),
            chroma_subsamplings: json!(null),
            file_extensions: json!(null),
            longest_shortest: String::new(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {

        match msg {
            Msg::SendJson(value) => {
                // log::info!("");
                self.json_filename = format!("File loading . . . please wait . . . ");
                let file = value.first().unwrap().clone();
                let callback = self.link.callback(Msg::FileLoaded);
                let j = ReaderService::default().read_file(file, callback).unwrap();
                self.tasks.push(j);
                true
            }
            Msg::FileLoaded(file) => {        
                let content = serde_json::from_slice(&*file.content).unwrap_or(vec![]);
                let v: Vec<MediaInfo> = content;
                if v.is_empty() {
                    self.json_filename = "Err, are you sure that is valid JSON you got there? Please note that the MediaInfo version must be 18.08-1 or later!".to_string();
                } else {
                    self.json_filename = format!("File loaded: {:?}", file.name);

                    self.tracks = Home::number_of_tracks(&v);
                    self.formats = Home::formats_in_collection(&v);
                    self.color_spaces = Home::color_spaces_types(&v);
                    self.dimensions = Home::dimensions_types(&v);
                    self.audio_codecs = Home::audio_codec_types(&v);
                    self.video_codecs = Home::video_codec_types(&v);
                    self.audio_bitdepths = Home::audio_bitdepth_types(&v);
                    self.video_bitdepths = Home::video_bitdepth_types(&v);
                    self.video_standards = Home::video_standard_types(&v);
                    self.chroma_subsamplings = Home::chroma_subsampling_types(&v);
                    self.file_extensions = Home::file_extensions_types(&v);
                    self.longest_shortest = Home::longest_shortest(&v);
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
                </header>
                <About />
                <main>
                    <input type="file" 
                        id="jsonImport" 
                        accept="application/JSON" 
                        multiple=false 
                        onchange=callback/>
                    <p>{ &self.json_filename.to_string() }</p>
                    <button class="button" id="jsonStart">{ "Build charts" }</button>
                    <span style="display:none;" id="chart_tracks">{ &self.tracks.to_string() }</span>
                    <span style="display:none;" id="chart_formats">{ &self.formats.to_string() }</span>
                    <span style="display:none;" id="chart_color_spaces">{ &self.color_spaces.to_string() }</span>
                    <span style="display:none;" id="chart_dimensions">{ &self.dimensions.to_string() }</span>
                    <span style="display:none;" id="chart_audio_codecs">{ &self.audio_codecs.to_string() }</span>
                    <span style="display:none;" id="chart_audio_bitdepths">{ &self.audio_bitdepths.to_string() }</span>
                    <span style="display:none;" id="chart_video_codecs">{ &self.video_codecs.to_string() }</span>
                    <span style="display:none;" id="chart_video_bitdepths">{ &self.video_bitdepths.to_string() }</span>
                    <span style="display:none;" id="chart_video_standards">{ &self.video_standards.to_string() }</span>
                    <span style="display:none;" id="chart_chroma_subsamplings">{ &self.chroma_subsamplings.to_string() }</span>
                    <span style="display:none;" id="chart_file_extensions">{ &self.file_extensions.to_string() }</span>                    
                    <div id="all_the_charts">
                        <Charts/>
                        <h2>{ "Superlatives / Experimental 👩‍🔬" }</h2>
                        <section class="Superlatives">
                            <div>
                                <h3>{ "Longest / Shortest" }</h3>
                                <span id="chart_longest_shortest">{ &self.longest_shortest }</span>
                            </div>
                        </section>
                        
                    </div>
                </main>
                <footer>{"By @ablwr: "}<a href="https://github.com/ablwr/media-collection-viewer">{"source"}</a></footer>
            </div>
        }
    }
}


impl Home {

// Functions for abstraction of repeat tasks

    fn count_values(counts: Vec<String>) -> serde_json::Value {
        let mut value_counts : HashMap<String, i32> = HashMap::new();
        for item in counts.iter() {
            *value_counts.entry(String::from(item)).or_insert(0) += 1;
        };
        json!(value_counts)
    }

    fn get_to_tracks(v: &Vec<MediaInfo>) -> Vec<&Vec<serde_json::Value>> {
        let mut medias = Vec::new();
        for elem in v.iter() {
            medias.push(&elem.media);
        };
        let mut tracks = Vec::new();
        for t in &medias {
            tracks.push(&t.track);
        }; 
        tracks.clone() 
    }

// Functions getting JSONs

    fn number_of_tracks(v: &Vec<MediaInfo>) -> serde_json::Value {
        // could potentially use an array with known length here
        let mut counts = Vec::new();
        for elem in v.iter() {
            counts.push(elem.media.track.len().to_string());
        };
        Home::count_values(counts)
    }


    fn formats_in_collection(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Home::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in &tracks {
            // First track is General
            ttracks.push(tt[0]["Format"].to_string());
        };
        Home::count_values(ttracks)
    }


    fn color_spaces_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Home::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in tracks.iter() {
            for ttt in tt.iter() {
                if ttt.get("@type").unwrap() == "Video" {
                    ttt.get("ColorSpace");
                    if ttt.get("ColorSpace") == None {
                        ttracks.push("None".to_string())
                    } else {
                        ttracks.push(ttt.get("ColorSpace").unwrap().as_str().unwrap().to_string());

                    }
                }
            }
        };
        Home::count_values(ttracks)
    }


    fn dimensions_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Home::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in tracks.iter() {
            for ttt in tt.iter() {
                if ttt.get("@type").unwrap() == "Video" {
                    ttt.get("Width");
                    if ttt.get("Width") == None {
                        ttracks.push("None".to_string())
                    } else {
                        let mut w = ttt.get("Width").unwrap().as_str().unwrap();
                        let mut h = ttt.get("Height").unwrap().as_str().unwrap();
                        let mut d = [w, h].join(" x ");
                        ttracks.push(d.to_string());
                    }
                }
            }
        };
        Home::count_values(ttracks)
    }

    fn audio_codec_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Home::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in tracks.iter() {
            for ttt in tt.iter() {
                if ttt.get("@type").unwrap() == "Audio" {
                    ttt.get("Format");
                    ttracks.push(ttt.get("Format").unwrap().as_str().unwrap().to_string());
                }
            }
        };
        Home::count_values(ttracks)
    }


    fn video_codec_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Home::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in tracks.iter() {
            for ttt in tt.iter() {
                if ttt.get("@type").unwrap() == "Video" {
                    ttt.get("Format");
                    ttracks.push(ttt.get("Format").unwrap().as_str().unwrap().to_string());
                }
            }
        };
        Home::count_values(ttracks)
    }


    fn audio_bitdepth_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Home::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in tracks.iter() {
            for ttt in tt.iter() {
                if ttt.get("@type").unwrap() == "Audio" {
                    ttt.get("BitDepth");
                    if ttt.get("BitDepth") == None {
                        ttracks.push("None".to_string())
                    } else {
                        ttracks.push(ttt.get("BitDepth").unwrap().as_str().unwrap().to_string());

                    }
                }
            }
        };
        Home::count_values(ttracks)
    }


    fn video_bitdepth_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Home::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in tracks.iter() {
            for ttt in tt.iter() {
                if ttt.get("@type").unwrap() == "Video" {
                    ttt.get("BitDepth");
                    if ttt.get("BitDepth") == None {
                        ttracks.push("None".to_string())
                    } else {
                        ttracks.push(ttt.get("BitDepth").unwrap().as_str().unwrap().to_string());

                    }
                }
            }
        };
        Home::count_values(ttracks)
    }

    fn video_standard_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Home::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in tracks.iter() {
            for ttt in tt.iter() {
                if ttt.get("@type").unwrap() == "Video" {
                    ttt.get("Standard");
                    if ttt.get("Standard") == None {
                        ttracks.push("None".to_string())
                    } else {
                        ttracks.push(ttt.get("Standard").unwrap().as_str().unwrap().to_string());

                    }
                }
            }
        };
        Home::count_values(ttracks)
    }


    fn chroma_subsampling_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Home::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in tracks.iter() {
            for ttt in tt.iter() {
                if ttt.get("@type").unwrap() == "Video" {
                    ttt.get("ChromaSubsampling");
                    if ttt.get("ChromaSubsampling") == None {
                        ttracks.push("None".to_string())
                    } else {
                        ttracks.push(ttt.get("ChromaSubsampling").unwrap().as_str().unwrap().to_string());

                    }
                }
            }
        };
        Home::count_values(ttracks)
    }


    fn file_extensions_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Home::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in tracks.iter() {
            for ttt in tt.iter() {
                if ttt.get("@type").unwrap() == "General" {
                    ttt.get("FileExtension");
                    if ttt.get("FileExtension") == None {
                        ttracks.push("None".to_string())
                    } else {
                        ttracks.push(ttt.get("FileExtension").unwrap().as_str().unwrap().to_string());

                    }
                }
            }
        };
        Home::count_values(ttracks)
    }


    fn longest_shortest(v: &Vec<MediaInfo>) -> String {
        let tracks = Home::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in tracks.iter() {
            for ttt in tt.iter() {
                if ttt.get("@type").unwrap() == "General" {
                    if ttt.get("Duration") != None {
                        let s: f64 = ttt.get("Duration").unwrap().as_str().unwrap().parse().unwrap();
                        ttracks.push(s);
                    }
                }
            }
        };
        ttracks.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let info: String = format!("Shortest: {:?}, Longest: {:?}", ttracks.first().unwrap().to_string(), ttracks.last().unwrap().to_string());
        info
    }


}

fn main() {
    yew::start_app::<Home>();
}
