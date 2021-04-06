use yew::prelude::*;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::routes::home::MediaInfo;

#[derive(Properties, Clone)]
pub struct Props {
    pub json_data: Vec<MediaInfo>,
}


pub struct Charts {
    json_data: Vec<MediaInfo>,
}

impl Component for Charts {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Charts {
            json_data: props.json_data,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.json_data = props.json_data;
        true
    }

    fn update(&mut self, props: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div id="all_the_charts">
                <h2>{ "General" }</h2>
                <section class="General">
                    <div>
                        <h3>{ "Tracks per file" }</h3>
                        <canvas id="tracks"></canvas>
                    </div>
                    <div>
                        <h3>{ "Formats" }</h3>
                        <canvas id="formats"></canvas>
                    </div>
                    <div>
                        <h3>{ "File extensions" }</h3>
                        <canvas id="file_extensions"></canvas>
                    </div>
                </section>
                <h2>{ "Video" }</h2>
                <section class="Video">
                    <div>
                        <h3>{ "Color spaces" }</h3>
                        <canvas id="color_spaces"></canvas>
                    </div>
                    <div>
                        <h3>{ "Video codecs" }</h3>
                        <canvas id="video_codecs"></canvas>
                    </div>
                    <div>
                        <h3>{ "Video color depths" }</h3>
                        <canvas id="video_bitdepths"></canvas>
                    </div>    
                    <div>
                        <h3>{ "Chroma subsampling" }</h3>
                        <canvas id="chroma_subsamplings"></canvas>
                    </div>  
                    <div>
                        <h3>{ "Video standard" }</h3>
                        <canvas id="video_standards"></canvas>
                    </div>
                    <div>
                        <h3>{ "Dimensions" }</h3>
                        <canvas id="dimensions"></canvas>
                    </div>        
                </section>
                <h2>{ "Audio" }</h2>
                <section class="Audio">
                    <div>
                        <h3>{ "Audio codecs" }</h3>
                        <canvas id="audio_codecs"></canvas>
                    </div>
                    <div>
                        <h3>{ "Audio bit depths" }</h3>
                        <canvas id="audio_bitdepths"></canvas>
                    </div>
                </section>
                <h2>{ "Superlatives / Experimental 👩‍🔬" }</h2>
                <section class="Superlatives">
                    <div>
                        <h3>{ "Longest / Shortest" }</h3>
                        <span id="chart_longest_shortest">{ Charts::longest_shortest(&self.json_data) }</span>
                    </div>
                </section>
                // just don't look at this...!!!
                <span style="display:none;" id="chart_tracks">{ Charts::number_of_tracks(&self.json_data) }</span>
                <span style="display:none;" id="chart_formats">{ Charts::formats_in_collection(&self.json_data) }</span>
                <span style="display:none;" id="chart_color_spaces">{ Charts::color_spaces_types(&self.json_data) }</span>
                <span style="display:none;" id="chart_dimensions">{ Charts::dimensions_types(&self.json_data) }</span>
                <span style="display:none;" id="chart_audio_codecs">{ Charts::audio_codec_types(&self.json_data) }</span>
                <span style="display:none;" id="chart_audio_bitdepths">{ Charts::audio_bitdepth_types(&self.json_data) }</span>
                <span style="display:none;" id="chart_video_codecs">{ Charts::video_codec_types(&self.json_data) }</span>
                <span style="display:none;" id="chart_video_bitdepths">{ Charts::video_bitdepth_types(&self.json_data) }</span>
                <span style="display:none;" id="chart_video_standards">{ Charts::video_standard_types(&self.json_data) }</span>
                <span style="display:none;" id="chart_chroma_subsamplings">{ Charts::chroma_subsampling_types(&self.json_data) }</span>
                <span style="display:none;" id="chart_file_extensions">{ Charts::file_extensions_types(&self.json_data) }</span>              
            </div>
        }
    }
}



impl Charts {

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
        Charts::count_values(counts)
    }


    fn formats_in_collection(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Charts::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in &tracks {
            // First track is General
            ttracks.push(tt[0]["Format"].to_string());
        };
        Charts::count_values(ttracks)
    }


    fn color_spaces_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Charts::get_to_tracks(&v);
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
        Charts::count_values(ttracks)
    }


    fn dimensions_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Charts::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in tracks.iter() {
            for ttt in tt.iter() {
                if ttt.get("@type").unwrap() == "Video" {
                    ttt.get("Width");
                    if ttt.get("Width") == None {
                        ttracks.push("None".to_string())
                    } else {
                        let w = ttt.get("Width").unwrap().as_str().unwrap();
                        let h = ttt.get("Height").unwrap().as_str().unwrap();
                        let d = [w, h].join(" x ");
                        ttracks.push(d.to_string());
                    }
                }
            }
        };
        Charts::count_values(ttracks)
    }

    fn audio_codec_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Charts::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in tracks.iter() {
            for ttt in tt.iter() {
                if ttt.get("@type").unwrap() == "Audio" {
                    ttt.get("Format");
                    ttracks.push(ttt.get("Format").unwrap().as_str().unwrap().to_string());
                }
            }
        };
        Charts::count_values(ttracks)
    }


    fn video_codec_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Charts::get_to_tracks(&v);
        let mut ttracks = Vec::new();
        for tt in tracks.iter() {
            for ttt in tt.iter() {
                if ttt.get("@type").unwrap() == "Video" {
                    ttt.get("Format");
                    ttracks.push(ttt.get("Format").unwrap().as_str().unwrap().to_string());
                }
            }
        };
        Charts::count_values(ttracks)
    }


    fn audio_bitdepth_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Charts::get_to_tracks(&v);
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
        Charts::count_values(ttracks)
    }


    fn video_bitdepth_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Charts::get_to_tracks(&v);
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
        Charts::count_values(ttracks)
    }

    fn video_standard_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Charts::get_to_tracks(&v);
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
        Charts::count_values(ttracks)
    }


    fn chroma_subsampling_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Charts::get_to_tracks(&v);
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
        Charts::count_values(ttracks)
    }


    fn file_extensions_types(v: &Vec<MediaInfo>) -> serde_json::Value {
        let tracks = Charts::get_to_tracks(&v);
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
        Charts::count_values(ttracks)
    }


    fn longest_shortest(v: &Vec<MediaInfo>) -> String {
        let tracks = Charts::get_to_tracks(&v);
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
        let info: String = format!("Shortest: {:?}, Longest: {:?}", ttracks.first().unwrap_or(&0.0).to_string(), ttracks.last().unwrap_or(&0.0).to_string());
        info
    }


}