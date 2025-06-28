use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    pub title: String,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    rsx! {
        header { 
            class: "app-header",
            
            div { 
                class: "header-content",
                h1 { class: "app-title", "{props.title}" }
                
                // Future: Add menu button for mobile
                button { 
                    class: "menu-button",
                    "☰"
                }
            }
        }
    }
}