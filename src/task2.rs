use crate::config;
use crate::plot::{PlotFactory, PlotRef};
use crate::raport::{ReportBuilder, Table};
use crate::runner::ExperimentRunner;
use crate::simulations;

fn fmt(x: f64) -> String {
    format!("{:.4}", x)
}

pub fn append(
    report: ReportBuilder,
    pf: &PlotFactory,
    runner: &ExperimentRunner,
) -> Result<ReportBuilder, Box<dyn std::error::Error>> {
    let exp_table = config::TASK2_NS.iter().fold(
        Table::new(["n", "Średnia próbki", "μ_n", "σ²_n"]),
        |table, &n| {
            let exp_sample = runner.sample_experiment("task2", "exp_sample", n, |k| {
                simulations::sample(k, || simulations::exp(2.0))
            }).expect("exp sample failed");

            table.row([
                n.to_string(),
                fmt(exp_sample.stats.mean),
                fmt(0.5),
                fmt(1.0 / (4.0 * n as f64)),
            ])
        },
    );

    let exp_plots: Vec<PlotRef> = config::TASK2_NS
        .iter()
        .map(|&n| {
            let clt = runner.clt(
                "task2",
                "exp_clt",
                n,
                config::TASK2_REPS,
                0.5,
                0.5,
                |k| simulations::sample(k, || simulations::exp(2.0)),
            )?;

            pf.clt_hist(
                &format!("zad2b_exp_n{}", n),
                &format!("Exp(2): histogram standaryzowanej średniej, n={}", n),
                &clt.normalized,
            )
        })
        .collect::<Result<_, _>>()?;

    let pois_table = config::TASK2_NS.iter().fold(
        Table::new(["n", "Średnia próbki", "μ_n", "σ²_n"]),
        |table, &n| {
            let pois_sample = runner.sample_experiment("task2", "pois_sample", n, |k| {
                simulations::sample(k, || simulations::poisson(2.0))
            }).expect("pois sample failed");

            table.row([
                n.to_string(),
                fmt(pois_sample.stats.mean),
                fmt(2.0),
                fmt(2.0 / n as f64),
            ])
        },
    );

    let pois_plots: Vec<PlotRef> = config::TASK2_NS
        .iter()
        .map(|&n| {
            let clt = runner.clt(
                "task2",
                "pois_clt",
                n,
                config::TASK2_REPS,
                2.0,
                2.0f64.sqrt(),
                |k| simulations::sample(k, || simulations::poisson(2.0)),
            )?;

            pf.clt_hist(
                &format!("zad2c_pois_n{}", n),
                &format!("Pois(2): histogram standaryzowanej średniej, n={}", n),
                &clt.normalized,
            )
        })
        .collect::<Result<_, _>>()?;

    Ok(
        report
            .section("Zadanie 2(a): średnie próbki dla Exp(2)", |s| {
                s.text("Wygenerowano próbki z rozkładu wykładniczego Exp(2) dla n = 5, 10, 50, 100 i obliczono średnią próbki. Dla tego rozkładu wartość oczekiwana wynosi 1/2, a wariancja średniej próbki wynosi 1/(4n).")
                    .table(exp_table)
                    .conclusion("Empiryczne średnie są zbliżone do wartości teoretycznej 0.5, a wariancja średniej maleje wraz ze wzrostem liczności próby.");
            })
            .section("Zadanie 2(b): histogramy standaryzowanej średniej dla Exp(2)", |s| {
                s.text("Dla każdego n doświadczenie powtórzono 200 razy i narysowano histogram standaryzowanej średniej. Na wykres nałożono gęstość rozkładu normalnego N(0,1).")
                    .plot_grid(exp_plots, 2)
                    .conclusion("Wraz ze wzrostem n rozkład standaryzowanej średniej coraz lepiej przybliża rozkład normalny, co potwierdza centralne twierdzenie graniczne.");
            })
            .section("Zadanie 2(c): średnie próbki i histogramy dla Pois(2)", |s| {
                s.text("Powtórzono analizę z punktów (a) i (b) dla rozkładu Poissona z parametrem 2. W tym przypadku wartość oczekiwana wynosi 2, a wariancja średniej próbki wynosi 2/n.")
                    .table(pois_table)
                    .plot_grid(pois_plots, 2)
                    .conclusion("Również dla rozkładu Poissona standaryzowana średnia zbiega rozkładem do N(0,1) wraz ze wzrostem n.");
            })
    )
}