use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RarityCode {
    C,
    U,
    R,
    M,
}

impl RarityCode {
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        let s_ref = s.as_ref();
        match s_ref.to_lowercase().as_str() {
            "common" => RarityCode::C,
            "uncommon" => RarityCode::U,
            "rare" => RarityCode::R,
            "mythic" => RarityCode::M,
            _ => panic!("invalid rarity code : {}", s_ref),
        }
    }
}

impl Display for RarityCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RarityCode::C => write!(f, "C"),
            RarityCode::U => write!(f, "U"),
            RarityCode::R => write!(f, "R"),
            RarityCode::M => write!(f, "M"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_common_for_common_string() {
        assert_eq!(RarityCode::new("common"), RarityCode::C);
    }

    #[test]
    fn new_returns_uncommon_for_uncommon_string() {
        assert_eq!(RarityCode::new("uncommon"), RarityCode::U);
    }

    #[test]
    fn new_returns_rare_for_rare_string() {
        assert_eq!(RarityCode::new("rare"), RarityCode::R);
    }

    #[test]
    fn new_returns_mythic_for_mythic_string() {
        assert_eq!(RarityCode::new("mythic"), RarityCode::M);
    }

    #[test]
    fn new_is_case_insensitive_for_common() {
        assert_eq!(RarityCode::new("Common"), RarityCode::C);
        assert_eq!(RarityCode::new("COMMON"), RarityCode::C);
    }

    #[test]
    fn new_is_case_insensitive_for_uncommon() {
        assert_eq!(RarityCode::new("Uncommon"), RarityCode::U);
        assert_eq!(RarityCode::new("UNCOMMON"), RarityCode::U);
    }

    #[test]
    fn new_is_case_insensitive_for_rare() {
        assert_eq!(RarityCode::new("Rare"), RarityCode::R);
        assert_eq!(RarityCode::new("RARE"), RarityCode::R);
    }

    #[test]
    fn new_is_case_insensitive_for_mythic() {
        assert_eq!(RarityCode::new("Mythic"), RarityCode::M);
        assert_eq!(RarityCode::new("MYTHIC"), RarityCode::M);
    }

    #[test]
    #[should_panic(expected = "invalid rarity code")]
    fn new_panics_on_unknown_rarity() {
        RarityCode::new("special");
    }

    #[test]
    #[should_panic(expected = "invalid rarity code")]
    fn new_panics_on_empty_string() {
        RarityCode::new("");
    }

    #[test]
    fn display_formats_common_as_c() {
        assert_eq!(RarityCode::C.to_string(), "C");
    }

    #[test]
    fn display_formats_uncommon_as_u() {
        assert_eq!(RarityCode::U.to_string(), "U");
    }

    #[test]
    fn display_formats_rare_as_r() {
        assert_eq!(RarityCode::R.to_string(), "R");
    }

    #[test]
    fn display_formats_mythic_as_m() {
        assert_eq!(RarityCode::M.to_string(), "M");
    }
}
