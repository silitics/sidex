//! Tests marker traits of groups in generated Rust code.

use sidex_serde_tests::generated::ClientGroup;
use sidex_serde_tests::generated::groups::Change;
use sidex_serde_tests::generated::groups::Item;
use sidex_serde_tests::generated::groups::ItemId;
use sidex_serde_tests::generated::groups::Page;
use sidex_serde_tests::generated::groups::SyncChanges;

fn assert_member<T: ClientGroup>() {}

/// Records, variants, wrappers, and generic definitions belonging to a group
/// implement its marker trait.
#[test]
fn members_implement_group_trait() {
    assert_member::<SyncChanges>();
    assert_member::<Page<Change>>();
    assert_member::<Change>();
    assert_member::<Item>();
    assert_member::<ItemId>();
}
