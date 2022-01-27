pub mod about;
pub mod home;
pub mod product_page;

use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum AppRoute { 
    #[to = "/about"]
    About,
    #[to = "/"]
    Home,
   
}