use serde::{Deserialize, Serialize};
use serde_json::{Value};
use yew::{html, ChangeData, Component, ComponentLink, Html, ShouldRender};
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};

use crate::routes::about::About;
use crate::components::charts::Charts;


pub enum Msg {
    SendJson(Vec<File>),
    FileLoaded(FileData),
    PopulateChartData
}

pub struct Home {
    link: ComponentLink<Self>,
    tasks: Vec<ReaderTask>,
    json_filename: String,
    json_data: Vec<MediaInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MediaInfo {
    pub media: Media,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Media {
    pub track: Vec<Value>,
}


impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { 
            link,
            json_filename: String::new(),
            json_data: vec![],
            tasks: vec![],
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
                    self.json_data = v.clone();

                    Home::update(self, Msg::PopulateChartData);
                };

                true
            }
            Msg::PopulateChartData => {
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
                    <Charts json_data=&self.json_data />
                </main>
                <footer>{"By @ablwr: "}<a href="https://github.com/ablwr/media-collection-viewer">{"source"}</a></footer>
            </div>
        }
    }
}



fn main() {
    yew::start_app::<Home>();
}
