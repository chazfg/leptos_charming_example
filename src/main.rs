use leptos::*;
use leptos_router::*;
mod graph;
use crate::graph::GraphExample;

fn main() {
    mount_to_body(App);
}


#[component]
fn App() -> impl IntoView {

   view! {
        <Router>
            <div class="min-h-screen min-w-full justify-items-center justify-center">
            <Routes>
                <Route path="/" view=GraphExample />
            </Routes>
        </div>
        </Router>
        }
}


