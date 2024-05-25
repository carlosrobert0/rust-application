use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/prototype.css"/>

        <Link rel="preload"
            href="https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600;700&display=swap"
            as_="font"
            crossorigin="anonymous"
        />

        <Stylesheet
            id="mdb-css"
            href="https://cdnjs.cloudflare.com/ajax/libs/mdb-ui-kit/7.2.0/mdb.min.css"
        />

        <Stylesheet
            id="font-awesome"
            href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css"
        />

        // sets the document title
        <Title text="Guardianm"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main style="background-color: #013763;">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/forgot-password" view=ForgotPasswordPage/>
                    <Route path="/register-new-password" view=RegisterNewPasswordPage/>WelcomePage
                    <Route path="/welcome" view=WelcomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="container">
            <div class="flexColumn">
                <div>
                    <img
                        class="img-logo"
                        src="/logo.png"
                    />
                </div>
                <form style="width: 323px;">
                    <div class="wrapperInputAndLabel">
                        <label>Email *</label>
                        <input
                            type="text"
                            placeholder="Email"
                        />
                    </div>
                    <div class="wrapperInputAndLabel">
                        <label>Password *</label>
                        <input
                            type="password"
                            placeholder="Password"
                        />
                    </div>

                    <div>
                        <div class="flex">
                            <div
                                class="flex"
                            >
                                <input
                                    type="checkbox"
                                    value="remember"
                                />
                                <label class="link" style="margin-left: 8px">Remember  Me</label>
                            </div>
                            <a
                                href="/forgot-password"
                                class="link"
                            >
                                Forgot Password?
                            </a>
                        </div>
                        <button
                            class="submit-btn"
                            type="submit"
                        >
                        Sign in
                        </button>
                    </div>
                </form>
            </div>
        </div>

        <footer class="footer-mobile">
            r#"Don't have an account?"# <a href='#'>Register</a>
        </footer>
    }
}

#[component]
fn ForgotPasswordPage() -> impl IntoView {
    view! {
        <div class="container">
            <div class="flexColumn">
                <div>
                    <img
                        class="img-logo"
                        src="/logo.png"
                    />
                </div>
                <form style="width: 323px;">
                    <p class="text-information">r"Enter the email associated with your account:"</p>
                    <div class="wrapperInputAndLabel">
                        <label>Email address *</label>
                        <input
                            type="text"
                            placeholder="Email"
                        />
                        <a
                            href="/register-new-password"
                            class="submit-btn d-flex justify-content-center align-items-center"
                        >
                            Recover Password
                        </a>
                    </div>

                </form>
            </div>
        </div>

        <footer class="footer-mobile">
            r#"Don't have an account?"# <a href='#'>Register</a>
        </footer>
    }
}

#[component]
fn RegisterNewPasswordPage() -> impl IntoView {
    view! {
        <div class="container">
            <div class="flexColumn">
                <div>
                    <img
                        class="img-logo"
                        src="/logo.png"
                    />
                </div>
                <form style="width: 323px;">
                    <p class="text-information">r"Register a new password to access the system"</p>
                    <div class="wrapperInputAndLabel">
                        <label>Password *</label>
                        <input
                            type="password"
                            placeholder="Password"
                        />
                    </div>
                    <div class="wrapperInputAndLabel">
                        <label>Confirm your new password *</label>
                        <input
                            type="password"
                            placeholder="Password"
                        />
                    </div>
                    <a
                        href="/"
                        class="submit-btn d-flex justify-content-center align-items-center"
                    >
                        Confirm
                    </a>
                </form>
            </div>
        </div>

        <footer class="footer-mobile">
            r#"Don't have an account?"# <a href='#'>Register</a>
        </footer>
    }
}

#[component]
fn WelcomePage() -> impl IntoView {
    view! {
        <div class="container bg-white">
            <header class="flexBetween" >
                <img
                    class="img-logo-blue"
                    src="/logo-blue.png"
                />
                <img
                    src="/menu.png"
                />
            </header>
            <article class="welcome-box">
                <h1 class="text-left welcome-text">r"Welcome!"<br/><strong>r"Amber Sanders"</strong></h1>
            </article>
        </div>

        <footer class="footer-mobile-clean">
            <a href="/">
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
