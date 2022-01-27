use yew::prelude::*;

/// Designing the State of this component
/// - We need to store a list of products retrieved from server
/// - Store the products the user has added to cart
pub struct Product { 
    id: i32,
    name: String,
    description: String,
    image: String,
    price: f64
}

struct State { 
    products: Vec<Product>
}

pub struct Home { 
    state: State
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let products: Vec<Product> = vec![
            Product { 
                id: 1,
                name: "Nike Jordan".to_string(),
                description: "Nike Jordan".to_string(),
                image: "/assets/img/featured1.png".to_string(),
                price: 149.99
            }

        ];
        
        Self { 
            state: State { 
                products
            }
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true 
    }

    fn view(&self) -> Html {

        let products: Vec<_> = self.state.products.iter().map(|product| { 
            html! {
                <div>
                    <img src={product.image.clone()}/>
                    <div>{product.name.clone()}</div>
                </div>
            }
        }).collect();



        html! {
            <div class="home">
                <img class="profile-picture" src="assets/images/avatar.jpg" alt="ShironCat's avatar" />
                <h1>{ "Hello, World!" }</h1>
                <span>{products}</span>
            </div>
        }
    }
}
