use yew::{prelude::*};
use crate::types::Product;
pub struct ProductCard { 
    props: Props

}
#[derive(Properties, Clone)]
pub struct Props { 
    pub product: Product,
    pub on_add_to_cart: Callback<()>
}

impl Component for ProductCard {
    type Message = ();
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { 
            props: _props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // let onclick = self.props.on_add_to_cart.reform(|_| ());
        html! {
            <>

            </>
        }
    }
}