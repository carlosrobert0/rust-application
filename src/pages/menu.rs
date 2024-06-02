use leptos::*;

#[component]
pub fn Menu() -> impl IntoView {
    view! {
        <div class="container bg-white">
            <header class="flexBetween">
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
            <div class="overlay">
                <nav>
                    <ul>
                        <li>
                            <a href="/home">Dashboard</a>
                        </li>
                        <li>
                            <a href="/operators">Operators</a>
                        </li>
                        <li class="flexBetween">
                            <a href="/uploads">Files</a>
                            <img
                                src="/arrow-down.png"
                            />
                        </li>
                        <li class="margin-left-20">
                            <a href="/land">Land</a>
                        </li>
                        <li class="margin-left-20">
                            <a href="/revenue">Revenue</a>
                        </li>
                        <li class="margin-left-20">
                            <a href="/upload">Upload</a>
                        </li>
                    </ul>
                </nav>
            </div>
        </div>
    }
}
