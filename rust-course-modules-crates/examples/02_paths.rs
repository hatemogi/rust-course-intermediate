// ANCHOR: external_imports
use serde_json::{Value, json};
// ANCHOR_END: external_imports

// ANCHOR: path_module_open
mod shop {
    pub mod catalog {
        // ANCHOR_END: path_module_open
        pub struct Item {
            name: &'static str,
        }

        impl Item {
            pub fn new(name: &'static str) -> Self {
                Self { name }
            }

            pub fn name(&self) -> &str {
                self.name
            }
        }

        // ANCHOR: path_starts
        // ANCHOR: item_count
        pub fn item_count() -> usize {
            3
        }
        // ANCHOR_END: item_count

        pub fn count_from_crate_root() -> usize {
            crate::shop::catalog::item_count() // ①
        }

        pub fn count_from_current_module() -> usize {
            self::item_count() // ②
        }

        pub fn has_items_from_parent() -> bool {
            super::has_items() // ③
        }
        // ANCHOR_END: path_starts
        // ANCHOR: path_module_close
        // ANCHOR: catalog_close
    }
    // ANCHOR_END: catalog_close

    pub fn has_items() -> bool {
        self::catalog::item_count() > 0
    }
    // ANCHOR: shop_close
}
// ANCHOR_END: shop_close
// ANCHOR_END: path_module_close

// ANCHOR: aliases
mod online {
    pub fn item_count() -> usize {
        5
    }
}

mod store {
    pub fn item_count() -> usize {
        3
    }
}

fn compare_item_counts() {
    use crate::online::item_count as online_item_count;
    use crate::store::item_count as store_item_count;

    assert_eq!(online_item_count(), 5);
    assert_eq!(store_item_count(), 3);
}
// ANCHOR_END: aliases

fn read_external_value() {
    // ANCHOR: external_use
    let task: Value = json!({
        "title": "문서 작성",
        "done": false,
    });

    assert_eq!(task["title"], "문서 작성");
    assert_eq!(task["done"], false);
    // ANCHOR_END: external_use
}

// ANCHOR: main_open
fn main() {
    // ANCHOR_END: main_open
    // ANCHOR: root_paths
    let absolute_count = crate::shop::catalog::item_count();
    let relative_count = shop::catalog::item_count();
    assert_eq!(absolute_count, relative_count);
    // ANCHOR_END: root_paths

    // ANCHOR: imports
    use crate::shop::catalog;
    use crate::shop::catalog::Item;

    let item = Item::new("키보드");
    assert_eq!(item.name(), "키보드");
    assert_eq!(catalog::item_count(), 3);
    assert!(shop::has_items());
    // ANCHOR_END: imports

    assert_eq!(catalog::count_from_crate_root(), 3);
    assert_eq!(catalog::count_from_current_module(), 3);
    assert!(catalog::has_items_from_parent());
    compare_item_counts();
    read_external_value();
    // ANCHOR: main_close
}
// ANCHOR_END: main_close
