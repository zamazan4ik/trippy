use crate::config::{TuiColumn, TuiColumns};
use ratatui::layout::{Constraint, Rect};
use std::fmt::{Display, Formatter};

/// The columns to display in the hops table of the TUI.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Columns(Vec<Column>);

impl Columns {
    /// Column width constraints.
    ///
    /// All columns are returned as `Constraint::Min(width)`.
    ///
    /// For `Fixed(n)` columns the width is as specified in `n`.
    /// For `Variable` columns the width is calculated by subtracting the total
    /// size of all `Fixed` columns from the width of the containing `Rect` and
    /// dividing by the number of `Variable` columns.
    pub fn constraints(&self, rect: Rect) -> Vec<Constraint> {
        let total_fixed_width = self
            .columns()
            .map(|c| match c.typ.width() {
                ColumnWidth::Fixed(width) => width,
                ColumnWidth::Variable => 0,
            })
            .sum();
        let variable_width_count = self
            .columns()
            .filter(|c| matches!(c.typ.width(), ColumnWidth::Variable))
            .count() as u16;
        let variable_width =
            rect.width.saturating_sub(total_fixed_width) / variable_width_count.max(1);
        self.columns()
            .map(|c| match c.typ.width() {
                ColumnWidth::Fixed(width) => Constraint::Min(width),
                ColumnWidth::Variable => Constraint::Min(variable_width),
            })
            .collect()
    }

    pub fn columns(&self) -> impl Iterator<Item = &Column> {
        self.0
            .iter()
            .filter(|c| matches!(c.status, ColumnStatus::Shown))
    }

    pub fn all_columns(&self) -> impl Iterator<Item = &Column> {
        self.0.iter()
    }

    pub fn toggle(&mut self, index: usize) {
        if self.0[index].status == ColumnStatus::Shown {
            self.0[index].status = ColumnStatus::Hidden;
        } else {
            self.0[index].status = ColumnStatus::Shown;
        }
    }

    pub fn move_down(&mut self, index: usize) {
        let removed = self.0.remove(index);
        self.0.insert(index + 1, removed);
    }

    pub fn move_up(&mut self, index: usize) {
        let removed = self.0.remove(index);
        self.0.insert(index - 1, removed);
    }
}

impl From<TuiColumns> for Columns {
    fn from(value: TuiColumns) -> Self {
        Self(value.0.into_iter().map(Column::from).collect())
    }
}

impl Display for Columns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output: Vec<char> = self.0.clone().into_iter().map(|c| c.typ.into()).collect();
        write!(f, "{}", String::from_iter(output))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Column {
    pub typ: ColumnType,
    pub status: ColumnStatus,
}

impl Column {
    pub fn new(typ: ColumnType) -> Self {
        Self {
            typ,
            status: ColumnStatus::Shown,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ColumnStatus {
    Shown,
    Hidden,
}

impl Display for ColumnStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shown => write!(f, "on"),
            Self::Hidden => write!(f, "off"),
        }
    }
}

/// A TUI hops table column.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColumnType {
    /// The ttl for a hop.
    Ttl,
    /// The hostname for a hostname.
    Host,
    /// The packet loss % for a hop.
    LossPct,
    /// The number of probes sent for a hop.
    Sent,
    /// The number of responses received for a hop.
    Received,
    /// The last RTT for a hop.
    Last,
    /// The rolling average RTT for a hop.
    Average,
    /// The best RTT for a hop.
    Best,
    /// The worst RTT for a hop.
    Worst,
    /// The stddev of RTT for a hop.
    StdDev,
    /// The status of a hop.
    Status,
}

impl From<ColumnType> for char {
    fn from(col_type: ColumnType) -> Self {
        match col_type {
            ColumnType::Ttl => 'h',
            ColumnType::Host => 'o',
            ColumnType::LossPct => 'l',
            ColumnType::Sent => 's',
            ColumnType::Received => 'r',
            ColumnType::Last => 'a',
            ColumnType::Average => 'v',
            ColumnType::Best => 'b',
            ColumnType::Worst => 'w',
            ColumnType::StdDev => 'd',
            ColumnType::Status => 't',
        }
    }
}

impl From<TuiColumn> for Column {
    fn from(value: TuiColumn) -> Self {
        match value {
            TuiColumn::Ttl => Self::new(ColumnType::Ttl),
            TuiColumn::Host => Self::new(ColumnType::Host),
            TuiColumn::LossPct => Self::new(ColumnType::LossPct),
            TuiColumn::Sent => Self::new(ColumnType::Sent),
            TuiColumn::Received => Self::new(ColumnType::Received),
            TuiColumn::Last => Self::new(ColumnType::Last),
            TuiColumn::Average => Self::new(ColumnType::Average),
            TuiColumn::Best => Self::new(ColumnType::Best),
            TuiColumn::Worst => Self::new(ColumnType::Worst),
            TuiColumn::StdDev => Self::new(ColumnType::StdDev),
            TuiColumn::Status => Self::new(ColumnType::Status),
        }
    }
}

