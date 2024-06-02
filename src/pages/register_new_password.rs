use crate::components::button::*;
use leptos::*;

#[component]
pub fn RegisterNewPassword() -> impl IntoView {
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
                    <Button title=String::from("Confirm") />
                </form>
            </div>
        </div>

        <footer class="footer-mobile">
            r#"Don't have an account?"# <a href='#'>Register</a>
        </footer>
    }
}
