use crate::models::{FilterField, FilterFieldType, SourceFilters};

pub fn get_wallhaven_filters() -> SourceFilters {
    SourceFilters {
        source_name: "wallhaven".to_string(),
        fields: vec![
            FilterField {
                name: "q".to_string(),
                display_name: "Search Query".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "Enter search terms...".to_string(),
                required: false,
            },
            FilterField {
                name: "categories".to_string(),
                display_name: "Categories".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: Some("111".to_string()),
                placeholder: "e.g. 111 (general/anime/people)".to_string(),
                required: false,
            },
            FilterField {
                name: "purity".to_string(),
                display_name: "Purity".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: Some("100".to_string()),
                placeholder: "e.g. 100 (sfw/sketchy/nsfw)".to_string(),
                required: false,
            },
            FilterField {
                name: "sorting".to_string(),
                display_name: "Sort By".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec![
                        "date_added".to_string(),
                        "relevance".to_string(),
                        "random".to_string(),
                        "views".to_string(),
                        "favorites".to_string(),
                        "toplist".to_string(),
                    ],
                },
                default_value: Some("date_added".to_string()),
                placeholder: "Select sort order".to_string(),
                required: false,
            },
            FilterField {
                name: "order".to_string(),
                display_name: "Order".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec!["desc".to_string(), "asc".to_string()],
                },
                default_value: Some("desc".to_string()),
                placeholder: "Select order".to_string(),
                required: false,
            },
            FilterField {
                name: "topRange".to_string(),
                display_name: "Top Range".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec![
                        "1d".to_string(),
                        "3d".to_string(),
                        "1w".to_string(),
                        "1M".to_string(),
                        "3M".to_string(),
                        "6M".to_string(),
                        "1y".to_string(),
                    ],
                },
                default_value: Some("1M".to_string()),
                placeholder: "Select time range".to_string(),
                required: false,
            },
            FilterField {
                name: "atleast".to_string(),
                display_name: "Min Resolution".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "e.g. 1920x1080".to_string(),
                required: false,
            },
            FilterField {
                name: "resolutions".to_string(),
                display_name: "Resolutions".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "e.g. 1920x1080,2560x1440".to_string(),
                required: false,
            },
            FilterField {
                name: "ratios".to_string(),
                display_name: "Aspect Ratios".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "e.g. 16x9,16x10".to_string(),
                required: false,
            },
            FilterField {
                name: "colors".to_string(),
                display_name: "Colors".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "e.g. 660000,990000".to_string(),
                required: false,
            },
            FilterField {
                name: "page".to_string(),
                display_name: "Page".to_string(),
                filter_type: FilterFieldType::Number,
                default_value: Some("1".to_string()),
                placeholder: "Page number".to_string(),
                required: false,
            },
            FilterField {
                name: "per_page".to_string(),
                display_name: "Per Page".to_string(),
                filter_type: FilterFieldType::Number,
                default_value: Some("24".to_string()),
                placeholder: "Results per page".to_string(),
                required: false,
            },
        ],
    }
}

pub fn get_unsplash_filters() -> SourceFilters {
    SourceFilters {
        source_name: "unsplash".to_string(),
        fields: vec![
            FilterField {
                name: "query".to_string(),
                display_name: "Search Query".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "Enter search terms...".to_string(),
                required: false,
            },
            FilterField {
                name: "per_page".to_string(),
                display_name: "Per Page".to_string(),
                filter_type: FilterFieldType::Number,
                default_value: Some("10".to_string()),
                placeholder: "Results per page".to_string(),
                required: false,
            },
            FilterField {
                name: "order_by".to_string(),
                display_name: "Order By".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec![
                        "relevant".to_string(),
                        "latest".to_string(),
                        "editorial".to_string(),
                    ],
                },
                default_value: Some("relevant".to_string()),
                placeholder: "Select order".to_string(),
                required: false,
            },
            FilterField {
                name: "orientation".to_string(),
                display_name: "Orientation".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec![
                        "landscape".to_string(),
                        "portrait".to_string(),
                        "squarish".to_string(),
                    ],
                },
                default_value: None,
                placeholder: "Select orientation".to_string(),
                required: false,
            },
            FilterField {
                name: "color".to_string(),
                display_name: "Color".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "e.g. #ff0000 or black_and_white".to_string(),
                required: false,
            },
        ],
    }
}

pub fn get_pexels_filters() -> SourceFilters {
    SourceFilters {
        source_name: "pexels".to_string(),
        fields: vec![
            FilterField {
                name: "query".to_string(),
                display_name: "Search Query".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "Enter search terms...".to_string(),
                required: false,
            },
            FilterField {
                name: "page".to_string(),
                display_name: "Page".to_string(),
                filter_type: FilterFieldType::Number,
                default_value: Some("1".to_string()),
                placeholder: "Page number".to_string(),
                required: false,
            },
            FilterField {
                name: "per_page".to_string(),
                display_name: "Per Page".to_string(),
                filter_type: FilterFieldType::Number,
                default_value: Some("15".to_string()),
                placeholder: "Results per page".to_string(),
                required: false,
            },
            FilterField {
                name: "orientation".to_string(),
                display_name: "Orientation".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec![
                        "landscape".to_string(),
                        "portrait".to_string(),
                        "square".to_string(),
                    ],
                },
                default_value: None,
                placeholder: "Select orientation".to_string(),
                required: false,
            },
            FilterField {
                name: "size".to_string(),
                display_name: "Size".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec![
                        "large".to_string(),
                        "medium".to_string(),
                        "small".to_string(),
                    ],
                },
                default_value: None,
                placeholder: "Select size".to_string(),
                required: false,
            },
            FilterField {
                name: "color".to_string(),
                display_name: "Color".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "e.g. red, blue, #ff0000".to_string(),
                required: false,
            },
        ],
    }
}

pub fn get_filters_for_source(source: &str) -> Option<SourceFilters> {
    match source {
        "wallhaven" => Some(get_wallhaven_filters()),
        "unsplash" => Some(get_unsplash_filters()),
        "pexels" => Some(get_pexels_filters()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallhaven_filter_count() {
        let filters = get_wallhaven_filters();
        assert_eq!(filters.fields.len(), 12);
    }

    #[test]
    fn test_unsplash_filter_count() {
        let filters = get_unsplash_filters();
        assert_eq!(filters.fields.len(), 5);
    }

    #[test]
    fn test_pexels_filter_count() {
        let filters = get_pexels_filters();
        assert_eq!(filters.fields.len(), 6);
    }

    #[test]
    fn test_get_filters_for_source_wallhaven() {
        let filters = get_filters_for_source("wallhaven");
        assert!(filters.is_some());
        assert_eq!(filters.unwrap().source_name, "wallhaven");
    }

    #[test]
    fn test_get_filters_for_source_unsplash() {
        let filters = get_filters_for_source("unsplash");
        assert!(filters.is_some());
        assert_eq!(filters.unwrap().source_name, "unsplash");
    }

    #[test]
    fn test_get_filters_for_source_pexels() {
        let filters = get_filters_for_source("pexels");
        assert!(filters.is_some());
        assert_eq!(filters.unwrap().source_name, "pexels");
    }

    #[test]
    fn test_get_filters_for_source_unknown() {
        let filters = get_filters_for_source("unknown");
        assert!(filters.is_none());
    }
}
