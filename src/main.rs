mod config;
mod experiments;
mod io;
mod math;
mod parser;
mod plot;
mod raport;
mod runner;
mod simulations;
mod task1;
mod task2;
mod task3;
mod theme;
mod task_1;
mod task_2;
mod analysis;
mod naming;
mod rich_text;

use plot::PlotFactory;
use raport::report;
use runner::ExperimentRunner;
use parser::load_grades_dataset;
use crate::parser::load_income_data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("out")?;
    simulations::set_seed(config::GLOBAL_SEED);
    //
    let pf = PlotFactory::new("out", theme::Theme::white());
    let runner = ExperimentRunner::new(config::GLOBAL_SEED);
    //
    // let report = report()
    //     .title("Raport Statystyczny")
    //     .author("Łukasz Stodółka");
    //
    // let report = task1::append(report, &pf, &runner)?;
    // let report = task2::append(report, &pf, &runner)?;
    // let report = task3::append(report, &pf, &runner)?;
    //
    // report.build_with_pdf("out/raport.html", "out/raport.pdf")?;
    //
    // println!("Gotowe: out/raport.html i out/raport.pdf");

    std::fs::create_dir_all("out")?;

    let dataset = load_grades_dataset("data/grades.TXT")?;
    let dataset2 = load_income_data("data/income.dat")?;
    let report = report()
        .title("Raport Statystyczny nr 1")
        .author("Łukasz Stodółka");

    let report = task_1::append(report, &pf, &runner, &dataset)?;
    let report = task_2::append(report, &pf, &runner, &dataset2)?;
    report.build_with_pdf("out/raport2.html", "out/raport2.pdf")?;

    println!("Gotowe: out/raport2.html i out/raport2.pdf");
    Ok(())
}