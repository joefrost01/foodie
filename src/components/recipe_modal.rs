use dioxus::prelude::*;
use crate::models::recipe::Recipe;
use crate::generated_assets;

#[derive(Props, Clone, PartialEq)]
pub struct RecipeModalProps {
    pub recipe: Recipe,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn RecipeModal(props: RecipeModalProps) -> Element {
    let recipe = &props.recipe;
    let image_asset = generated_assets::get_recipe_image(&recipe.id);

    rsx! {
        div { 
            class: "modal-overlay",
            onclick: move |_| props.on_close.call(()),
            
            div { 
                class: "modal-content",
                onclick: move |e| e.stop_propagation(), // Prevent closing when clicking inside modal
                
                header { 
                    class: "modal-header",
                    
                    div { 
                        class: "modal-title-section",
                        h1 { class: "modal-recipe-title", "{recipe.name}" }
                        span { class: "modal-recipe-cuisine", "{recipe.cuisine.as_str()}" }
                    }
                    
                    button { 
                        class: "modal-close-button",
                        onclick: move |_| props.on_close.call(()),
                        "✕"
                    }
                }
                
                div { 
                    class: "modal-body",
                    
                    if let Some(img_asset) = image_asset {
                        div { 
                            class: "modal-image-container",
                            img { 
                                class: "modal-recipe-image",
                                src: "{img_asset}",
                                alt: "{recipe.name}"
                            }
                        }
                    }
                    
                    div { 
                        class: "modal-recipe-info",
                        
                        if let Some(description) = &recipe.description {
                            p { class: "modal-description", "{description}" }
                        }
                        
                        div { 
                            class: "modal-meta-grid",
                            
                            if let Some(time) = recipe.total_time() {
                                div { 
                                    class: "meta-item",
                                    span { class: "meta-label", "Total Time" }
                                    span { class: "meta-value", "{time} minutes" }
                                }
                            }
                            
                            if let Some(serves) = recipe.serves {
                                div { 
                                    class: "meta-item",
                                    span { class: "meta-label", "Serves" }
                                    span { class: "meta-value", "{serves} people" }
                                }
                            }
                            
                            if let Some(rating) = recipe.our_rating {
                                div { 
                                    class: "meta-item",
                                    span { class: "meta-label", "Our Rating" }
                                    span { 
                                        class: "meta-value rating-stars",
                                        for i in 1..=5 {
                                            span { 
                                                class: if i <= rating { "star filled" } else { "star" },
                                                "⭐"
                                            }
                                        }
                                    }
                                }
                            }
                            
                            if let Some(spice) = recipe.spice_level {
                                div { 
                                    class: "meta-item",
                                    span { class: "meta-label", "Spice Level" }
                                    span { 
                                        class: "meta-value spice-level",
                                        {(0..spice).map(|_| "🌶️").collect::<String>()}
                                    }
                                }
                            }
                        }
                        
                        if !recipe.our_changes.is_empty() {
                            div { 
                                class: "modal-our-changes",
                                h3 { class: "section-title", "Our Tweaks" }
                                ul { 
                                    class: "changes-list",
                                    for change in &recipe.our_changes {
                                        li { class: "change-item", "{change}" }
                                    }
                                }
                            }
                        }
                        
                        div { 
                            class: "modal-ingredients",
                            h3 { class: "section-title", "Ingredients" }
                            ul { 
                                class: "ingredients-list",
                                for ingredient in &recipe.ingredients {
                                    li { 
                                        class: "ingredient-item",
                                        span { 
                                            class: "ingredient-amount",
                                            if let Some(amount) = &ingredient.amount {
                                                "{amount}"
                                            }
                                        }
                                        span { class: "ingredient-name", "{ingredient.name}" }
                                        if let Some(notes) = &ingredient.notes {
                                            span { class: "ingredient-notes", "({notes})" }
                                        }
                                    }
                                }
                            }
                        }
                        
                        div { 
                            class: "modal-instructions",
                            h3 { class: "section-title", "Instructions" }
                            ol { 
                                class: "instructions-list",
                                for (i, instruction) in recipe.instructions.iter().enumerate() {
                                    li { 
                                        class: "instruction-item",
                                        span { class: "instruction-number", "{i + 1}" }
                                        span { class: "instruction-text", "{instruction}" }
                                    }
                                }
                            }
                        }
                        
                        if let Some(notes) = &recipe.notes {
                            div { 
                                class: "modal-notes",
                                h3 { class: "section-title", "Our Notes" }
                                p { class: "notes-text", "{notes}" }
                            }
                        }
                    }
                }
            }
        }
    }
}