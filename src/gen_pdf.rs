use handlebars::Handlebars;
use serde::Serialize;
use std::fs;

#[derive(Serialize, Clone)]
pub struct VariableStats {
    pub mean: String,
    pub median: String,
    pub min: String,
    pub max: String,
    pub q1: String,
    pub q3: String,
    pub var: String,
    pub std: String,
    pub cv: String,
}

#[derive(Serialize, Clone)]
pub struct VariablePlots {
    pub hist_path: String,
    pub boxplot_path: String,
}

#[derive(Serialize, Clone)]
pub struct VariableSection {
    pub stats: VariableStats,
    pub plots: VariablePlots,
}

#[derive(Serialize, Clone)]
pub struct OverallSection {
    pub iq: VariableSection,
    pub sroc: VariableSection,
    pub ph: VariableSection,
}

#[derive(Serialize, Clone)]
pub struct GenderSection {
    pub iq: VariableSection,
    pub sroc: VariableSection,
    pub ph: VariableSection,
}

#[derive(Serialize, Clone)]
pub struct ComparePlots {
    pub boxplot_iq_gender_path: String,
    pub boxplot_sroc_gender_path: String,
    pub boxplot_ph_gender_path: String,
}

#[derive(Serialize, Clone)]
pub struct HistogramBinsSection {
    pub bins_few: usize,
    pub bins_opt: usize,
    pub bins_many: usize,
    pub hist_iq_few_path: String,
    pub hist_iq_opt_path: String,
    pub hist_iq_many_path: String,
}

#[derive(Serialize, Clone)]
pub struct Task1Section {
    pub n_total: usize,
    pub n_boys: usize,
    pub n_girls: usize,
    pub overall: OverallSection,
    pub boys: GenderSection,
    pub girls: GenderSection,
    pub compare: ComparePlots,
    pub bins_demo: HistogramBinsSection,
}

#[derive(Serialize, Clone)]
pub struct GroupStatsRow {
    pub label: String,
    pub mean: String,
    pub median: String,
    pub min: String,
    pub max: String,
    pub q1: String,
    pub q3: String,
    pub std: String,
    pub cv: String,
}

#[derive(Serialize, Clone)]
pub struct Task2ASection {
    pub n_total: usize,
    pub income: VariableSection,
}

#[derive(Serialize, Clone)]
pub struct Task2BSection {
    pub rows: Vec<GroupStatsRow>,
    pub boxplot_path: String,
}

#[derive(Serialize, Clone)]
pub struct Task2CSection {
    pub men: VariableSection,
    pub women: VariableSection,
    pub compare_boxplot_path: String,
}

#[derive(Serialize, Clone)]
pub struct Task2DSection {
    pub mean: String,
    pub median: String,
}

#[derive(Serialize, Clone)]
pub struct Task2Section {
    pub a: Task2ASection,
    pub b: Task2BSection,
    pub c: Task2CSection,
    pub d: Task2DSection,
}

#[derive(Serialize)]
pub struct ReportData {
    pub task1: Task1Section,
    pub task2: Task2Section,
}

pub fn format_f64(value: f64) -> String {
    if value.is_nan() {
        "Brak".to_string()
    } else {
        format!("{:.2}", value)
    }
}

pub fn generate_markdown<T: Serialize>(
    data: &T,
    base_template_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_template = fs::read_to_string(base_template_path)?;
    let task1_template = fs::read_to_string("templates/task1.html.hbs")?;
    let task2_template = fs::read_to_string("templates/task2.html.hbs")?;

    let mut reg = Handlebars::new();
    reg.register_escape_fn(handlebars::no_escape);

    reg.register_template_string("base", base_template)?;
    reg.register_partial("task1", task1_template)?;
    reg.register_partial("task2", task2_template)?;

    let rendered = reg.render("base", data)?;
    fs::write(output_path, rendered)?;

    Ok(())
}