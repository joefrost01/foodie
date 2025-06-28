use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub cuisine: Cuisine,
    pub description: Option<String>,
    pub image: Option<String>,

    // Core recipe data
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
    pub prep_time: Option<u32>, // minutes
    pub cook_time: Option<u32>, // minutes
    pub serves: Option<u8>,

    // Our personal touches
    pub our_changes: Vec<String>,
    pub our_rating: Option<u8>, // 1-5 stars
    pub spice_level: Option<u8>, // 1-5 heat
    pub difficulty: Difficulty,
    pub last_made: Option<DateTime<Utc>>,
    pub notes: Option<String>,

    // Categorization
    pub tags: Vec<String>,
    pub main_ingredient: Option<String>,
    pub dietary_info: Vec<DietaryTag>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ingredient {
    pub name: String,
    pub amount: Option<String>, // "2 tbsp", "1 large", etc.
    pub notes: Option<String>,  // "finely chopped", "room temperature"
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Cuisine {
    Malaysian,
    Italian,
    British,
    Indian,
    Mexican,
    Thai,
    Chinese,
    Mediterranean,
    Fusion,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DietaryTag {
    Vegetarian,
    Vegan,
    GlutenFree,
    DairyFree,
    LowCarb,
    Spicy,
    KidFriendly,
    GreatLeftovers,
    QuickMeal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteProfile {
    pub spice: u8,     // 1-5
    pub saltiness: u8, // 1-5
    pub richness: u8,  // 1-5
}

impl Recipe {
    pub fn new(name: String, cuisine: Cuisine) -> Self {
        let now = Utc::now();
        Self {
            id: format!("recipe-{}", uuid::Uuid::new_v4()),
            name,
            cuisine,
            description: None,
            image: None,
            ingredients: Vec::new(),
            instructions: Vec::new(),
            prep_time: None,
            cook_time: None,
            serves: None,
            our_changes: Vec::new(),
            our_rating: None,
            spice_level: None,
            difficulty: Difficulty::Medium,
            last_made: None,
            notes: None,
            tags: Vec::new(),
            main_ingredient: None,
            dietary_info: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn total_time(&self) -> Option<u32> {
        match (self.prep_time, self.cook_time) {
            (Some(prep), Some(cook)) => Some(prep + cook),
            (Some(prep), None) => Some(prep),
            (None, Some(cook)) => Some(cook),
            (None, None) => None,
        }
    }

    pub fn is_quick_meal(&self) -> bool {
        self.total_time().map_or(false, |time| time <= 30)
    }

    pub fn days_since_last_made(&self) -> Option<i64> {
        self.last_made.map(|last| {
            (Utc::now() - last).num_days()
        })
    }
}

impl Cuisine {
    pub fn as_str(&self) -> &str {
        match self {
            Cuisine::Malaysian => "Malaysian",
            Cuisine::Italian => "Italian",
            Cuisine::British => "British",
            Cuisine::Indian => "Indian",
            Cuisine::Mexican => "Mexican",
            Cuisine::Thai => "Thai",
            Cuisine::Chinese => "Chinese",
            Cuisine::Mediterranean => "Mediterranean",
            Cuisine::Fusion => "Fusion",
            Cuisine::Other(name) => name,
        }
    }
}