use yew::prelude::*;

/// Charts page
pub struct Charts;

impl Component for Charts {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Charts {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
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
            </>
        }
    }
}


