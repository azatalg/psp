use handlebars::Handlebars;
use serde::Serialize;
use std::fs;



#[derive(Serialize)]
pub struct ReportData {
    pub n_total: usize,
    pub sroc_mean: String,
    pub sroc_median: String,
    pub sroc_min: String,
    pub sroc_max: String,
    pub sroc_q1: String,
    pub sroc_q3: String,
    pub sroc_var: String,
    pub sroc_std: String,
    pub sroc_cv: String,
    pub iq_mean: String,
    pub iq_median: String,
    pub iq_min: String,
    pub iq_max: String,
    pub iq_q1: String,
    pub iq_q3: String,
    pub iq_var: String,
    pub iq_std: String,
    pub iq_cv: String,
    pub ph_mean: String,
    pub ph_median: String,
    pub ph_min: String,
    pub ph_max: String,
    pub ph_q1: String,
    pub ph_q3: String,
    pub ph_var: String,
    pub ph_std: String,
    pub ph_cv: String,
    pub hist_iq_path: String,
    pub boxplot_iq_path: String,
    pub hist_sroc_path: String,
    pub boxplot_sroc_path: String,
    pub hist_ph_path: String,
    pub boxplot_ph_path: String,
    pub iq_mean_b: String,
    pub iq_mean_g: String,
    pub sroc_mean_b: String,
    pub sroc_mean_g: String,
    pub iq_median_b: String,
    pub iq_median_g: String,
    pub sroc_median_b: String,
    pub sroc_median_g: String,
    pub boxplot_iq_gender_path: String,
    pub boxplot_sroc_gender_path: String,
    pub boxplot_ph_gender_path: String,
    pub bins_few: usize,
    pub bins_opt: usize,
    pub bins_many: usize,
    pub hist_iq_few_path: String,
    pub hist_iq_opt_path: String,
    pub hist_iq_many_path: String,
}


pub fn format_f64(value: f64) -> String {
    if value.is_nan() {
        "Brak".to_string()
    } else {
        format!("{:.2}", value)
    }
}


pub fn generate_markdown(
    data: &ReportData,
    template_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {

    let template_str = fs::read_to_string(template_path)?;


    let mut reg = Handlebars::new();


    reg.register_escape_fn(handlebars::no_escape);

    reg.register_template_string("raport_template", template_str)?;


    let rendered = reg.render("raport_template", data)?;


    fs::write(output_path, rendered)?;

    Ok(())
}