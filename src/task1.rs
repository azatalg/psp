use crate::config;
use crate::plot::PlotFactory;
use crate::raport::{ReportBuilder, Table};
use crate::runner::ExperimentRunner;
use crate::simulations;

pub fn append(
    report: ReportBuilder,
    pf: &PlotFactory,
    runner: &ExperimentRunner,
) -> Result<ReportBuilder, Box<dyn std::error::Error>> {
    let dice_means = runner.means_for_ns("task1", "dice_means", &config::TASK1_NS, |n| {
        simulations::sample(n, simulations::dice)
    })?;

    let table_1a = Table::from_usize_f64_pairs(["n", "Średnia próbki"], &dice_means.rows);

    let convergence = runner.convergence("task1", "dice_convergence", 100, 3.5, |n| {
        simulations::sample(n, simulations::dice)
    })?;

    let convergence_plot = pf.convergence_plot(
        "zad1b_kostka_zbieznosc",
        "Zbieżność średniej próbki dla rzutu kostką",
        &convergence.xs,
        &convergence.ys,
        convergence.reference_y,
    )?;

    let coin_exp = runner.sample_experiment("task1", "coin_heads_100", config::TASK1_COIN_REPS, |_| {
        simulations::repeat(config::TASK1_COIN_REPS, || {
            simulations::experiment_sum(config::TASK1_COIN_THROWS, simulations::coin)
        })
    })?;

    let (coin_hist, coin_box) = pf.hist_and_box(
        "zad1c_moneta",
        "Liczba orłów w 100 rzutach monetą",
        &coin_exp.data,
    )?;

    Ok(
        report
            .section("Zadanie 1(a): średnia z n rzutów kostką", |s| {
                s.text("W tej części zasymulowano n-krotny rzut uczciwą sześcienną kostką dla różnych wartości n oraz wyznaczono średnią próbki.")
                    .table(table_1a)
                    .conclusion("Dla małych wartości n średnia próbki może się wyraźnie wahać, natomiast przy większych próbach powinna stabilizować się wokół wartości oczekiwanej.");
            })
            .section("Zadanie 1(b): wykres zależności średniej próbki od n", |s| {
                s.text("Na wykresie pokazano, jak zmienia się średnia próbki w zależności od liczby rzutów. Pozioma linia odniesienia odpowiada teoretycznej wartości oczekiwanej dla uczciwej kostki, czyli 3.5.")
                    .plot(convergence_plot)
                    .conclusion("Wraz ze wzrostem liczby rzutów średnia próbki zbliża się do 3.5, co jest zgodne z prawem wielkich liczb.");
            })
            .section("Zadanie 1(c): liczba orłów w 100 rzutach monetą", |s| {
                s.text("W tej części 200 razy powtórzono eksperyment polegający na wykonaniu 100 rzutów uczciwą monetą. W każdym powtórzeniu zliczano liczbę uzyskanych orłów.")
                    .variable_report(&coin_exp.stats, coin_hist, coin_box)
                    .note(&format!(
                        "Średnia liczba orłów w 100 rzutach w badaniu symulacyjnym wyniosła {:.2}.",
                        coin_exp.stats.mean
                    ))
                    .conclusion("Rozkład liczby orłów koncentruje się wokół wartości około 50, co odpowiada intuicji dla uczciwej monety.");
            })
    )
}