use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchBarProps {
    pub query: Signal<String>,
    pub on_query_change: EventHandler<String>,
}

#[component]
pub fn SearchBar(props: SearchBarProps) -> Element {
    rsx! {
        div { 
            class: "search-container",
            
            div { 
                class: "search-bar",
                
                input {
                    class: "search-input",
                    r#type: "text",
                    placeholder: "Search recipes, ingredients...",
                    value: "{props.query.read()}",
                    oninput: move |evt| {
                        props.on_query_change.call(evt.value());
                    }
                }
                
                button { 
                    class: "search-button",
                    "🔍"
                }
            }
        }
    }
}