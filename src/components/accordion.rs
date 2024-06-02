use leptos::*;

#[component]
pub fn Accordion(title: String) -> impl IntoView {
  let (_show_accordion, set_show_accordion) = create_signal(String::new());
  let (_show_accordion_style, set_show_accordion_style) = create_signal(String::from("bg-blue-950 text-white"));

  let toggle_accordion = move |_| {
    set_show_accordion.update(|show| {
        if show.is_empty() {
            show.push_str("show");
        } else {
            show.clear();
        }
    });

    set_show_accordion_style.update(|show| {
        if show.is_empty() {
            show.push_str("bg-blue-950 text-white");
        } else {
            if show.contains("bg-blue-950 text-white") {
              show.clear();
              show.push_str("bg-neutral-100 text-blue-950");
            } else {
              if show.contains("bg-neutral-100 text-blue-950") {
                show.clear();
                show.push_str("bg-blue-950 text-white");
              }
            }
        }
    });
  };

  view! {
    <div class="accordion" id="accordionPanelsStayOpenExample">
      <div class="accordion-item background-accordion-item">
        <h2 class="accordion-header" id="headingAccordionItem">
          <button
            data-mdb-collapse-init
            class=move || format!("accordion-button collapsed background-accordion-land {}", _show_accordion_style.get())
            type="button"
            data-mdb-toggle="collapse"
            data-mdb-target="#panelsStayOpen-collapseTwo"
            aria-expanded="false"
            on:click={toggle_accordion}
          >
            {title}
          </button>
        </h2>
        <div
          id="panelsStayOpen-collapseTwo"
          class=move || format!("accordion-collapse collapse {}", _show_accordion.get())
          aria-labelledby="headingAccordionItem"
        >
          <div class="accordion-body accordion-content">
            <p>r"Creator: " <strong>r"Kênia Araujo"</strong></p>
            <p>r"Size: " <strong>r"89MB"</strong></p>
            <p>r"Last modified: " <strong>r"Marcia Newton"</strong></p>
            <p><strong>r"04/16/2023 10:08:44 PM"</strong></p>
          </div>
        </div>
      </div>
    </div>
  }
}