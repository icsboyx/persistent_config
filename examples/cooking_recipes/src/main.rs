//! Example: Persistent Cooking Recipes
//!
//! This example demonstrates how to use `persistent_config` to persist a collection
//! of cooking recipes. Each recipe has a name, ingredients, and instructions.
//!
//! Run with: `cargo run --example cooking_recipes`

use persistent_config::prelude::*;
use serde::{Deserialize, Serialize};

/// A single cooking recipe.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<String>,
    pub instructions: String,
}

/// Persistent collection of recipes.
#[derive(Debug, Serialize, Deserialize, Default, Persistent)]
pub struct RecipeBook {
    pub recipes: Vec<Recipe>,
}

fn main() -> anyhow::Result<()> {
    // Initialize book, which will be used to persist recipes.
    let mut book = RecipeBook::default();

    // Register for persistence, using default configuration, with panic_on_error set to false.
    // If file does not exist, or error occurs, it will not panic, defaults will be used.
    // Default save config is TOML format, stored in the current "./config" directory,
    book.config_builder(
        Some("./my_custom_config_dir"),
        Some("recipe_book"),
        SaveFormat::YAML,
        false,
    )?;

    // Load existing recipes from disk if available.
    book.load()?;

    //let see if we have any recipes loaded.
    if book.recipes.is_empty() {
        println!("No recipes found. Starting with an empty recipe book.");
    } else {
        println!("Loaded recipes: {:#?}", book.recipes);
    }

    // Create a sample recipe book.
    book.recipes.push(Recipe {
        name: "Pancakes".to_string(),
        ingredients: vec![
            "2 cups flour".to_string(),
            "2 eggs".to_string(),
            "1.5 cups milk".to_string(),
            "1 tbsp sugar".to_string(),
            "1 tsp baking powder".to_string(),
        ],
        instructions: "Mix all ingredients and cook on a hot griddle.".to_string(),
    });

    book.recipes.push(Recipe {
        name: "Scrambled Eggs".to_string(),
        ingredients: vec![
            "3 eggs".to_string(),
            "1 tbsp butter".to_string(),
            "Salt".to_string(),
            "Pepper".to_string(),
        ],
        instructions: "Beat eggs, melt butter in pan, cook eggs while stirring.".to_string(),
    });

    // Save the recipe book to disk.
    book.save()?;
    println!("Recipe book saved: {:#?}", book);

    // Simulate a new session by resetting the struct.
    let mut loaded_book = RecipeBook::default();
    loaded_book.default_save_config(false)?;

    // Load the recipe book from disk.
    loaded_book.load()?;
    println!("Recipe book loaded: {:#?}", loaded_book);

    Ok(())
}
