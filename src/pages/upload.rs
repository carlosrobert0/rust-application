use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn Upload() -> impl IntoView {
    let navigate = use_navigate();
    let navigate_to_uploaded = move |_| {
        navigate("/uploaded", Default::default());
    };

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
            <h1 class="text-left land-text">Upload Files</h1>
            <label for="fileInput" class="input-file">
              <input type="file" id="fileInput" />
              <img src="/upload-file.png" />
              <p>r"Select file from your phone."</p>
            </label>
            <button
              on:click=navigate_to_uploaded
              class="upload-btn"
              type="submit"
              data-mdb-ripple-init data-mdb-modal-init data-mdb-target="#uploadedModal"
            >
              Upload
            </button>
            <button
              class="cancel-btn"
              type="submit"
            >
              Cancel
            </button>
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
