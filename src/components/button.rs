use leptos::*;

#[component]
pub fn Button(title: String) -> impl IntoView {
  view! { 
    <button
      class="submit-btn"
      type="submit"
    >
      {title}
    </button>
} 
}