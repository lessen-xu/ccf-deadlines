//! Bilingual UI text (English / 简体中文).
//!
//! Every user-visible string needs both languages, including `title`, `aria-label`,
//! `placeholder` and `alt` values:
//! - static text: `tr("English", "中文")`, e.g. `{tr("Close", "关闭")}` or `title=tr(..)`;
//! - thaw props typed `MaybeProp<String>` or `Signal<String>`: `tr_signal("English", "中文")`;
//! - text built at runtime: read `use_english()` in the component body and format both versions
//!   in a pure `fn(.., english: bool) -> String` that is unit-tested in both languages.
//!
//! The language flag comes from context, which `use_language_preference()` provides at the app
//! roots, so components don't need a `use_english` prop. Call these helpers in the component body
//! or `view!`, not inside event handlers, `spawn_local` or timers: there is no reactive owner
//! there, so they fall back to English. Existing `if use_english.get() { .. } else { .. }` code
//! can stay as it is.
use leptos::prelude::*;

/// Context key; private so it cannot clash with other `RwSignal<bool>` contexts.
#[derive(Clone, Copy)]
struct Language(RwSignal<bool>);

/// Shares the language flag (`true` = English) with everything under the current owner.
pub fn provide_language(english: RwSignal<bool>) {
    provide_context(Language(english));
}

/// The shared language flag, or English when nothing provided it (unit tests, outside the app).
pub fn use_english() -> RwSignal<bool> {
    use_context::<Language>()
        .map(|Language(english)| english)
        .unwrap_or_else(|| RwSignal::new(true))
}

/// Reactive static text for text nodes and attributes: `{tr("Close", "关闭")}`, `title=tr(..)`.
pub fn tr(
    en: &'static str,
    zh: &'static str,
) -> impl Fn() -> &'static str + Copy + Send + Sync + 'static {
    let english = use_english();
    move || if english.get() { en } else { zh }
}

/// [`tr`] as a `Signal<String>`, for thaw props such as `placeholder` or `label`.
pub fn tr_signal(en: &'static str, zh: &'static str) -> Signal<String> {
    let text = tr(en, zh);
    Signal::derive(move || text().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn falls_back_to_english_without_a_provider() {
        Owner::new().with(|| {
            assert!(use_english().get_untracked());
            assert_eq!(untrack(tr("Close", "关闭")), "Close");
        });
    }

    #[test]
    fn follows_the_provided_language() {
        Owner::new().with(|| {
            let english = RwSignal::new(false);
            provide_language(english);
            let close = tr("Close", "关闭");
            let label = tr_signal("Close", "关闭");
            assert_eq!(untrack(close), "关闭");
            assert_eq!(label.get_untracked(), "关闭");
            // A nested owner, like a child component, sees the same flag.
            Owner::new().with(|| assert_eq!(use_english(), english));

            english.set(true);
            assert_eq!(untrack(close), "Close");
            assert_eq!(label.get_untracked(), "Close");
        });
    }
}
