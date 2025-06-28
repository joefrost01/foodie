# Flavours We Love 🌶️

*A personal recipe collection for dishes that actually work*

## What is this?

Flavours We Love is our personal cookbook app - a place to capture all the recipes we've tested, tweaked, and perfected. No more "was it 1 tsp or 2 tsp of that spice?" or "which version of pad thai did we like?" 

This is for recipes that have earned their place through real cooking, real tweaking, and real enjoyment.

## Why build this?

- **Personal collection**: Our proven winners from kitchens around the world
- **Real modifications**: Documented tweaks that make recipes work for our tastes
- **Always accessible**: Mobile-first design for kitchen use and grocery shopping
- **Fast & responsive**: Built with Rust/Dioxus and WASM for instant search and filtering
- **Easy sharing**: QR codes and exports to share our favorites with friends

## Features

### Core Functionality
- 📱 **Mobile-first design** - perfect for cooking and shopping
- 🔍 **Instant search** - find recipes by name, ingredient, or cuisine
- 🌍 **Multi-cuisine support** - from Malaysian rendang to Glaswegian lasagne
- ⚖️ **Taste adjustment sliders** - customize spice, salt, and richness levels
- 📋 **Smart filtering** - by cuisine, main ingredient, difficulty, or mood
- 📸 **Photo gallery** - our own food photography

### Personal Touch
- ✍️ **Our modifications** - highlighted changes from original recipes
- ⭐ **Personal ratings** - 5-star system based on our actual experience
- 📅 **Last made tracking** - "haven't had this in ages!"
- 💡 **Cooking notes** - "reheats well", "too spicy for kids", etc.
- 🛒 **Shopping lists** - generate from selected recipes

### Recipe Management
- 📝 **Built-in editor** - add new recipes through the app
- 💾 **Local storage** - works offline, syncs when connected
- 📤 **Export/Import** - backup and share recipe collections
- 🔒 **Admin interface** - password-protected recipe management

## Tech Stack

- **Frontend**: Dioxus (Rust) compiled to WebAssembly
- **Styling**: CSS with mobile-first responsive design
- **Storage**: JSON data files + IndexedDB for user additions
- **Deployment**: Static hosting (Netlify/Vercel/GitHub Pages)
- **Performance**: WASM for lightning-fast filtering and search

## Getting Started

### Prerequisites
- Rust (latest stable)
- Dioxus CLI: `cargo install dioxus-cli`

### Development
```bash
# Clone the repo
git clone https://github.com/yourusername/flavours-we-love.git
cd flavours-we-love

# Install dependencies
cargo check

# Run development server
dx serve

# Build for production
dx build --release
```

### Adding Recipes

#### Method 1: JSON Data (for initial recipes)
Edit `src/data/recipes.json` with your perfected recipes:

```json
{
  "id": "our-perfect-rendang",
  "name": "Chicken Rendang with Jasmine Rice",
  "cuisine": "Malaysian",
  "image": "assets/rendang.jpg",
  "our_changes": [
    "Double the coconut milk",
    "Toast spices 30 seconds longer",
    "Add extra lime leaves"
  ],
  "spice_level": 4,
  "our_rating": 5,
  "last_made": "2025-06-20",
  "notes": "Jen's absolute favorite - make extra for leftovers!"
}
```

#### Method 2: Built-in Editor (coming soon)
Use the `/admin` interface to add recipes through the app.

## Sample Recipes

Our starter collection includes:
- Good for you Chilli-con-carne
- Glaswegian Nonna's Lasagne  
- Machu Peckham Chicken
- Chicken satay with spicy noodles
- Meatballs & rigatoni
- Chicken rendang with jasmine rice and cucumber salad
- Dulwich Chicken and Chorizo stew

## Roadmap

- [ ] Core recipe display and filtering
- [ ] Taste adjustment sliders
- [ ] Mobile-responsive design
- [ ] Recipe search functionality
- [ ] Built-in recipe editor
- [ ] Shopping list generation
- [ ] QR code sharing
- [ ] Offline support
- [ ] Recipe import/export
- [ ] Meal planning features

## Contributing

This is our personal recipe collection, but if you want to build your own version:

1. Fork the repo
2. Customize the recipes in `src/data/recipes.json`
3. Replace food photos in `assets/images/`
4. Modify styling to match your taste
5. Deploy to your preferred static host

## License

MIT License - build your own delicious recipe collection!

---

*"Life is too short for bland food and forgotten recipes"*
