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
        html! {
            <>
            <header class="navbar">
                <nav class="nav bd-grid">
                    <div class="nav__toggle" id="nav-toggle">
                        <i class="bx bxs-grid"></i>
                    </div>
                    <a href="#" class="nav__logo">{"SOHNE"}</a>
                </nav>
                <div class="nav__menu" id="nav-menu">
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
            </header>
            </>
        }
    }
}

