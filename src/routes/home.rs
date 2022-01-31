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
    FetchProductsOk, 
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

        html! {
            <>
            <div class="home">
                <img class="profile-picture" src="assets/img/imghome.png" alt="" />
                <h1>{ "Hello, World!" }</h1>
                <span>{products}</span>
                <span>{format!("Cart Value: {:.2}", cart)}</span>
                <span>{format!("Quantity: {}", quantity)}</span>

            </div>
            <script>
                {"
                    /*===== MENU SHOW =====*/ 
                    const showMenu = (toggleId, navId) => { 
                        const toggle = document.getElementById(toggleId),
                        nav = document.getElementById(navId) 
                        if (toggle && nav) { 
                            toggle.addEventListener('click', () => { 
                                nav.classList.toggle('show')
                            })
                        } 
                    }
                    showMenu('nav-toggle', 'nav-menu')
                    
                    /*===== REMOVE MENU =====*/
                    const navLink = document.querySelectorAll('.nav__link'),
                        navMenu = document.getElementById('nav-menu')
                    function linkAction() { 
                        navMenu.classList.remove('show')
                    }
                    navLink.forEach(n => n.addEventListener('click', linkAction))
                    /*===== SCROLL SECTIONS ACTIVE LINK =====*/
                    const sections = document.querySelectorAll('section[id]')
                    window.addEventListener('scroll', scrollActive)
                    
                    function scrollActive() { 
                        const scrollY = window.pageYOffset
                        sections.forEach(current => { 
                            const sectionHeight = current.offsetHeight
                            const sectionTop = current.offsetTop - 50 
                            sectionId = current.getAttribute('id')
                    
                            if(scrollY > sectionTop && scrollY <= sectionTop + sectionHeight) { 
                                document.querySelectory('.nav__menu a[href*=' + sectionId + ']').classList.add('active')
                            } else { 
                                document.querySelectory('.nav__menu a[href*=' + sectionId + ']').classList.remove('active')
                    
                            }
                        })
                    }
                    
                    /*===== CHANGE COLOR HEADER =====*/ 
                    window.onscroll = () => {
                        const nav = document.getElementById('header') 
                        if(this.scrollY >=200) nav.classList.add('scroll-header'); else nav.classList.remove('scroll-header')
                    }
                "}
        
            </script>
            </>
        }
    }
}
