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
        // <div class="container min-h-screen min-w-full bg-[#E7E9E8] justify-center place-items-center">
            // <nav>
                // <components::nav_bar::NavBar />
            // </nav>
            // <div class="min-h-screen bg-[#E7E9E8] justify-center">
            <Routes>
                <Route path="/" view=GraphExample />
            </Routes>
        // </div>
        </Router>
        }
}


