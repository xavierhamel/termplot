use crate::Domain;
use std::fmt;

pub(crate) struct XTicks {
    labels: Vec<String>,
    width: usize,
}

impl XTicks {
    pub fn new(domain: &Domain, width: usize, count: usize) -> Self {
        let max = format!("{:.1}", domain.max());
        let min = format!("{:.1}", domain.min());
        Self {
            labels: vec![min, max],
            width,
        }
    }
}

impl fmt::Display for XTicks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let labels_width = self.labels.iter().fold(0, |sum, label| label.len() + sum);
        let total_spacing = self.width - labels_width;
        let spacing = total_spacing / (self.labels.len() - 1);
        for (index, label) in self.labels.iter().enumerate() {
            let space = if index == 0 { 0 } else { spacing };
            write!(f, "{: >space$}{label}", "")?;
        }
        let fill = self.width - spacing * (self.labels.len() - 1) - labels_width;
        write!(f, "{: >fill$}", "",)
    }
}

pub(crate) struct YTicks {
    labels: Vec<String>,
    row_indexes: Vec<usize>,
}

impl YTicks {
    pub fn new(codomain: &Domain, row_count: usize, count: usize) -> Self {
        let max = format!("{:.1}", codomain.max());
        let min = format!("{:.1}", codomain.min());
        Self {
            labels: vec![max, min],
            row_indexes: vec![0, row_count - 1],
        }
    }

    pub fn display_width(&self) -> usize {
        let widest_label = self.labels.iter().max_by_key(|label| label.len());
        match widest_label {
            Some(label) => label.len(),
            _ => 0,
        }
    }

    pub fn get(&self, row_index: usize) -> &str {
        let maybe_index = self
            .row_indexes
            .iter()
            .position(|&index| index == row_index);
        match maybe_index {
            Some(index) => &self.labels[index],
            None => "",
        }
    }
}
