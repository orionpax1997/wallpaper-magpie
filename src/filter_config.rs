use crate::models::{FilterField, FilterFieldType, SourceFilters};

pub fn get_wallhaven_filters() -> SourceFilters {
    SourceFilters {
        source_name: "wallhaven".to_string(),
        fields: vec![
            FilterField {
                name: "page".to_string(),
                display_name: "下载页数".to_string(),
                filter_type: FilterFieldType::Number,
                default_value: Some("1".to_string()),
                placeholder: "下载页数".to_string(),
                hint: None,
                required: false,
            },
            FilterField {
                name: "q".to_string(),
                display_name: "关键词".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "搜索关键词".to_string(),
                hint: Some("tagname, -tagname, +tag1 +tag2, @username, id:123, type:png/jpg".to_string()),
                required: false,
            },
            FilterField {
                name: "categories".to_string(),
                display_name: "选择分类".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec![
                        "111".to_string(),
                        "110".to_string(),
                        "101".to_string(),
                        "011".to_string(),
                        "100".to_string(),
                        "010".to_string(),
                        "001".to_string(),
                    ],
                },
                default_value: Some("111".to_string()),
                placeholder: "选择分类".to_string(),
                hint: Some("1=开启, 0=关闭 (general/anime/people)".to_string()),
                required: false,
            },
            FilterField {
                name: "purity".to_string(),
                display_name: "选择纯度".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec!["SFW".to_string(), "Sketchy".to_string(), "NSFW".to_string()],
                },
                default_value: Some("SFW".to_string()),
                placeholder: "选择纯度".to_string(),
                hint: Some("SFW=安全, Sketchy=可疑, NSFW需API Key".to_string()),
                required: false,
            },
            FilterField {
                name: "sorting".to_string(),
                display_name: "选择排序方式".to_string(),
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
                default_value: Some("toplist".to_string()),
                placeholder: "选择排序方式".to_string(),
                hint: None,
                required: false,
            },
            FilterField {
                name: "order".to_string(),
                display_name: "选择顺序".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec!["desc".to_string(), "asc".to_string()],
                },
                default_value: Some("desc".to_string()),
                placeholder: "选择顺序".to_string(),
                hint: Some("desc=降序, asc=升序".to_string()),
                required: false,
            },
            FilterField {
                name: "topRange".to_string(),
                display_name: "选择时间范围".to_string(),
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
                placeholder: "选择时间范围".to_string(),
                hint: Some("排序方式需设为 toplist".to_string()),
                required: false,
            },
            FilterField {
                name: "atleast".to_string(),
                display_name: "最小分辨率".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "如: 1920x1080".to_string(),
                hint: Some("最小宽x高，如 1920x1080".to_string()),
                required: false,
            },
            FilterField {
                name: "resolutions".to_string(),
                display_name: "分辨率".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "如: 1920x1080,2560x1440".to_string(),
                hint: Some("精确分辨率，多个用逗号分隔".to_string()),
                required: false,
            },
            FilterField {
                name: "ratios".to_string(),
                display_name: "宽高比".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "如: 16x9,16x10".to_string(),
                hint: Some("宽x高比例，如 16x9, 16x10".to_string()),
                required: false,
            },
            FilterField {
                name: "colors".to_string(),
                display_name: "颜色".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "如: 660000".to_string(),
                hint: Some("6位十六进制颜色码，如 ffffff, 000000, 0099cc".to_string()),
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
                name: "page".to_string(),
                display_name: "页码".to_string(),
                filter_type: FilterFieldType::Number,
                default_value: Some("1".to_string()),
                placeholder: "页码".to_string(),
                hint: None,
                required: false,
            },
            FilterField {
                name: "query".to_string(),
                display_name: "关键词".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "搜索关键词".to_string(),
                hint: None,
                required: false,
            },
            FilterField {
                name: "per_page".to_string(),
                display_name: "每页数量".to_string(),
                filter_type: FilterFieldType::Number,
                default_value: Some("10".to_string()),
                placeholder: "每页数量".to_string(),
                hint: None,
                required: false,
            },
            FilterField {
                name: "order_by".to_string(),
                display_name: "选择排序".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec!["relevant".to_string(), "latest".to_string()],
                },
                default_value: Some("relevant".to_string()),
                placeholder: "选择排序".to_string(),
                hint: None,
                required: false,
            },
            FilterField {
                name: "orientation".to_string(),
                display_name: "选择方向".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec![
                        "landscape".to_string(),
                        "portrait".to_string(),
                        "squarish".to_string(),
                    ],
                },
                default_value: None,
                placeholder: "选择方向".to_string(),
                hint: None,
                required: false,
            },
            FilterField {
                name: "color".to_string(),
                display_name: "颜色".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "如: #000000".to_string(),
                hint: None,
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
                name: "page".to_string(),
                display_name: "页码".to_string(),
                filter_type: FilterFieldType::Number,
                default_value: Some("1".to_string()),
                placeholder: "页码".to_string(),
                hint: None,
                required: false,
            },
            FilterField {
                name: "query".to_string(),
                display_name: "关键词".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "搜索关键词".to_string(),
                hint: None,
                required: false,
            },
            FilterField {
                name: "per_page".to_string(),
                display_name: "每页数量".to_string(),
                filter_type: FilterFieldType::Number,
                default_value: Some("10".to_string()),
                placeholder: "每页数量".to_string(),
                hint: None,
                required: false,
            },
            FilterField {
                name: "orientation".to_string(),
                display_name: "选择方向".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec![
                        "landscape".to_string(),
                        "portrait".to_string(),
                        "square".to_string(),
                    ],
                },
                default_value: None,
                placeholder: "选择方向".to_string(),
                hint: None,
                required: false,
            },
            FilterField {
                name: "size".to_string(),
                display_name: "选择尺寸".to_string(),
                filter_type: FilterFieldType::Enum {
                    options: vec![
                        "large".to_string(),
                        "medium".to_string(),
                        "small".to_string(),
                    ],
                },
                default_value: None,
                placeholder: "选择尺寸".to_string(),
                hint: None,
                required: false,
            },
            FilterField {
                name: "color".to_string(),
                display_name: "颜色".to_string(),
                filter_type: FilterFieldType::Text,
                default_value: None,
                placeholder: "如: red, #FF0000".to_string(),
                hint: None,
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
        assert_eq!(filters.fields.len(), 11);
    }

    #[test]
    fn test_unsplash_filter_count() {
        let filters = get_unsplash_filters();
        assert_eq!(filters.fields.len(), 6);
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
