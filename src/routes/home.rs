// Yew Fetch Services
use crate::api;
use crate::types::{AddToCart, Product};
use anyhow::Error;
use yew::format::Json;
use yew::services::fetch::FetchTask;

use yew::prelude::*;
/// The state object is where you store property vlaues that belongs to that component
/// When the state object changes, the components re-renders 
struct State { 
    products: Vec<Product>,
    cart_products: Vec<AddToCart>,
    fetch_products_error: Option<Error>,
    fetch_products: bool,
}
pub enum Msg { 
    AddToCart(i32),
    FetchProducts,
    FetchProductsOk(Vec<Product>), 
    FetchProductsErr(Error)
}
/// ComponentLinks: Help us create CallBack functions which allows us to change a piece of the state
/// that is part of the parent component. it a Child-to-Parent communication
/// This allows us to register callbacks that can trigger our update lifecycle method 
pub struct Home { 
    state: State,
    _link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();
    //  The Products are queries using the fetch API

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let products = vec![];
        let cart_products = vec![];
        //  Send_Message - Send a message to a component
        _link.send_message(Msg::FetchProducts);
        
        Self { 
            state: State { 
                products,
                cart_products,
                fetch_products_error: None,
                fetch_products: false,
            },
            _link,
            task: None
        }
    }
    ///  The communication between components happens primarly through Messages 
    ///  This allows the component to update itself based on what the based messaged was and rerednered 
    ///  itself
    /// ----------------------
    ///  When the presses the Button Add To Cart, we trigger a Msg::AddToCart message to update
    /// If the product doesnt exist, this product is still added to the cart or it increments the quantity
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        
        match _msg { 
            //  Find the product_id from the product list
            Msg::AddToCart(id) => { 
                let product = self
                    .state
                    .products
                    .iter()
                    .find(|item| item.id==id)
                    .unwrap();
                let cart = self
                    .state
                    .cart_products
                    .iter_mut()
                    .find(|prod| prod.product.id == id);

                if let Some(prod) = cart { 
                    prod.quantity +=1;
                } else { 
                    self.state.cart_products.push(AddToCart { 
                        product: product.clone(),
                        quantity: 1
                    })
                }
                // When we return True,  component is re-rendered.
                true
            },
            Msg::FetchProducts => { 
                self.state.fetch_products = false;
                let handler = self
                    ._link
                    .callback(move |response: api::FetchResponse<Vec<Product>>| { 
                        let (s, Json(data)) = response.into_parts();
                        match data { 
                            Ok(product) => Msg::FetchProductsOk(product),
                            Err(error) => Msg::FetchProductsErr(error),
                        }
                    });
                    self.task = Some(api::get_products(handler));
                true 
            },
            Msg::FetchProductsOk(products) => { 
                self.state.products = products;
                self.state.fetch_products = true;
                true
            },
            Msg::FetchProductsErr(error) => { 
                self.state.fetch_products_error = Some(error);
                self.state.fetch_products = true;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true 
    }

    fn view(&self) -> Html {

        let products: Vec<_> = self.state.products.iter().map(|product| { 
            let id = product.id;
            html! {
                <div>
                    <img src={product.image.clone()}/>
                    <div>{product.name.clone()}</div>
                    <div>{"$"}{product.price}</div>
                    <button onclick=self._link.callback(move |_| Msg::AddToCart(id))>{"Add to Cart"}</button>
                </div>
            }
        }).collect();

        let cart = self
            .state
            .cart_products
            .iter()
            .fold(0.0, |total, cart| total + (cart.quantity as f64 * cart.product.price));
        let quantity = self
            .state
            .cart_products
            .iter()
            .fold(0, |total, cart| {
                cart.quantity + total
            });
        
        
        if !self.state.fetch_products { 
           html! { <div>{"Loading..."}</div>}
        } else if let Some(_) = self.state.fetch_products_error { 
            return html! { 
                <div>
                    <span>{"Error loading products!"}</span>
                </div>
            }
        } else { 
            html! {
                <>
                <section class="home" id="home">
                    <div class="home__container  bd-grid">
                        <div class="home__sneaker">
                            <div class="home__shape"></div>
                            <img src="assets/img/imghome.png" alt="" class="home__img"/>
                        </div>
                        <div class="home__data">
                            <span class="home__new">{"New in"}</span>
                            <h1 class="home__title">{"YEEZY BOOST "}<br/>{" SPLY - 350"}</h1>
                            <p class="home__description">{"Explore the new collections of sneakers"}</p>
                            <a href="#" class="button">{"Explore Now"}</a>
                        </div>                    
                    </div>
                </section>
                <section class="featured section" id="featured">
                    <h2 class="section-title">{"FEATURED"}</h2>
                    <div class="featured__container bd-grid">
                        <article class="sneaker">
                            <div class="sneaker__sale">{"Sale"}</div>
                            <img src="assets/img/featured1.png" alt="" class="sneaker__img"/>
                            <span class="sneaker__name">{"Nike Jordan"}</span>
                            <span class="sneaker__preci">{"$149.99"}</span>
                            <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                        </article>

                        <article class="sneaker">
                            <div class="sneaker__sale">{"Sale"}</div>
                            <img src="assets/img/featured2.png" alt="" class="sneaker__img"/>
                            <span class="sneaker__name">{"Nike Free RN"}</span>
                            <span class="sneaker__preci">{"$149.99"}</span>
                            <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                        </article>

                        <article class="sneaker">
                            <div class="sneaker__sale">{"Sale"}</div>
                            <img src="assets/img/featured3.png" alt="" class="sneaker__img"/>
                            <span class="sneaker__name">{"Nike Free RN"}</span>
                            <span class="sneaker__preci">{"$149.99"}</span>
                            <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                        </article>
                    </div>
                </section>
                <section class="collection section">
                    <div class="collection__container bd-grid">
                        <div class="collection__card">
                            <div class="collection__data">
                                <h3 class="collection__name">{"Nike"}</h3>
                                <p class="collection__description">{"New Collection 2020"}</p>
                                <a href="#" class="button-light">{"Buy Now"}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                            </div>
                            <img src="assets/img/collection1.png" alt="" class="collection__img"/>
                        </div>

                        <div class="collection__card">
                            <div class="collection__data">
                                <h3 class="collection__name">{"Adidas"}</h3>
                                <p class="collection__description">{"New Collection 2020"}</p>
                                <a href="#" class="button-light">{"Buy Now"}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                            </div>
                            <img src="assets/img/collection2.png" alt="" class="collection__img"/>
                        </div>

                        <div class="collection__card">
                            <div class="collection__data">
                                <h3 class="collection__name">{"Name"}</h3>
                                <p class="collection__description">{"New Collection 2020"}</p>
                                <a href="#" class="button-light">{"Buy Now"}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                            </div>
                            <img src="assets/img/imghome.png" alt="" class="collection__img"/>
                        </div>
                    </div>
                </section>
                <section class="women section" id="women">
                    <h2 class="section-title"> {"WOMEN SNEAKERS"}</h2>
                    <div class="women__container bd-grid">
                        <article class="sneaker">
                            <img src="assets/img/women1.png" alt="" class="sneaker__img"/>
                            <span class="sneaker__name">{"Nike Free TN"}</span>
                            <span class="sneaker__preci">{"$129.99"}</span>
                            <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                        </article>

                        <article class="sneaker ">
                            <img src="assets/img/women2.png" alt="" class="sneaker__img"/>
                            <span class="sneaker__name">{"Nike Free TR"}</span>
                            <span class="sneaker__preci">{"$129.99"}</span>
                            <a href="" class="button-light">{"Add to Cart"} <i class="bx bx-right-arrow-alt button-icon"></i></a>
                        </article>

                        <article class="sneaker">
                            <img src="assets/img/women3.png" alt="" class="sneaker__img"/>
                            <span class="sneaker__name">{"Nike GS Pink"}</span>
                            <span class="sneaker__preci">{"$129.99"}</span>
                            <a href="" class="button-light">{"Add to Cart"} <i class="bx bx-right-arrow-alt button-icon"></i></a>
                        </article>

                        <article class="sneaker">
                            <img src="assets/img/women1.png" alt="" class="sneaker__img"/>
                            <span class="sneaker__name">{"Nike Get5"}</span>
                            <span class="sneaker__preci">{"$129.99"}</span>
                            <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                        </article>
                    </div>
                </section>
                <section class="offer section">
                    <div class="offer__container bd-grid">
                        <div class="offer__data">
                            <h3 class="offer__title">{"50%"}</h3>
                            <p class="offer__description">{"in Adidas Superstar sneakers"}</p>
                            <a href="#" class="button">{"Shop Now"}</a>
                        </div>
                        <img src="assets/img/offert.png" alt="" class="offer__img"/>            
                    </div>
                </section>

















                // <div class="home">
                //     <img class="profile-picture" src="assets/img/imghome.png" alt="" />
                //     <h1>{ "Hello, World!" }</h1>
                //     <span>{products}</span>
                //     <span>{format!("Cart Value: {:.2}", cart)}</span>
                //     <span>{format!("Quantity: {}", quantity)}</span>
    
                // </div>
                <script src="assets/js/main.js"></script>
                </>
            }
        }
        
    }
}
