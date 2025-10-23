// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

mod ui;
mod bytearray;

use ui::Ui;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Title { "Piecewise" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Meta { name: "description", content: "Securely share a key through multiple channels." }
        document::Meta { name: "keywords", content: "key,split,share,secure,security,encryption,keysplit,multichannel,key sharing,multichannel key sharing" }
        document::Meta { name: "author", content: "Fabian Boesiger" }
        
        main {
            Ui {}
            {
                // Check if we are compiling for the web target
                if cfg!(target_arch = "wasm32") {
                    rsx! {
                        details { 
                            summary { "Download the App" }
                            p { class: "info", "This app is also avaliable to download for offline use." }
                            div {
                                class: "row",
                                a { class: "button", href: "/releases/macos/Piecewise.app", "Download for macOS" }
                                a { class: "button", href: "/releases/windows/Piecewise.exe", "Download for Windows" }
                                a { class: "button", href: "/releases/linux/Piecewise", "Download for Linux" }
                            }
                        }
                    }
                } else {
                    rsx! {}
                }
            }
        }
    }
}
