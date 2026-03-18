use crate::math::{self, SummaryStats};
use crate::plot::{PlotFactory, PlotRef};
use crate::raport::Table;

pub struct VariableReport {
    pub name: String,
    pub stats: SummaryStats,
    pub hist: PlotRef,
    pub boxplot: PlotRef,
}

pub struct GroupComparison {
    pub table: Table,
    pub plots: Vec<PlotRef>,
    pub box_compare: PlotRef,
}

pub fn analyze_variable(
    pf: &PlotFactory,
    key: &str,
    display_name: &str,
    data: &[f64],
) -> Result<VariableReport, Box<dyn std::error::Error>> {
    Ok(VariableReport {
        name: display_name.to_string(),
        stats: math::summarize(data),
        hist: pf.histogram_auto(
            &format!("{}_hist", key),
            &format!("Rozkład {}", display_name),
            data,
        )?,
        boxplot: pf.boxplot_single(
            &format!("{}_box", key),
            &format!("Wykres ramkowy {}", display_name),
            display_name,
            data,
        )?,
    })
}

pub fn compare_groups(
    pf: &PlotFactory,
    key: &str,
    display_name: &str,
    g1: (&str, &[f64]),
    g2: (&str, &[f64]),
) -> Result<GroupComparison, Box<dyn std::error::Error>> {
    let s1 = math::summarize(g1.1);
    let s2 = math::summarize(g2.1);

    let p1_hist = pf.histogram_auto(
        &format!("{}_{}_hist", key, slug(g1.0)),
        &format!("Rozkład {} - {}", display_name, g1.0),
        g1.1,
    )?;
    let p1_box = pf.boxplot_single(
        &format!("{}_{}_box", key, slug(g1.0)),
        &format!("Wykres ramkowy {} - {}", display_name, g1.0),
        g1.0,
        g1.1,
    )?;

    let p2_hist = pf.histogram_auto(
        &format!("{}_{}_hist", key, slug(g2.0)),
        &format!("Rozkład {} - {}", display_name, g2.0),
        g2.1,
    )?;
    let p2_box = pf.boxplot_single(
        &format!("{}_{}_box", key, slug(g2.0)),
        &format!("Wykres ramkowy {} - {}", display_name, g2.0),
        g2.0,
        g2.1,
    )?;

    let box_compare = pf.boxplot_groups(
        &format!("{}_compare", key),
        &format!("{} ({} vs {})", display_name, g1.0, g2.0),
        &[g1, g2],
    )?;

    Ok(GroupComparison {
        table: Table::compare_two_groups(g1.0, g2.0, &s1, &s2),
        plots: vec![p1_hist, p1_box, p2_hist, p2_box],
        box_compare,
    })
}

fn slug(s: &str) -> String {
    s.to_lowercase()
        .replace('ł', "l")
        .replace('ó', "o")
        .replace('ś', "s")
        .replace('ż', "z")
        .replace('ź', "z")
        .replace('ć', "c")
        .replace('ń', "n")
        .replace('ą', "a")
        .replace('ę', "e")
        .replace(' ', "_")
}

pub struct PrettyStats(pub SummaryStats);

impl PrettyStats {
    pub fn mean(&self) -> String { format!("{:.2}", self.0.mean) }
    pub fn median(&self) -> String { format!("{:.2}", self.0.median) }
    pub fn min(&self) -> String { format!("{:.2}", self.0.min) }
    pub fn max(&self) -> String { format!("{:.2}", self.0.max) }
    pub fn q1(&self) -> String { format!("{:.2}", self.0.q1) }
    pub fn q3(&self) -> String { format!("{:.2}", self.0.q3) }
    pub fn var(&self) -> String { format!("{:.2}", self.0.variance_sample) }
    pub fn std(&self) -> String { format!("{:.2}", self.0.std_dev_sample) }
    pub fn cv(&self) -> String { format!("{:.2}%", self.0.cv_sample * 100.0) }
}