use dioxus::prelude::*;
use crate::models::recipe::Recipe;
use crate::generated_assets;

#[derive(Props, Clone, PartialEq)]
pub struct RecipeListProps {
    pub recipes: Vec<Recipe>,
}

#[component]
pub fn RecipeList(props: RecipeListProps) -> Element {
    if props.recipes.is_empty() {
        return rsx! {
            div { 
                class: "empty-state",
                
                div { 
                    class: "empty-icon", "🍽️" 
                }
                h3 { "No recipes found" }
                p { "Try adjusting your search or filters" }
            }
        };
    }

    rsx! {
        div { 
            class: "recipe-grid",
            
            for recipe in props.recipes {
                RecipeCard { recipe: recipe.clone() }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RecipeCardProps {
    pub recipe: Recipe,
}

#[component]
pub fn RecipeCard(props: RecipeCardProps) -> Element {
    let recipe = &props.recipe;

    // Get image asset using auto-generated function
    let image_asset = generated_assets::get_recipe_image(&recipe.id);

    rsx! {
        article { 
            class: "recipe-card",
            
            div { 
                class: "recipe-image",
                if let Some(img_asset) = image_asset {
                    img { 
                        src: "{img_asset}",
                        alt: "{recipe.name}",
                        loading: "lazy"
                    }
                } else {
                    div { 
                        class: "placeholder-image",
                        "🍽️"
                    }
                }
            }
            
            div { 
                class: "recipe-content",
                
                header { 
                    class: "recipe-header",
                    h3 { class: "recipe-title", "{recipe.name}" }
                    span { class: "recipe-cuisine", "{recipe.cuisine.as_str()}" }
                }
                
                if let Some(description) = &recipe.description {
                    p { class: "recipe-description", "{description}" }
                }
                
                div { 
                    class: "recipe-meta",
                    
                    if let Some(time) = recipe.total_time() {
                        span { class: "recipe-time", "🕒 {time}min" }
                    }
                    
                    if let Some(rating) = recipe.our_rating {
                        span { 
                            class: "recipe-rating",
                            for i in 1..=5 {
                                span { 
                                    class: if i <= rating { "star filled" } else { "star" },
                                    "⭐"
                                }
                            }
                        }
                    }
                    
                    if let Some(spice) = recipe.spice_level {
                        if spice >= 3 {
                            span { 
                                class: "spice-indicator", 
                                {(0..spice).map(|_| "🌶️").collect::<String>()}
                            }
                        }
                    }
                }
                
                if !recipe.our_changes.is_empty() {
                    div { 
                        class: "our-changes",
                        span { class: "changes-label", "Our tweaks:" }
                        span { class: "changes-count", "{recipe.our_changes.len()}" }
                    }
                }
            }
        }
    }
}