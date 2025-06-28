use crate::models::recipe::Recipe;
use crate::generated_assets;

// Load all recipes automatically discovered by the build script
pub async fn load_recipes() -> Result<Vec<Recipe>, String> {
    web_sys::console::log_1(&"Loading auto-discovered recipes...".into());

    let recipes_data = generated_assets::get_all_recipe_includes();
    let mut recipes = Vec::new();

    for (i, recipe_json) in recipes_data.iter().enumerate() {
        match serde_json::from_str::<Recipe>(recipe_json) {
            Ok(recipe) => {
                web_sys::console::log_1(&format!("✅ Loaded recipe: {}", recipe.name).into());
                recipes.push(recipe);
            }
            Err(e) => {
                web_sys::console::log_1(&format!("❌ Failed to parse recipe {}: {}", i, e).into());
            }
        }
    }

    web_sys::console::log_1(&format!("Successfully loaded {} total recipes", recipes.len()).into());
    Ok(recipes)
}