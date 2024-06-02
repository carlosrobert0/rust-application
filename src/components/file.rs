use leptos::*;

#[component]
pub fn File(name: String, size: String, status: String) -> impl IntoView {
  let status_class = match status.as_str() {
    "success" => "bg-success",
    "error" => "bg-danger",
    _ => "bg-success",
  };

  let status_image = match status.as_str() {
    "success" => "/success.png",
    "error" => "/error.png",
    _ => "/success.png",
  };

  view! {
    <div class="card bg-transparent border">
      <div class="card-body flexBetween flex-start">
        <div>
          <h5 class="card-title">{name}</h5>
          <p class="card-text">{size}</p>
          <div class="flexBetween">
            <div class="progress">
              <div 
                class={format!("progress-bar {}", status_class)}
                role="progressbar" 
                style="width: 100%;" 
                aria-valuenow="100" 
                aria-valuemin="0" 
                aria-valuemax="100"
              />
            </div>
            r"100%"
          </div>
        </div>
        <img src={format!("{}", status_image)} />
      </div>
    </div>
  }
}