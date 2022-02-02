use yew::prelude::*;


pub struct Shop;

impl Component for Shop {
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
           
            <main class="l-main">
                <section class="featured section" id="shop">
                <h2 class="section-title">{"All Products"}</h2>
                <div class="featured__container bd-grid">
                    <article class="sneaker">
                        <img src="assets/img/featured2.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Free RN"}</span>
                        <span class="sneaker__preci">{"$149.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/featured3.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Free RN"}</span>
                        <span class="sneaker__preci">{"$149.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/new2.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Sply"}</span>
                        <span class="sneaker__preci">{"$79.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/new3.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Sply"}</span>
                        <span class="sneaker__preci">{"$79.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/new4.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Sply"}</span>
                        <span class="sneaker__preci">{"$79.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/women1.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Womens"}</span>
                        <span class="sneaker__preci">{"$129.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/women2.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Womens"}</span>
                        <span class="sneaker__preci">{"$129.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/women3.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Womens"}</span>
                        <span class="sneaker__preci">{"$129.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/women4.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Womens"}</span>
                        <span class="sneaker__preci">{"$129.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                </div>
                <div class="sneaker__page bd-grid">
                    <div>
                        <span class="sneaker__pag">{"1"}</span>
                        <span class="sneaker__pag">{"2"}</span>
                        <span class="sneaker__pag">{"3"}</span>
                        <span class="sneaker__pag">{"4"}</span>
                        <span class="sneaker__pag"><i class="bx bx-right-arrow-alt button-icon"></i></span>
                    </div>
                </div>
                </section>
            </main>
            </>
        }
    }
}