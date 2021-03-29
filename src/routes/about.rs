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
            <div class="app">
                <header class="app-header">
                    <h1>{"media collection viewer"}</h1>
                </header>
                <main id="about">
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
                    <p>
                        { "Upload a mediainfo.json. You can make one by running something like this: " }
                        <code>{ "mediainfo path/to/files/ --Output=JSON > mediainfo.json" }</code>
                        { " After confirmation of successful loading, press the button to build charts." }
                    </p>
                </div>
                <div>
                    <h2>{ "What else?" }</h2>
                    <p>
                        { "This is an unrefactored demo/proof-of-concept, expect bugs! 🐛 and please report any 
                        unexpected behavior. For example, it's better to refresh the page before loading another 
                        file because things get a little weird in ways I haven't cleaned up yet."}
                    </p><p>
                        {"For technical details and forthcoming features, check the " }
                        <a href="https://github.com/ablwr/media-collection-viewer">{"README"}</a>{"."}
                    </p>
                </div>
                    
                </main>
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
