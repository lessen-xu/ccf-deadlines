use leptos::prelude::*;

// Modules
mod components;
pub mod i18n;
mod pages;

// Top-Level pages
use crate::pages::home::Home;

#[component]
pub fn App() -> impl IntoView {
    view! { <Home /> }
}

pub use components::directory_controls::DirectoryControls;
