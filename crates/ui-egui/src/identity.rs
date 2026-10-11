//! Who this app is: the product name and where it lives. Atelier Pigment is built on PhotoCraft
//! (storytold/photocraft, MIT OR Apache-2.0). The code keeps its upstream names (crates, command
//! ids, the `.pcraft` format) so upstream work keeps merging cleanly; only what a person sees is
//! renamed, through the constants here and the catalog rewrite in [`rewrite`]. The places that
//! carry the name outside this crate are listed in `docs/brand.md`.

/// The product name as shown in the title bar, menus, dialogs and the start screen.
pub const PRODUCT: &str = "Atelier Pigment";
/// The short name, for places where the suite name is already in view.
pub const SHORT: &str = "Pigment";
/// The suite this app belongs to.
pub const SUITE: &str = "Atelier";
/// Who makes it.
pub const VENDOR: &str = "Aetherion Nation";
/// The upstream project this app is built on, as named in its own strings and catalogs.
pub const UPSTREAM: &str = "PhotoCraft";

/// Rewrites an upstream string so it names this product. Applied to every translation catalog
/// entry as it is read, so the UI follows the brand in every language without editing the
/// catalogs (which would make every upstream catalog change a merge conflict).
pub fn rewrite(s: &str) -> String {
    s.replace(UPSTREAM, PRODUCT)
}

/// Like [`rewrite`], borrowing when there is nothing to change (the common case).
pub fn rewrite_cow(s: &str) -> std::borrow::Cow<'_, str> {
    if s.contains(UPSTREAM) { std::borrow::Cow::Owned(rewrite(s)) } else { std::borrow::Cow::Borrowed(s) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_upstream_name_is_rewritten_wherever_it_appears() {
        assert_eq!(rewrite("About PhotoCraft"), "About Atelier Pigment");
        assert_eq!(rewrite("PhotoCraft — PhotoCraft"), "Atelier Pigment — Atelier Pigment");
        assert_eq!(rewrite("Photoshop"), "Photoshop");
        assert!(matches!(rewrite_cow("Open"), std::borrow::Cow::Borrowed(_)));
        assert!(matches!(rewrite_cow("Quit PhotoCraft"), std::borrow::Cow::Owned(_)));
    }

    #[test]
    fn the_names_hang_together() {
        assert!(PRODUCT.starts_with(SUITE));
        assert!(PRODUCT.ends_with(SHORT));
        assert_ne!(PRODUCT, UPSTREAM);
    }
}

/// Our own UI strings, with their translations: `i18n/brand.tsv`. The catalogs are upstream
/// files, so our strings live beside them and are added to each language's catalog as it loads.
/// The file's `product` line is the rename [`rewrite`] applies; a test keeps it equal to
/// [`PRODUCT`].
const BRAND_TSV: &str = include_str!("i18n/brand.tsv");

/// `(code, source, translation)` for every line of `brand.tsv` (the `product` line included).
fn brand_lines() -> impl Iterator<Item = (&'static str, &'static str, &'static str)> {
    BRAND_TSV.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).filter_map(|l| {
        let mut cols = l.split('\t');
        match (cols.next(), cols.next(), cols.next(), cols.next()) {
            (Some(code), Some(src), Some(tr), None) if !src.is_empty() && !tr.is_empty() => Some((code, src, tr)),
            _ => None,
        }
    })
}

/// The brand strings for a language (English for a language the file does not cover).
pub fn strings(code: &str) -> Vec<(&'static str, &'static str)> {
    let of = |c: &str| brand_lines().filter(|(l, _, _)| *l == c).map(|(_, s, t)| (s, t)).collect::<Vec<_>>();
    let own = of(code);
    if own.is_empty() { of("en") } else { own }
}

#[cfg(test)]
mod string_tests {
    use super::*;

    #[test]
    fn the_product_line_matches_the_constants() {
        let product = brand_lines().find(|(code, _, _)| *code == "product");
        assert_eq!(product, Some(("product", UPSTREAM, PRODUCT)));
    }

    #[test]
    fn every_line_parses_and_every_language_covers_the_english_set() {
        let lines = BRAND_TSV.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).count();
        assert_eq!(lines, brand_lines().count(), "every non-comment line of brand.tsv has three columns");
        let en: Vec<&str> = strings("en").iter().map(|(k, _)| *k).collect();
        assert!(en.len() >= 5);
        let codes: std::collections::BTreeSet<&str> = brand_lines().map(|(c, _, _)| c).filter(|c| *c != "product" && *c != "en").collect();
        assert!(codes.len() >= 16, "{codes:?}");
        for code in codes {
            let keys: Vec<&str> = strings(code).iter().map(|(k, _)| *k).collect();
            assert_eq!(keys, en, "{code}");
            for (k, v) in strings(code) {
                assert_eq!(v.matches('…').count(), k.matches('…').count(), "{code}: {k:?} -> {v:?}");
            }
        }
        assert_eq!(strings("xx"), strings("en"));
    }
}
