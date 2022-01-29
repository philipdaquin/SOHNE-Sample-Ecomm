pub mod about;
pub mod home;
pub mod product_page;
pub mod feature;
pub mod shop;
pub mod men;
pub mod women;
pub mod new;
use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum AppRoute { 
    #[to = "/shop"]
    Shop,

    #[to = "/new"]
    New, 
    
    #[to = "/women"]
    Women, 
   
    #[to = "/men"]
    Men,
   
    #[to = "/feature"]
    Feature,
   
    #[to = "/about"]
    About,
    
    #[to = "/"]
    Home,
   
}