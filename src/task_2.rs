use crate::analysis::{GroupComparison, PrettyStats, VariableReport};
use crate::math;
use crate::parser::IncomeRecord;
use crate::plot::{PlotFactory, PlotRef};
use crate::raport::{ReportBuilder, Table};
use crate::runner::ExperimentRunner;

fn fmt(x: f64) -> String {
    format!("{:.2}", x)
}

fn extract_incomes(records: &[IncomeRecord]) -> Vec<f64> {
    records.iter().map(|r| r.income).collect()
}

fn education_label(code: usize) -> &'static str {
    match code {
        1 => "Podstawowe",
        2 => "Niepełne średnie",
        3 => "Średnie",
        4 => "Niepełne wyższe",
        5 => "Wyższe (licencjat)",
        6 => "Wyższe (magister)",
        _ => "Nieznane",
    }
}

fn education_table_with_cv(records: &[IncomeRecord]) -> Table {
    let mut rows: Vec<[String; 9]> = Vec::new();

    for edu in 1..=6 {
        let incomes: Vec<f64> = records
            .iter()
            .filter(|r| r.education == edu)
            .map(|r| r.income)
            .collect();

        let stats = math::summarize(&incomes);

        rows.push([
            education_label(edu).into(),
            fmt(stats.mean),
            fmt(stats.median),
            fmt(stats.min),
            fmt(stats.max),
            fmt(stats.q1),
            fmt(stats.q3),
            fmt(stats.std_dev_sample),
            format!("{:.2}%", stats.cv_sample * 100.0),
        ]);
    }

    Table::from_string_rows(
        [
            "Poziom wykształcenia",
            "Średnia",
            "Mediana",
            "Minimum",
            "Maksimum",
            "Q1",
            "Q3",
            "Odch. std.",
            "CV",
        ],
        rows,
    )
}

fn education_boxplot(
    pf: &PlotFactory,
    records: &[IncomeRecord],
) -> Result<PlotRef, Box<dyn std::error::Error>> {
    let groups: Vec<(&str, Vec<f64>)> = (1..=6)
        .map(|edu| {
            let vals: Vec<f64> = records
                .iter()
                .filter(|r| r.education == edu)
                .map(|r| r.income)
                .collect();
            (education_label(edu), vals)
        })
        .collect();

    let refs: Vec<(&str, &[f64])> = groups
        .iter()
        .map(|(label, vals)| (*label, vals.as_slice()))
        .collect();

    pf.boxplot_groups(
        "income_education_box",
        "Zarobki wg wykształcenia",
        &refs,
    )
}

