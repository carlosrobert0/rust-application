use leptos::*;
use crate::components::file::*;

#[component]
pub fn Uploads() -> impl IntoView {
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