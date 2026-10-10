// The columns of a listing that can be edited after it has been recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Area,
    CloseDate,
    Company,
    DateApplied,
    Level,
    Notes,
    Pay,
    Requirements,
    Role,
    Stage,
}

impl Field {
    // Accepts the labels shown in the update form ("Date Applied", "Grades")
    // as well as the column names themselves.
    pub fn from_label(label: &str) -> Option<Self> {
        match label.trim().to_lowercase().replace(' ', "_").as_str() {
            "area" => Some(Field::Area),
            "close_date" => Some(Field::CloseDate),
            "company" => Some(Field::Company),
            "date_applied" => Some(Field::DateApplied),
            "level" => Some(Field::Level),
            "notes" => Some(Field::Notes),
            "pay" => Some(Field::Pay),
            "requirements" | "grades" => Some(Field::Requirements),
            "role" => Some(Field::Role),
            "stage" => Some(Field::Stage),
            _ => None,
        }
    }

    pub fn column(self) -> &'static str {
        match self {
            Field::Area => "area",
            Field::CloseDate => "close_date",
            Field::Company => "company",
            Field::DateApplied => "date_applied",
            Field::Level => "level",
            Field::Notes => "notes",
            Field::Pay => "pay",
            Field::Requirements => "requirements",
            Field::Role => "role",
            Field::Stage => "stage",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Field; 10] = [
        Field::Area,
        Field::CloseDate,
        Field::Company,
        Field::DateApplied,
        Field::Level,
        Field::Notes,
        Field::Pay,
        Field::Requirements,
        Field::Role,
        Field::Stage,
    ];

    #[test]
    fn from_label_accepts_column_names() {
        for field in ALL {
            assert_eq!(Field::from_label(field.column()), Some(field));
        }
    }

    #[test]
    fn from_label_accepts_form_labels() {
        assert_eq!(Field::from_label("Date Applied"), Some(Field::DateApplied));
        assert_eq!(Field::from_label("Close Date"), Some(Field::CloseDate));
        assert_eq!(Field::from_label("Grades"), Some(Field::Requirements));
    }

    #[test]
    fn from_label_ignores_case_and_surrounding_whitespace() {
        assert_eq!(Field::from_label("  COMPANY "), Some(Field::Company));
        assert_eq!(Field::from_label("sTaGe"), Some(Field::Stage));
    }

    #[test]
    fn from_label_rejects_unknown_labels() {
        assert_eq!(Field::from_label("id"), None);
        assert_eq!(Field::from_label("user_id"), None);
        assert_eq!(Field::from_label(""), None);
        assert_eq!(
            Field::from_label("company; DROP TABLE apprenticeship"),
            None
        );
    }
}
