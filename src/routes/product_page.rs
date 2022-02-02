use yew::prelude::*;
use crate::components::{
    footer::Footer, navbar::Navbar};
pub struct ProductPage;

impl Component for ProductPage {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>  
            <Navbar/>
            
            <Footer/>
            </>
        }
    }
}