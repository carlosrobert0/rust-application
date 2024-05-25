use leptos::*;

#[component]
pub fn ForgotPassword() -> impl IntoView {
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
