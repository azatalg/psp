mod math;
mod plot;
mod theme;
mod gen_pdf;
mod parser;
use std::env;
use plot::ChartConfig;
use theme::Theme;
use gen_pdf::{format_f64, generate_markdown, ReportData};
use std::{fs, process::Command};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("out")?;
    println!("Wczytywanie pliku data/grades.TXT...");
    let (iq_boys, iq_girls, sroc_boys, sroc_girls, ph_boys, ph_girls) =
        parser::load_grades_data("data/grades.TXT")?;

    let mut iq_all = iq_boys.clone();
    iq_all.extend(&iq_girls);

    let mut sroc_all = sroc_boys.clone();
    sroc_all.extend(&sroc_girls);

    let mut ph_all = ph_boys.clone();
    ph_all.extend(&ph_girls);

    println!(
        "Wczytano {} uczniów ({} chłopców, {} dziewcząt).",
        iq_all.len(), iq_boys.len(), iq_girls.len()
    );

    println!("Liczenie statystyk...");
    let stats_iq = math::summarize(&iq_all);
    let stats_sroc = math::summarize(&sroc_all);
    let stats_ph = math::summarize(&ph_all);
    let stats_iq_b = math::summarize(&iq_boys);
    let stats_iq_g = math::summarize(&iq_girls);
    let stats_sroc_b = math::summarize(&sroc_boys);
    let stats_sroc_g = math::summarize(&sroc_girls);

    println!("Generowanie wykresów obrazkowych...");
    let theme = Theme::white();


    ChartConfig::new("out/hist_iq.png", "Rozkład IQ")
        .with_theme(theme.clone())
        .draw_histogram(&iq_all, math::optimal_bins_sturges(iq_all.len()))?;
    ChartConfig::new("out/boxplot_iq.png", "Wykres ramkowy IQ")
        .with_theme(theme.clone())
        .draw_boxplots(&[("Ogółem", &iq_all)])?;
    ChartConfig::new("out/boxplot_iq_gender.png", "IQ (Chłopcy vs Dziewczęta)")
        .with_theme(theme.clone())
        .draw_boxplots(&[("Chłopcy", &iq_boys), ("Dziewczęta", &iq_girls)])?;
    ChartConfig::new("out/hist_sroc.png", "Rozkład ŚrOc")
        .with_theme(theme.clone())
        .draw_histogram(&sroc_all, math::optimal_bins_sturges(sroc_all.len()))?;
    ChartConfig::new("out/boxplot_sroc.png", "Wykres ramkowy ŚrOc")
        .with_theme(theme.clone())
        .draw_boxplots(&[("Ogółem", &sroc_all)])?;
    ChartConfig::new("out/boxplot_sroc_gender.png", "ŚrOc (Chłopcy vs Dziewczęta)")
        .with_theme(theme.clone())
        .draw_boxplots(&[("Chłopcy", &sroc_boys), ("Dziewczęta", &sroc_girls)])?;
    ChartConfig::new("out/hist_ph.png", "Rozkład PH")
        .with_theme(theme.clone())
        .draw_histogram(&ph_all, math::optimal_bins_sturges(ph_all.len()))?;
    ChartConfig::new("out/boxplot_ph.png", "Wykres ramkowy PH")
        .with_theme(theme.clone())
        .draw_boxplots(&[("Ogółem", &ph_all)])?;
    ChartConfig::new("out/boxplot_ph_gender.png", "PH (Chłopcy vs Dziewczęta)")
        .with_theme(theme.clone())
        .draw_boxplots(&[("Chłopcy", &ph_boys), ("Dziewczęta", &ph_girls)])?;

    let bins_opt = math::optimal_bins_sturges(iq_all.len());
    let bins_few = 3;
    let bins_many = 30;

    ChartConfig::new("out/hist_iq_few.png", "Rozkład IQ (Zbyt mało klas)")
        .with_theme(theme.clone())
        .draw_histogram(&iq_all, bins_few)?;
    ChartConfig::new("out/hist_iq_opt.png", "Rozkład IQ (Optymalnie)")
        .with_theme(theme.clone())
        .draw_histogram(&iq_all, bins_opt)?;
    ChartConfig::new("out/hist_iq_many.png", "Rozkład IQ (Zbyt dużo klas)")
        .with_theme(theme.clone())
        .draw_histogram(&iq_all, bins_many)?;

    println!("Składanie raportu do kupy...");
    let data = ReportData {
        n_total: iq_all.len(),
        sroc_mean: format_f64(stats_sroc.mean),
        sroc_median: format_f64(stats_sroc.median),
        sroc_min: format_f64(stats_sroc.min),
        sroc_max: format_f64(stats_sroc.max),
        sroc_q1: format_f64(stats_sroc.q1),
        sroc_q3: format_f64(stats_sroc.q3),
        sroc_var: format_f64(stats_sroc.variance_sample),
        sroc_std: format_f64(stats_sroc.std_dev_sample),
        sroc_cv: format_f64(stats_sroc.cv_sample * 100.0),
        iq_mean: format_f64(stats_iq.mean),
        iq_median: format_f64(stats_iq.median),
        iq_min: format_f64(stats_iq.min),
        iq_max: format_f64(stats_iq.max),
        iq_q1: format_f64(stats_iq.q1),
        iq_q3: format_f64(stats_iq.q3),
        iq_var: format_f64(stats_iq.variance_sample),
        iq_std: format_f64(stats_iq.std_dev_sample),
        iq_cv: format_f64(stats_iq.cv_sample * 100.0),
        ph_mean: format_f64(stats_ph.mean),
        ph_median: format_f64(stats_ph.median),
        ph_min: format_f64(stats_ph.min),
        ph_max: format_f64(stats_ph.max),
        ph_q1: format_f64(stats_ph.q1),
        ph_q3: format_f64(stats_ph.q3),
        ph_var: format_f64(stats_ph.variance_sample),
        ph_std: format_f64(stats_ph.std_dev_sample),
        ph_cv: format_f64(stats_ph.cv_sample * 100.0),
        hist_iq_path: "hist_iq.png".to_string(),
        boxplot_iq_path: "boxplot_iq.png".to_string(),
        hist_sroc_path: "hist_sroc.png".to_string(),
        boxplot_sroc_path: "boxplot_sroc.png".to_string(),
        hist_ph_path: "hist_ph.png".to_string(),
        boxplot_ph_path: "boxplot_ph.png".to_string(),
        iq_mean_b: format_f64(stats_iq_b.mean),
        iq_mean_g: format_f64(stats_iq_g.mean),
        sroc_mean_b: format_f64(stats_sroc_b.mean),
        sroc_mean_g: format_f64(stats_sroc_g.mean),
        iq_median_b: format_f64(stats_iq_b.median),
        iq_median_g: format_f64(stats_iq_g.median),
        sroc_median_b: format_f64(stats_sroc_b.median),
        sroc_median_g: format_f64(stats_sroc_g.median),
        boxplot_iq_gender_path: "boxplot_iq_gender.png".to_string(),
        boxplot_sroc_gender_path: "boxplot_sroc_gender.png".to_string(),
        boxplot_ph_gender_path: "boxplot_ph_gender.png".to_string(),
        bins_few,
        bins_opt,
        bins_many,
        hist_iq_few_path: "hist_iq_few.png".to_string(),
        hist_iq_opt_path: "hist_iq_opt.png".to_string(),
        hist_iq_many_path: "hist_iq_many.png".to_string(),
    };

    generate_markdown(&data, "templates/raport.html.hbs", "out/raport.html")?;
    println!("Wygenerowano out/raport.html");

    let current_dir = env::current_dir()?;
    let html_path = current_dir.join("out").join("raport.html");
    let pdf_path = current_dir.join("out").join("lista1zad1.pdf");

    println!("Generowanie PDFa za pomocą przeglądarki");

    let browser_path = "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe";

    let status = std::process::Command::new(browser_path)
        .args([
            "--headless",
            "--disable-gpu",
            "--no-pdf-header-footer",
            &format!("--print-to-pdf={}", pdf_path.display()),
            html_path.to_str().unwrap()
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("PDF gotowy: out/raport.pdf");
        }
        Ok(s) => eprintln!("Przeglądarka zakończyła się kodem: {}", s),
        Err(e) => eprintln!("Błąd uruchamiania przeglądarki: {}", e),
    }

    Ok(())
}