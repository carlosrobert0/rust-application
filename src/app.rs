use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::pages::login::*;
use crate::pages::forgot_password::*;
use crate::pages::register_new_password::*;
use crate::pages::home::*;
use crate::error_template::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
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
        <Title text="Guardianm"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate  outside_errors/>
            }
            .into_view()
        }>
            <main style="background-color: #013763;">
                <Routes>
                    <Route path="" view=Login/>
                    <Route path="/forgot-password" view=ForgotPassword/>
                    <Route path="/register-new-password" view=RegisterNewPassword/>
                    <Route path="/home" view=Home/>
                </Routes>
            </main>
        </Router>
    }
}