impl Display for ColumnType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ttl => write!(f, "#"),
            Self::Host => write!(f, "Host"),
            Self::LossPct => write!(f, "Loss%"),
            Self::Sent => write!(f, "Snd"),
            Self::Received => write!(f, "Recv"),
            Self::Last => write!(f, "Last"),
            Self::Average => write!(f, "Avg"),
            Self::Best => write!(f, "Best"),
            Self::Worst => write!(f, "Wrst"),
            Self::StdDev => write!(f, "StDev"),
            Self::Status => write!(f, "Sts"),
        }
    }
}

impl ColumnType {
    /// The width of the column.
    pub(self) fn width(self) -> ColumnWidth {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ttl => ColumnWidth::Fixed(4),
            Self::Host => ColumnWidth::Variable,
            Self::LossPct => ColumnWidth::Fixed(8),
            Self::Sent => ColumnWidth::Fixed(7),
            Self::Received => ColumnWidth::Fixed(7),
            Self::Last => ColumnWidth::Fixed(7),
            Self::Average => ColumnWidth::Fixed(7),
            Self::Best => ColumnWidth::Fixed(7),
            Self::Worst => ColumnWidth::Fixed(7),
            Self::StdDev => ColumnWidth::Fixed(8),
            Self::Status => ColumnWidth::Fixed(7),
        }
    }
}

/// Table column layout constraints.
#[derive(Debug, PartialEq)]
enum ColumnWidth {
    /// A fixed size column.
    Fixed(u16),
    /// A column that will use the remaining space.
    Variable,
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ratatui::layout::Constraint::Min;
//     use test_case::test_case;
//
//     #[test]
//     fn test_columns_conversion_from_tui_columns() {
//         let tui_columns = TuiColumns(vec![
//             TuiColumn::Ttl,
//             TuiColumn::Host,
//             TuiColumn::LossPct,
//             TuiColumn::Sent,
//         ]);
//
//         let columns = Columns::from(tui_columns);
//
//         assert_eq!(
//             columns,
//             Columns(vec![
//                 ColumnType::Ttl,
//                 ColumnType::Host,
//                 ColumnType::LossPct,
//                 ColumnType::Sent,
//             ])
//         );
//     }
//
//     #[test]
//     fn test_column_conversion_from_tui_column() {
//         let tui_column = TuiColumn::Received;
//         let column = ColumnType::from(tui_column);
//
//         assert_eq!(column, ColumnType::Received);
//     }
//
//     #[test_case(Column::Ttl, "#")]
//     #[test_case(Column::Host, "Host")]
//     #[test_case(Column::LossPct, "Loss%")]
//     #[test_case(Column::Sent, "Snd")]
//     #[test_case(Column::Received, "Recv")]
//     #[test_case(Column::Last, "Last")]
//     #[test_case(Column::Average, "Avg")]
//     #[test_case(Column::Best, "Best")]
//     #[test_case(Column::Worst, "Wrst")]
//     #[test_case(Column::StdDev, "StDev")]
//     #[test_case(Column::Status, "Sts")]
//     fn test_column_display_formatting(c: ColumnType, heading: &'static str) {
//         assert_eq!(format!("{c}"), heading);
//     }
//
//     #[test_case(Column::Ttl, & ColumnWidth::Fixed(4))]
//     #[test_case(Column::Host, & ColumnWidth::Variable)]
//     #[test_case(Column::LossPct, & ColumnWidth::Fixed(8))]
//     fn test_column_width(column_type: ColumnType, width: &ColumnWidth) {
//         assert_eq!(column_type.width(), *width);
//     }
//
//     #[test]
//     fn test_column_constraints() {
//         let columns = Columns::from(TuiColumns::default());
//         let constraints = columns.constraints(Rect::new(0, 0, 80, 0));
//         assert_eq!(
//             vec![
//                 Min(4),
//                 Min(11),
//                 Min(8),
//                 Min(7),
//                 Min(7),
//                 Min(7),
//                 Min(7),
//                 Min(7),
//                 Min(7),
//                 Min(8),
//                 Min(7)
//             ],
//             constraints
//         );
//     }
//
//     /// Expect to test the Column Into <char> flow.
//     #[test]
//     fn test_columns_into_string_short() {
//         let cols = Columns(vec![
//             ColumnType::Ttl,
//             ColumnType::Host,
//             ColumnType::LossPct,
//             ColumnType::Sent,
//         ]);
//         assert_eq!("hols", format!("{cols}"));
//     }
//
//     /// Happy path test for full set of columns.
//     #[test]
//     fn test_columns_into_string_happy_path() {
//         let cols = Columns(vec![
//             ColumnType::Ttl,
//             ColumnType::Host,
//             ColumnType::LossPct,
//             ColumnType::Sent,
//             ColumnType::Received,
//             ColumnType::Last,
//             ColumnType::Average,
//             ColumnType::Best,
//             ColumnType::Worst,
//             ColumnType::StdDev,
//             ColumnType::Status,
//         ]);
//         assert_eq!("holsravbwdt", format!("{cols}"));
//     }
//
//     /// Reverse subset test for subset of columns.
//     #[test]
//     fn test_columns_into_string_reverse_str() {
//         let cols = Columns(vec![
//             ColumnType::Status,
//             ColumnType::Last,
//             ColumnType::StdDev,
//             ColumnType::Worst,
//             ColumnType::Best,
//             ColumnType::Average,
//             ColumnType::Received,
//         ]);
//         assert_eq!("tadwbvr", format!("{cols}"));
//     }
// }
