use leptos::*;

#[component]
pub fn Modal() -> impl IntoView {
    view! {
      <div class="modal" tabindex="-1">
        <div class="modal-dialog">
          <div class="modal-content">
            <div class="modal-header">
              <h5 class="modal-title">Modal title</h5>
              <button type="button" class="btn-close" data-mdb-ripple-init data-mdb-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
              <p>Modal body text goes here.</p>
            </div>
            <div class="modal-footer">
              <button type="button" class="btn btn-secondary" data-mdb-ripple-init data-mdb-dismiss="modal">Close</button>
              <button type="button" class="btn btn-primary" data-mdb-ripple-init>Save changes</button>
            </div>
          </div>
        </div>
      </div>
    }
}