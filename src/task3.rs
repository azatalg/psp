use crate::config;
use crate::parser::{self, IncomeRecord};
use crate::plot::{PlotFactory, PlotRef};
use crate::raport::{ReportBuilder, Table};
use crate::runner::ExperimentRunner;
use crate::simulations;

fn fmt(x: f64) -> String {
    format!("{:.4}", x)
}

fn extract_incomes(records: &[IncomeRecord]) -> Vec<f64> {
    records.iter().map(|r| r.income).collect()
}

pub fn append(
    report: ReportBuilder,
    pf: &PlotFactory,
    runner: &ExperimentRunner,
) -> Result<ReportBuilder, Box<dyn std::error::Error>> {
    let income_records = parser::load_income_data("data/income.dat")?;
    let population = extract_incomes(&income_records);

    let pop_stats = crate::math::summarize(&population);
    let pop_mean = pop_stats.mean;

    let sample_means_table = config::TASK3_NS.iter().fold(
        Table::new(["n", "Średnia próby", "Średnia populacji", "Różnica"]),
        |table, &n| {
            let res = runner.population_sample(
                "task3",
                "population_sample",
                &population,
                n,
                simulations::sample_from_population_without_replacement,
            ).expect("population sample failed");

            table.row([
                n.to_string(),
                fmt(res.sample_mean),
                fmt(res.population_mean),
                fmt(res.abs_diff),
            ])
        },
    );

    let population_hist = pf.population_hist(
        "zad3_population_hist",
        "Histogram zarobków całej populacji",
        &population,
    )?;

    let hist_samples: Vec<PlotRef> = [10usize, 50, 100]
        .iter()
        .map(|&n| {
            let res = runner.population_sample(
                "task3",
                "population_hist_sample",
                &population,
                n,
                simulations::sample_from_population_without_replacement,
            )?;

            pf.population_hist(
                &format!("zad3_sample_hist_{}", n),
                &format!("Histogram zarobków próby, n={}", n),
                &res.sample,
            )
        })
        .collect::<Result<_, _>>()?;

    let mean_hists: Vec<(PlotRef, String)> = config::TASK3_NS
        .iter()
        .map(|&n| {
            let clt = runner.population_clt(
                "task3",
                "population_clt",
                &population,
                n,
                config::TASK3_REPS,
                simulations::sample_from_population_with_replacement,
            )?;

            let hist = pf.population_hist(
                &format!("zad3_mean_hist_n{}", n),
                &format!("Histogram średnich próbkowych, n={}", n),
                &clt.means,
            )?;

            let theoretical_std = pop_stats.std_dev_population / (n as f64).sqrt();

            Ok::<_, Box<dyn std::error::Error>>((
                hist,
                format!(
                    "Dla n = {} teoretyczne odchylenie standardowe średniej wynosi {}.",
                    n,
                    fmt(theoretical_std)
                ),
            ))
        })
        .collect::<Result<_, _>>()?;

    Ok(
        report
            .section("Zadanie 3(a): średnia populacji a średnie z prób losowych", |s| {
                s.text("Dane ze zbioru income.dat potraktowano jako pełną populację. Dla kilku rozmiarów próby porównano średnią populacji ze średnimi wybranych prób losowych.")
                    .table(sample_means_table)
                    .note(&format!("Średnia całej populacji wynosi {}.", fmt(pop_mean)))
                    .conclusion("Wraz ze wzrostem liczności próby średnia próbki zwykle lepiej przybliża średnią całej populacji, co stanowi ilustrację prawa wielkich liczb.");
            })
            .section("Zadanie 3(b): histogram populacji i histogramy prób", |s| {
                s.text("Porównano histogram zarobków całej populacji z histogramami kilku prób losowych o różnych rozmiarach.")
                    .plot(population_hist);

                let mut plots = Vec::new();
                plots.extend(hist_samples);
                s.plot_grid(plots, 2)
                    .conclusion("Dla małych prób histogram może znacząco odbiegać od histogramu populacji, natomiast przy większych próbach kształt rozkładu staje się coraz bardziej zbliżony do rozkładu całej populacji.");
            })
            .section("Zadanie 3(c): histogramy średnich próbkowych", |s| {
                s.text("Dla różnych liczności próby wielokrotnie losowano próbki i obliczano ich średnie. Następnie zbadano rozkład średnich próbkowych.");

                let plots: Vec<PlotRef> = mean_hists.iter().map(|(p, _)| p.clone()).collect();
                s.plot_grid(plots, 2);

                for (_, note) in &mean_hists {
                    s.note(note);
                }

                s.conclusion("Rozkład średnich próbkowych staje się coraz bardziej skupiony wokół średniej populacji wraz ze wzrostem n. Jest to zgodne z faktem, że odchylenie standardowe średniej maleje jak σ/√n.");
            })
    )
}