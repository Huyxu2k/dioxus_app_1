use dioxus::prelude::*;

use ui::App as UiApp;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");
fn main() {
    // dioxus::launch(App);
    dioxus::launch(|| {
        use_init_radio_station::<Data, DataChannel>(Data::default);
        let mut radio = use_radio::<Data, DataChannel>(DataChannel::ListCreation);

        let onclick = move |_| {
            radio.write().lists.push(Vec::default());
        };

        println!("Running DataChannel::ListCreation");

        rsx!(
            button { onclick, "Add new list" }
            for (list_n , _) in radio.read().lists.iter().enumerate() {
                ListComp { key: "{list_n}", list_n }
            }
        )
    });
}

<<<<<<< HEAD
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: FAVICON }

        UiApp {}
    }
=======
// #[component]
// fn App() -> Element {
//     // Build cool things ✌️

//     rsx! {
//         // Global app resources
//         document::Link { rel: "icon", href: FAVICON }
//         document::Link { rel: "stylesheet", href: MAIN_CSS }

//         Router::<Route> {}
//     }
// }

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}

///
/// TEST
/// 

use dioxus_radio::prelude::*;

#[derive(Default)]
struct Data {
    pub lists: Vec<Vec<String>>,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum DataChannel {
    ListCreation,
    SpecificListItemUpdate(usize),
}

impl RadioChannel<Data> for DataChannel {}

#[allow(non_snake_case)]
#[component]
fn ListComp(list_n: usize) -> Element {
    let mut radio = use_radio::<Data, DataChannel>(DataChannel::SpecificListItemUpdate(list_n));

    println!("Running DataChannel::SpecificListItemUpdate({list_n})");

    rsx!(
        div {
            button { onclick: move |_| radio.write().lists[list_n].push("Hello, World".to_string()),
                "New Item"
            }
            ul {
                for (i , item) in radio.read().lists[list_n].iter().enumerate() {
                    li { key: "{i}", "{item}" }
                }
            }
        }
    )
>>>>>>> 4ed44a90b5b9a863a3fbca4f17d4406c6448f4f6
}