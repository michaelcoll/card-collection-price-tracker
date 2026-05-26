use super::*;

#[test]
fn collection_sort_field_default_is_avg() {
    assert_eq!(CollectionSortField::default(), CollectionSortField::Trend);
}

#[test]
fn collection_sort_field_display() {
    assert_eq!(CollectionSortField::Avg.to_string(), "avg");
    assert_eq!(CollectionSortField::SetCode.to_string(), "set_code");
    assert_eq!(
        CollectionSortField::LanguageCode.to_string(),
        "language_code"
    );
}

#[test]
fn sort_direction_default_is_desc() {
    assert_eq!(SortDirection::default(), SortDirection::Desc);
}

#[test]
fn sort_direction_display() {
    assert_eq!(SortDirection::Asc.to_string(), "ASC");
    assert_eq!(SortDirection::Desc.to_string(), "DESC");
}

#[test]
fn collection_query_default_values() {
    let q = CollectionQuery::default();
    assert_eq!(q.page, 0);
    assert_eq!(q.page_size, 20);
    assert_eq!(q.sort_by, CollectionSortField::Trend);
    assert_eq!(q.sort_dir, SortDirection::Desc);
    assert_eq!(q.search_query, None);
}
