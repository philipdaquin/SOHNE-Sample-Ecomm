use serde::{Deserialize, Serialize};

/// Designing the State of this component
/// - We need to store a list of products retrieved from server
/// - Store the products the user has added to cart
#[derive(Clone)]
pub struct Product { 
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub price: f64
}
/// AddToCart component: 
/// - Track all products added to cart in a new state called 'cart_products'
/// - Render a 'add_to_cart' button for each product
/// - Add logic to update the 'cart_products' state when 'add_to_cart' button is clicked 
#[derive(Clone)]
pub struct AddToCart  {
    pub product: Product,
    pub quantity: i32
}