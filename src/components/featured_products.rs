use yew::{prelude::*};
use crate::types::Product;
pub struct Featured { 
    props: Props

}
#[derive(Properties, Clone)]
pub struct Props { 
    pub product: Product,
    pub on_add_to_cart: Callback<()>
}

impl Component for Featured {
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
        let onclick = self.props.on_add_to_cart.reform(|_| ());
        let product_image = &self.props.product.image;
        let name = &self.props.product.name;
        let price = &self.props.product.price;
        html! {
            <>
            <article class="sneaker">
                <div class="sneaker__sale">{"Sale"}</div>
                <img src={product_image.clone()} class="sneaker__img"/>
                <span class="sneaker__name">{name}</span>
                <span class="sneaker__preci">{"$"}{price}</span>
                <button onclick=onclick class="button-light">{"Add To Cart"}<i class="bx bx-right-arrow-alt button-icon"></i></button>
            </article>
            </>
        }
    }
}