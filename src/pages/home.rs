use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
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
            <article class="welcome-box">
                <h1 class="text-left welcome-text">r"Welcome!"<br/><strong>r"Amber Sanders"</strong></h1>
            </article>
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
