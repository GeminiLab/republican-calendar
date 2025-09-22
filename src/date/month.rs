use core::fmt::{self, Display, Formatter};

use enumerable::Enumerable;
/// Months of the Republican Calendar, including Sans-culottides
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enumerable)]
pub enum Month {
    /// Vendémiaire, month 1 of Autumn, months 1 to 30 of the year.
    Vendémiaire = 1,
    /// Brumaire, month 2 of Autumn, months 31 to 60 of the year.
    Brumaire = 2,
    /// Frimaire, month 3 of Autumn, months 61 to 90 of the year.
    Frimaire = 3,
    /// Nivôse, month 1 of Winter, months 91 to 120 of the year.
    Nivôse = 4,
    /// Pluviôse, month 2 of Winter, months 121 to 150 of the year.
    Pluviôse = 5,
    /// Ventôse, month 3 of Winter, months 151 to 180 of the year.
    Ventôse = 6,
    /// Germinal, month 1 of Spring, months 181 to 210 of the year.
    Germinal = 7,
    /// Floréal, month 2 of Spring, months 211 to 240 of the year.
    Floréal = 8,
    /// Prairial, month 3 of Spring, months 241 to 270 of the year.
    Prairial = 9,
    /// Messidor, month 1 of Summer, months 271 to 300 of the year.
    Messidor = 10,
    /// Thermidor, month 2 of Summer, months 301 to 330 of the year.
    Thermidor = 11,
    /// Fructidor, month 3 of Summer, months 331 to 360 of the year.
    Fructidor = 12,
    /// Sans-culottides, the complementary days at the end of the year.
    SansCulottides = 13,
}

impl Display for Month {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let month_str = match self {
            Month::Vendémiaire => "Vendémiaire",
            Month::Brumaire => "Brumaire",
            Month::Frimaire => "Frimaire",
            Month::Nivôse => "Nivôse",
            Month::Pluviôse => "Pluviôse",
            Month::Ventôse => "Ventôse",
            Month::Germinal => "Germinal",
            Month::Floréal => "Floréal",
            Month::Prairial => "Prairial",
            Month::Messidor => "Messidor",
            Month::Thermidor => "Thermidor",
            Month::Fructidor => "Fructidor",
            Month::SansCulottides => "Sans-culottides",
        };
        write!(f, "{month_str}")
    }
}

impl TryFrom<u32> for Month {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Month::Vendémiaire),
            2 => Ok(Month::Brumaire),
            3 => Ok(Month::Frimaire),
            4 => Ok(Month::Nivôse),
            5 => Ok(Month::Pluviôse),
            6 => Ok(Month::Ventôse),
            7 => Ok(Month::Germinal),
            8 => Ok(Month::Floréal),
            9 => Ok(Month::Prairial),
            10 => Ok(Month::Messidor),
            11 => Ok(Month::Thermidor),
            12 => Ok(Month::Fructidor),
            13 => Ok(Month::SansCulottides),
            _ => Err(()),
        }
    }
}

impl TryFrom<i32> for Month {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Month::try_from(value as u32)
    }
}
