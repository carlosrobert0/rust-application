use leptos::*;

#[component]
pub fn Operators() -> impl IntoView {
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
            <h1 class="text-left land-text">Operators</h1>
            <article class="operators-box">
                <table>
                  <thead>
                  <tr>
                    <th>Name</th>
                    <th>Number</th>
                  </tr>
                  </thead>
                  <tbody>
                    <tr>
                      <td>John Doe</td>
                      <td>1234567890</td>
                    </tr>
                    <tr>
                      <td>John Doe</td>
                      <td>1234567890</td>
                    </tr>
                    <tr>
                      <td>John Doe</td>
                      <td>1234567890</td>
                    </tr>
                    <tr>
                      <td>John Doe</td>
                      <td>1234567890</td>
                    </tr>
                    <tr>
                      <td>John Doe</td>
                      <td>1234567890</td>
                    </tr>
                    <tr>
                      <td>John Doe</td>
                      <td>1234567890</td>
                    </tr>
                    <tr>
                      <td>John Doe</td>
                      <td>1234567890</td>
                    </tr>
                    <tr>
                      <td>John Doe</td>
                      <td>1234567890</td>
                    </tr>
                    <tr>
                      <td>John Doe</td>
                      <td>1234567890</td>
                    </tr>
                  </tbody>
                </table>
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