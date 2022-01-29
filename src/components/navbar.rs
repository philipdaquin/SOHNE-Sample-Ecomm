use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{AppRoute};

pub struct Navbar { 
    link: ComponentLink<Self>,
    is_active: bool,
}
pub enum Msg { 
    Toggle
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { 
            link: _link,
            is_active: false,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg { 
            Msg::Toggle => { 
                self.is_active = !self.is_active;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let show_menu = if !self.is_active {"nav__menu show"} else {"nav__menu"};
        
        html! {
            <>
            <header class="l-header" id="header">
                <nav class="nav bd-grid">
                <div class="nav__toggle" id="nav-toggle">
                    <i class="bx bxs-grid"></i>
                </div>
                <a href="#" class="nav__logo">{"SOHNE"}</a>
                    
                <div class={show_menu} id="nav-menu">
                    <ul class="nav__list">
                        <li class="nav__item"><RouterAnchor<AppRoute> route=AppRoute::Home classes="nav__link"><a>{"Home"}</a></RouterAnchor<AppRoute>></li>
                        <li class="nav__item"> <RouterAnchor<AppRoute> route=AppRoute::About><a>{"About"}</a></RouterAnchor<AppRoute>></li>
                        <li class="nav__item"> <RouterAnchor<AppRoute> route=AppRoute::Feature><a>{"Feature"}</a></RouterAnchor<AppRoute>></li>
                        <li class="nav__item"> <RouterAnchor<AppRoute> route=AppRoute::Men><a>{"Men"}</a></RouterAnchor<AppRoute>></li>
                        <li class="nav__item"> <RouterAnchor<AppRoute> route=AppRoute::Women><a>{"Women"}</a></RouterAnchor<AppRoute>></li>
                        <li class="nav__item"> <RouterAnchor<AppRoute> route=AppRoute::New><a>{"New"}</a></RouterAnchor<AppRoute>></li>
                        <li class="nav__item"> <RouterAnchor<AppRoute> route=AppRoute::Shop><a>{"Shop"}</a></RouterAnchor<AppRoute>></li>
                    </ul>
                </div>
                <div class="nav__shop">
                    <i class="bx bx-shopping-bag"></i>
                </div>
                </nav>
            </header>
            </>
        }
    }
}

