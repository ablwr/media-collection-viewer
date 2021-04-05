use yew::prelude::*;

/// About page
pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        About {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
                <div id="about">
                <div>
                    <h2>{ "What is this?" }</h2>
                    <p>
                        { "This site generates charts that provide you with a high-level overview of your audio
                           and/or video collections. Because all of the rendering happens on the client side (your
                           browser), your data is not sent anywhere or collected." }
                    </p>
                </div>
                <div>
                    <h2>{ "How do I use this?" }</h2>
                    <p>{ "Upload a mediainfo.json. You can make one by running something like this: " }</p>
                    <code>{ "mediainfo path/to/files/ --Output=JSON > mediainfo.json" }</code>
                    <p>{ "You can also download the "}
                      <a href="https://mediaarea.net/en/MediaInfo">{"MediaInfo GUI"}</a>
                      {", select JSON from View, and save that data as a file."}</p>
                    <p>{ " After loaded, press the button to build charts." }</p>
                </div>
                <div>
                    <h2>{ "What else?" }</h2>
                    <p>{ " Note: You must use MediaInfo version 18.08-1 or later to get valid JSON." }</p>
                    <p>
                        { "This is still experimental, expect bugs and changes! 🐛 and please report any 
                        unexpected behavior."}
                    </p>
                    <p>
                        {"For technical details and forthcoming features, check the " }
                        <a href="https://github.com/ablwr/media-collection-viewer">{"README"}</a>{"."}
                    </p>
                </div>  
                </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    use super::About;
    use yew::App;

    #[wasm_bindgen_test]
    fn about_page_has_an_app_link() {
        let app: App<About> = yew::App::new();
        app.mount(yew::utils::document().get_element_by_id("output").unwrap());

        let app_links = yew::utils::document().get_elements_by_class_name("app-link");

        assert_eq!(app_links.length(), 1);

        let link = app_links.item(0).expect("No app-link").inner_html();
        assert_eq!(link, "Create Yew App");
    }
}
