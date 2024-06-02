use leptos::*;
use crate::components::accordion::*;

#[component]
pub fn Revenue() -> impl IntoView {
    view! {
            <div class="container bg-white">
                <header class="flexBetween" >
                  <img
                    class="img-logo-blue"
                    src="/logo-blue.png"
                  />
                  <a href="/menu">
                    <img
                      src="/menu.png"
                    />
                  </a>
                </header>
                <section class="container-land">
                  <div class="flexBetween input-search">
                    <input type="text" style="background: transparent; border: none;" placeholder="Search"/>
                    <i class="fa fa-search trailing"></i>
                  </div>
                  <h1 class="text-left land-text">r"Revenue"</h1>
                  <div class="scroll-accordion">
                    <Accordion title=String::from("2024 Revenue") />
                    <Accordion title=String::from("2024 Revenue") />
                    <Accordion title=String::from("2024 Revenue") />
                  </div>
                </section>
              </div>

            <footer class="footer-mobile-clean">
                <a href="/home">
                    <img src="/home.png" />
                </a>
                <a href="/upload">
                    <img src="/upload.png" />
                </a>
                <a href="/">
                    <img src="/logout.png" />
                </a>
            </footer>
        }
}