pub fn append(
    report: ReportBuilder,
    pf: &PlotFactory,
    runner: &ExperimentRunner,
    records: &[IncomeRecord],
) -> Result<ReportBuilder, Box<dyn std::error::Error>> {
    let all_income = extract_incomes(records);

    let men_income: Vec<f64> = records
        .iter()
        .filter(|r| r.sex == 1)
        .map(|r| r.income)
        .collect();

    let women_income: Vec<f64> = records
        .iter()
        .filter(|r| r.sex == 2)
        .map(|r| r.income)
        .collect();

    // =========================
    // Cached variable reports
    // =========================
    let overall: VariableReport = runner.cached_variable_report(
        pf,
        "income",
        "overall_income",
        "Zarobki",
        &all_income,
    )?;

    let men: VariableReport = runner.cached_variable_report(
        pf,
        "income",
        "men_income",
        "Zarobki - mężczyźni",
        &men_income,
    )?;

    let women: VariableReport = runner.cached_variable_report(
        pf,
        "income",
        "women_income",
        "Zarobki - kobiety",
        &women_income,
    )?;

    // =========================
    // Cached comparison
    // =========================
    let gender_cmp: GroupComparison = runner.cached_group_comparison(
        pf,
        "income",
        "gender_income",
        "Zarobki",
        ("Mężczyźni", &men_income),
        ("Kobiety", &women_income),
    )?;

    let overall_stats = PrettyStats(overall.stats.clone());
    let men_stats = PrettyStats(men.stats.clone());
    let women_stats = PrettyStats(women.stats.clone());

    let overall_table = Table::from_stats(&overall.stats);
    let men_table = Table::from_stats(&men.stats);
    let women_table = Table::from_stats(&women.stats);

    let edu_table = education_table_with_cv(records);
    let edu_box = education_boxplot(pf, records)?;

    let mean_vs_median = Table::from_string_rows(
        ["Miara", "Wartość"],
        vec![
            ["Średnia".into(), overall_stats.mean()],
            ["Mediana".into(), overall_stats.median()],
        ],
    );

    Ok(
        report
            .section("Struktura zarobków mieszkańców USA w roku 2000 (Zad. 2a)", |s| {
                s.text(&format!(
                    "Badana próba zawiera <strong>{}</strong> obserwacji. Analizie poddano roczne zarobki respondentów w <strong>dolarach amerykańskich</strong>.",
                    all_income.len()
                ))
                    .table(overall_table)
                    .plot_pair(overall.hist.clone(), overall.boxplot.clone())
                    .conclusion("Rozkład zarobków ma charakter <strong>asymetryczny prawostronnie</strong>, czyli <strong>skośny prawostronnie</strong>, co oznacza, że większość osób osiąga relatywnie niższe dochody, natomiast niewielka grupa uzyskuje bardzo wysokie zarobki. W konsekwencji <strong>średnia jest większa od mediany</strong>, co wskazuje na wpływ wysokich wartości odstających. Histogram ujawnia koncentrację obserwacji w dolnych przedziałach oraz <strong>długi ogon po stronie wyższych dochodów</strong>. W danych mogą występować również <strong>ujemne wartości dochodów</strong>, które mogą wynikać np. ze strat działalności gospodarczej lub specyfiki sposobu raportowania dochodów.")
                    .page_break();
            })

            .section("Porównanie zarobków w zależności od poziomu wykształcenia (Zad. 2b)", |s| {
                s.text("W tej części porównano poziom zarobków w <strong>sześciu grupach wykształcenia</strong>: od wykształcenia podstawowego do wyższego magisterskiego.")
                    .table(edu_table)
                    .plot(edu_box)
                    .conclusion("Zarobki wykazują <strong>wyraźną zależność od poziomu wykształcenia</strong>. Wraz ze wzrostem poziomu edukacji rosną zarówno <strong>średnie</strong>, jak i <strong>mediany</strong> dochodów. Osoby z wykształceniem wyższym osiągają istotnie wyższe dochody niż osoby z niższym poziomem wykształcenia. Jednocześnie w każdej grupie widoczne jest <strong>znaczne zróżnicowanie</strong>, co oznacza, że wykształcenie nie jest jedynym czynnikiem wpływającym na poziom zarobków. W grupach o wyższym wykształceniu częściej występują również <strong>wartości ekstremalne</strong>, co wskazuje na większe możliwości osiągania bardzo wysokich dochodów.")
                    .page_break();
            })

            .section("Porównanie zarobków kobiet i mężczyzn (Zad. 2c)", |s| {
                s.subtitle("<strong>Mężczyźni</strong>")
                    .table(men_table)
                    .plot_pair(men.hist.clone(), men.boxplot.clone())
                    .page_break()

                    .subtitle("<strong>Kobiety</strong>")
                    .table(women_table)
                    .plot_pair(women.hist.clone(), women.boxplot.clone())
                    .page_break()

                    .subtitle("<strong>Wykres porównawczy</strong>")
                    .plot(gender_cmp.box_compare.clone())
                    .conclusion(&format!(
                        "Analiza danych wskazuje na różnice w poziomie zarobków między <strong>mężczyznami</strong> a <strong>kobietami</strong>. Mężczyźni osiągają wyższe wartości średnie i mediany (średnia: <strong>{}</strong>, mediana: <strong>{}</strong>) niż kobiety (średnia: <strong>{}</strong>, mediana: <strong>{}</strong>). Jednak na podstawie tych danych nie można jednoznacznie stwierdzić istnienia <strong>związku przyczynowego</strong>. Różnice te mogą wynikać z wielu czynników, takich jak <strong>wykształcenie</strong>, <strong>sektor zatrudnienia</strong> czy <strong>struktura rynku pracy</strong>. W związku z tym obserwujemy jedynie <strong>korelację</strong>, a nie dowód bezpośredniego wpływu płci na poziom dochodów.",
                        men_stats.mean(),
                        men_stats.median(),
                        women_stats.mean(),
                        women_stats.median(),
                    ))
                    .page_break();
            })

            .section("Przeciętny zarobek mieszkańców USA w roku 2000 (Zad. 2d)", |s| {
                s.text("W celu wyznaczenia „<strong>przeciętnego zarobku</strong>” można rozważyć różne miary tendencji centralnej. Najczęściej stosowane są <strong>średnia arytmetyczna</strong> oraz <strong>mediana</strong>.")
                    .table(mean_vs_median)
                    .conclusion("Ze względu na <strong>silną asymetrię rozkładu zarobków</strong> oraz obecność <strong>wartości odstających</strong>, średnia arytmetyczna nie jest najlepszą miarą „przeciętnego” dochodu. Znacznie lepszą miarą jest <strong>mediana</strong>, ponieważ jest odporna na wpływ skrajnych wartości. Mediana lepiej oddaje <strong>typowy poziom dochodu większości populacji</strong>, podczas gdy średnia może być zawyżona przez niewielką grupę osób o bardzo wysokich zarobkach.");
            })
    )
}