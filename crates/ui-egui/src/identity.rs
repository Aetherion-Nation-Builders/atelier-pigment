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

/// UI strings that are ours, not upstream's, with their translations: the catalogs cannot carry
/// them (they are upstream files), so they are added to each language's catalog as it loads.
/// `about` is the one line in the About box.
pub fn strings(code: &str) -> Vec<(&'static str, &'static str)> {
    const JOIN: &str = "Join the Nation";
    const JOIN_MENU: &str = "Join the Nation…";
    const NATION: &str = "Nation";
    const SUITE_MENU: &str = "Atelier Suite";
    const ABOUT: &str = "Atelier Pigment — the Nation's image editor. Built on PhotoCraft, open source, in Rust.";
    let (join, join_menu, nation, suite_menu, about) = match code {
        "fr" => (
            "Rejoindre la Nation",
            "Rejoindre la Nation…",
            "Nation",
            "Suite Atelier",
            "Atelier Pigment — l'éditeur d'images de la Nation. Fondé sur PhotoCraft, libre, écrit en Rust.",
        ),
        "de" => (
            "Der Nation beitreten",
            "Der Nation beitreten…",
            "Nation",
            "Atelier-Suite",
            "Atelier Pigment — der Bildeditor der Nation. Aufgebaut auf PhotoCraft, Open Source, in Rust.",
        ),
        "es" => (
            "Únete a la Nación",
            "Únete a la Nación…",
            "Nación",
            "Suite Atelier",
            "Atelier Pigment — el editor de imágenes de la Nación. Construido sobre PhotoCraft, de código abierto, en Rust.",
        ),
        "it" => (
            "Unisciti alla Nazione",
            "Unisciti alla Nazione…",
            "Nazione",
            "Suite Atelier",
            "Atelier Pigment — l'editor di immagini della Nazione. Basato su PhotoCraft, open source, in Rust.",
        ),
        "pt-br" => (
            "Junte-se à Nação",
            "Junte-se à Nação…",
            "Nação",
            "Suíte Atelier",
            "Atelier Pigment — o editor de imagens da Nação. Construído sobre o PhotoCraft, código aberto, em Rust.",
        ),
        "nl" => (
            "Word lid van de Natie",
            "Word lid van de Natie…",
            "Natie",
            "Atelier-suite",
            "Atelier Pigment — de beeldbewerker van de Natie. Gebouwd op PhotoCraft, open source, in Rust.",
        ),
        "pl" => (
            "Dołącz do Narodu",
            "Dołącz do Narodu…",
            "Naród",
            "Pakiet Atelier",
            "Atelier Pigment — edytor obrazów Narodu. Zbudowany na PhotoCraft, open source, w Rust.",
        ),
        "cs" => (
            "Přidat se k Národu",
            "Přidat se k Národu…",
            "Národ",
            "Sada Atelier",
            "Atelier Pigment — obrazový editor Národa. Postaven na PhotoCraftu, open source, v Rustu.",
        ),
        "ru" => (
            "Присоединиться к Нации",
            "Присоединиться к Нации…",
            "Нация",
            "Пакет Atelier",
            "Atelier Pigment — редактор изображений Нации. Построен на PhotoCraft, открытый код, на Rust.",
        ),
        "uk" => (
            "Приєднатися до Нації",
            "Приєднатися до Нації…",
            "Нація",
            "Пакет Atelier",
            "Atelier Pigment — редактор зображень Нації. Побудований на PhotoCraft, відкритий код, на Rust.",
        ),
        "el" => (
            "Γίνετε μέλος του Έθνους",
            "Γίνετε μέλος του Έθνους…",
            "Έθνος",
            "Σουίτα Atelier",
            "Atelier Pigment — ο επεξεργαστής εικόνων του Έθνους. Χτισμένος πάνω στο PhotoCraft, ανοιχτού κώδικα, σε Rust.",
        ),
        "id" => (
            "Bergabung dengan Nation",
            "Bergabung dengan Nation…",
            "Nation",
            "Suite Atelier",
            "Atelier Pigment — penyunting gambar milik Nation. Dibangun di atas PhotoCraft, sumber terbuka, dalam Rust.",
        ),
        "ja" => (
            "Nation に参加",
            "Nation に参加…",
            "Nation",
            "Atelier スイート",
            "Atelier Pigment — Nation の画像エディター。PhotoCraft を基に、オープンソース、Rust 製。",
        ),
        "ko" => (
            "Nation에 참여",
            "Nation에 참여…",
            "Nation",
            "Atelier 스위트",
            "Atelier Pigment — Nation의 이미지 편집기. PhotoCraft 기반, 오픈 소스, Rust로 제작.",
        ),
        "zh-hans" => ("加入 Nation", "加入 Nation…", "Nation", "Atelier 套件", "Atelier Pigment — Nation 的图像编辑器。基于 PhotoCraft，开源，Rust 编写。"),
        "zh-hant" => {
            ("加入 Nation", "加入 Nation…", "Nation", "Atelier 套件", "Atelier Pigment — Nation 的影像編輯器。基於 PhotoCraft，開放原始碼，Rust 編寫。")
        }
        _ => (JOIN, JOIN_MENU, NATION, SUITE_MENU, ABOUT),
    };
    vec![(JOIN, join), (JOIN_MENU, join_menu), (NATION, nation), (SUITE_MENU, suite_menu), (ABOUT, about), ("Atelier", "Atelier")]
}

#[cfg(test)]
mod string_tests {
    #[test]
    fn every_language_gets_the_same_set_of_keys() {
        let en: Vec<&str> = super::strings("en").iter().map(|(k, _)| *k).collect();
        for code in ["fr", "de", "es", "it", "pt-br", "nl", "pl", "cs", "ru", "uk", "el", "id", "ja", "ko", "zh-hans", "zh-hant", "xx"] {
            let keys: Vec<&str> = super::strings(code).iter().map(|(k, _)| *k).collect();
            assert_eq!(keys, en, "{code}");
            for (k, v) in super::strings(code) {
                assert!(!v.is_empty() && v.matches('…').count() == k.matches('…').count(), "{code}: {k:?} -> {v:?}");
            }
        }
    }
}
