use leptos::*;
use crate::components::file::*;
use leptos_router::use_navigate;

#[component]
pub fn Uploaded() -> impl IntoView {
    let navigate = use_navigate();
    let navigate_to_uploads = move |_| {
        navigate("/uploads", Default::default());
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
            <div class="file-list">
                <File 
                    name=String::from("File name.pdf") 
                    size=String::from("123kb") 
                    status=String::from("success")
                />
                <File 
                    name=String::from("File name.pdf") 
                    size=String::from("123kb") 
                    status=String::from("success")
                />
                <File 
                    name=String::from("File name.pdf") 
                    size=String::from("123kb") 
                    status=String::from("error")
                />
                <File 
                    name=String::from("File name.pdf") 
                    size=String::from("123kb") 
                    status=String::from("success")
                />
                <File 
                    name=String::from("File name.pdf") 
                    size=String::from("123kb") 
                    status=String::from("success")
                />
                <File 
                    name=String::from("File name.pdf") 
                    size=String::from("123kb") 
                    status=String::from("error")
                />
                <File 
                    name=String::from("File name.pdf") 
                    size=String::from("123kb") 
                    status=String::from("success")
                />
                <File 
                    name=String::from("File name.pdf") 
                    size=String::from("123kb") 
                    status=String::from("success")
                />
                <File 
                    name=String::from("File name.pdf") 
                    size=String::from("123kb") 
                    status=String::from("error")
                />
            </div>
            
            <div 
              class="modal fade show"
              id="uploadedModal" 
              tabindex="-1" 
              aria-labelledby="exampleModalLabel" 
              aria-hidden="true"
            >
              <div class="modal-dialog">
                <div class="modal-content">
                  <div class="modal-body">
                    <p>The files have been successfully loaded.</p>
                  </div>
                  <div class="modal-footer">
                    <button
                      on:click=navigate_to_uploads
                      class="upload-btn"
                      type="submit"
                      data-mdb-ripple-init data-mdb-modal-init data-mdb-target="#uploadedModal"
                    >
                      Close
                    </button>               
                  </div>
                </div>
              </div>
            </div>
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