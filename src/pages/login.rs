use leptos::*;
use crate::components::button::*;

#[component]
pub fn Login() -> impl IntoView {
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
                        <Button title=String::from("Sign in") />
                    </div>
                </form>
            </div>
        </div>

        <footer class="footer-mobile">
            r#"Don't have an account?"# <a href='#'>Register</a>
        </footer>
    }
}
