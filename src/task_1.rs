use crate::analysis::{GroupComparison, PrettyStats, VariableReport};
use crate::math;
use crate::parser::{GradesDataset, Variable};
use crate::plot::{PlotFactory, PlotRef};
use crate::raport::{ReportBuilder, Table};
use crate::runner::ExperimentRunner;
use std::collections::HashMap;

fn fmt(x: f64) -> String {
    format!("{:.2}", x)
}

fn get_var<'a>(vars: &'a [Variable], name: &str) -> &'a Variable {
    vars.iter()
        .find(|v| v.name == name)
        .unwrap_or_else(|| panic!("Brak zmiennej {}", name))
}

fn slug(name: &str) -> String {
    match name {
        "ŚrOc" => "sroc".to_string(),
        "IQ" => "iq".to_string(),
        "PH" => "ph".to_string(),
        _ => name.to_lowercase(),
    }
}

fn overall_stats_table_from_reports(
    sroc: &VariableReport,
    iq: &VariableReport,
    ph: &VariableReport,
) -> Table {
    Table::from_string_rows(
        ["Statystyka", "ŚrOc", "IQ", "PH"],
        vec![
            ["Średnia".into(), fmt(sroc.stats.mean), fmt(iq.stats.mean), fmt(ph.stats.mean)],
            ["Mediana".into(), fmt(sroc.stats.median), fmt(iq.stats.median), fmt(ph.stats.median)],
            ["Minimum".into(), fmt(sroc.stats.min), fmt(iq.stats.min), fmt(ph.stats.min)],
            ["Maksimum".into(), fmt(sroc.stats.max), fmt(iq.stats.max), fmt(ph.stats.max)],
            ["Kwartyl dolny (Q1)".into(), fmt(sroc.stats.q1), fmt(iq.stats.q1), fmt(ph.stats.q1)],
            ["Kwartyl górny (Q3)".into(), fmt(sroc.stats.q3), fmt(iq.stats.q3), fmt(ph.stats.q3)],
            [
                "Wariancja (z próby)".into(),
                fmt(sroc.stats.variance_sample),
                fmt(iq.stats.variance_sample),
                fmt(ph.stats.variance_sample),
            ],
            [
                "Odchylenie standardowe".into(),
                fmt(sroc.stats.std_dev_sample),
                fmt(iq.stats.std_dev_sample),
                fmt(ph.stats.std_dev_sample),
            ],
            [
                "Współczynnik zmienności (CV)".into(),
                format!("{:.2}%", sroc.stats.cv_sample * 100.0),
                format!("{:.2}%", iq.stats.cv_sample * 100.0),
                format!("{:.2}%", ph.stats.cv_sample * 100.0),
            ],
        ],
    )
}

fn iq_bins_demo(
    pf: &PlotFactory,
    iq: &Variable,
) -> Result<Vec<PlotRef>, Box<dyn std::error::Error>> {
    let bins_opt = math::optimal_bins_sturges(iq.all.len());
    let bins_few = 3;
    let bins_many = 30;

    let few = pf.histogram(
        "grades_iq_bins_few",
        "Rozkład IQ (Zbyt mało klas)",
        &iq.all,
        bins_few,
    )?;

    let opt = pf.histogram(
        "grades_iq_bins_opt",
        "Rozkład IQ (Optymalnie)",
        &iq.all,
        bins_opt,
    )?;

    let many = pf.histogram(
        "grades_iq_bins_many",
        "Rozkład IQ (Zbyt dużo klas)",
        &iq.all,
        bins_many,
    )?;

    Ok(vec![few, opt, many])
}

pub fn append(
    report: ReportBuilder,
    pf: &PlotFactory,
    runner: &ExperimentRunner,
    dataset: &GradesDataset,
) -> Result<ReportBuilder, Box<dyn std::error::Error>> {
    let iq_var = get_var(&dataset.variables, "IQ");
    let sroc_var = get_var(&dataset.variables, "ŚrOc");
    let ph_var = get_var(&dataset.variables, "PH");

    let iq_overall = runner.cached_variable_report(
        pf,
        "grades",
        "iq_overall",
        "IQ",
        &iq_var.all,
    )?;
    let sroc_overall = runner.cached_variable_report(
        pf,
        "grades",
        "sroc_overall",
        "Średnia ocen (ŚrOc)",
        &sroc_var.all,
    )?;
    let ph_overall = runner.cached_variable_report(
        pf,
        "grades",
        "ph_overall",
        "Punktacja psychologiczna (PH)",
        &ph_var.all,
    )?;

    let iq_cmp: GroupComparison = runner.cached_group_comparison(
        pf,
        "grades",
        "iq_gender",
        "IQ",
        ("Chłopcy", &iq_var.boys),
        ("Dziewczęta", &iq_var.girls),
    )?;

    let sroc_cmp: GroupComparison = runner.cached_group_comparison(
        pf,
        "grades",
        "sroc_gender",
        "Średnia ocen (ŚrOc)",
        ("Chłopcy", &sroc_var.boys),
        ("Dziewczęta", &sroc_var.girls),
    )?;

    let ph_cmp: GroupComparison = runner.cached_group_comparison(
        pf,
        "grades",
        "ph_gender",
        "Punktacja psychologiczna (PH)",
        ("Chłopcy", &ph_var.boys),
        ("Dziewczęta", &ph_var.girls),
    )?;

    let overall_table = overall_stats_table_from_reports(&sroc_overall, &iq_overall, &ph_overall);

    let compare_plots = vec![
        iq_cmp.box_compare.clone(),
        sroc_cmp.box_compare.clone(),
        ph_cmp.box_compare.clone(),
    ];

    let iq_bins = iq_bins_demo(pf, iq_var)?;
    let bins_opt = math::optimal_bins_sturges(iq_var.all.len());

    let iq_boys = PrettyStats(math::summarize(&iq_var.boys));
    let iq_girls = PrettyStats(math::summarize(&iq_var.girls));
    let sroc_boys = PrettyStats(math::summarize(&sroc_var.boys));
    let sroc_girls = PrettyStats(math::summarize(&sroc_var.girls));
    let ph_boys = PrettyStats(math::summarize(&ph_var.boys));
    let ph_girls = PrettyStats(math::summarize(&ph_var.girls));

    Ok(
        report
            .section("Podstawowe statystyki opisowe (Zad. 1a)", |s| {
                s.text(&format!(
                    "Badana próba liczy <strong>{}</strong> uczniów siódmej klasy, w tym <strong>{}</strong> chłopców oraz <strong>{}</strong> dziewcząt. Poniższa tabela przedstawia podstawowe statystyki opisowe dla zmiennych ilościowych: średniej ocen (<strong>ŚrOc</strong>), ilorazu inteligencji (<strong>IQ</strong>) oraz punktacji w teście psychologicznym (<strong>PH</strong>).",
                    dataset.n_total, dataset.n_boys, dataset.n_girls
                ))
                    .table(overall_table)
                    .conclusion(&format!(
                        "Na podstawie przedstawionych statystyk można sformułować jedynie ogólne wnioski dotyczące rozkładu badanych zmiennych. Średni poziom <strong>IQ</strong> badanych uczniów wynosi <strong>{}</strong>, co jest wartością wyższą od średniej populacyjnej przyjmowanej na poziomie <strong>100</strong> punktów. Dla wszystkich trzech zmiennych (<strong>ŚrOc</strong>, <strong>IQ</strong> oraz <strong>PH</strong>) <strong>mediana jest wyższa od średniej</strong>, co może sugerować lekką asymetrię rozkładów i wpływ pojedynczych niższych wartości na obniżenie średniej. Analizując kwartyle można zauważyć, że połowa uczniów osiąga średnią ocen w przedziale od <strong>{}</strong> do <strong>{}</strong>, natomiast dla <strong>IQ</strong> połowa wyników mieści się w przedziale <strong>{}–{}</strong>. W przypadku zmiennej <strong>PH</strong> połowa uczniów uzyskała wyniki w przedziale <strong>{}–{}</strong>, co wskazuje na umiarkowane zróżnicowanie wyników w tej zmiennej.",
                        fmt(iq_overall.stats.mean),
                        fmt(sroc_overall.stats.q1),
                        fmt(sroc_overall.stats.q3),
                        fmt(iq_overall.stats.q1),
                        fmt(iq_overall.stats.q3),
                        fmt(ph_overall.stats.q1),
                        fmt(ph_overall.stats.q3),
                    ))
                    .page_break();
            })

            .section("Wizualizacja danych - histogramy i boxploty (Zad. 1b)", |s| {
                s.text("Poniższe wykresy przedstawiają rozkłady badanych zmiennych ilościowych oraz pozwalają ocenić ich skupienie, asymetrię i obecność obserwacji odstających.")
                    .subtitle("Zmienna: <strong>IQ</strong>")
                    .plot_pair(iq_overall.hist.clone(), iq_overall.boxplot.clone())
                    .subtitle("Zmienna: <strong>Średnia ocen (ŚrOc)</strong>")
                    .plot_pair(sroc_overall.hist.clone(), sroc_overall.boxplot.clone())
                    .subtitle("Zmienna: <strong>Punktacja psychologiczna (PH)</strong>")
                    .plot_pair(ph_overall.hist.clone(), ph_overall.boxplot.clone())
                    .page_break()
                    .conclusion("Na podstawie histogramów można zauważyć, że większość obserwacji dla wszystkich trzech zmiennych skupia się w określonych przedziałach wartości. W przypadku <strong>IQ</strong> największa liczba wyników znajduje się w przedziale około <strong>100–120</strong>, co wskazuje na stosunkowo skoncentrowany rozkład. Histogram dla zmiennej <strong>ŚrOc</strong> pokazuje, że większość uczniów osiąga średnie oceny w przedziale około <strong>6–9</strong>, natomiast niższe wartości pojawiają się znacznie rzadziej. W przypadku zmiennej <strong>PH</strong> największa liczba obserwacji mieści się w przedziale około <strong>50–70</strong> punktów, co wskazuje na skupienie wyników w tej części skali.<br><br>Wykresy ramkowe pokazują również obecność <strong>kilku obserwacji odstających</strong>, występujących głównie po stronie niższych wartości. Oznacza to, że w badanej próbie nie ma wyraźnych wartości skrajnie wysokich, pojawiają się natomiast pojedyncze jednostki uzyskujące wyniki niższe od większości grupy. Poza tym większość obserwacji znajduje się w stosunkowo wąskich przedziałach wartości, co sugeruje <strong>umiarkowaną spójność badanej próby</strong>.")
                    .page_break();
            })

            .section("Porównanie chłopców i dziewcząt (Zad. 1c)", |s| {
                s.subtitle("<strong>IQ</strong>")
                    .table(iq_cmp.table)
                    .plot_grid(iq_cmp.plots.clone(), 2)
                    .page_break()

                    .subtitle("<strong>Średnia ocen (ŚrOc)</strong>")
                    .table(sroc_cmp.table)
                    .plot_grid(sroc_cmp.plots.clone(), 2)
                    .page_break()

                    .subtitle("<strong>Punktacja psychologiczna (PH)</strong>")
                    .table(ph_cmp.table)
                    .plot_grid(ph_cmp.plots.clone(), 2)
                    .page_break()

                    .subtitle("<strong>Wykresy porównawcze</strong>")
                    .plot_grid(compare_plots, 1)
                    .page_break()

                    .conclusion(&format!(
                        "Na podstawie obliczonych statystyk oraz wykresów można zauważyć kilka różnic między grupą <strong>chłopców</strong> i <strong>dziewcząt</strong>. W przypadku <strong>IQ</strong> wyższe wartości średniej i mediany uzyskali <strong>chłopcy</strong> (średnia: <strong>{}</strong>, mediana: <strong>{}</strong>) niż <strong>dziewczęta</strong> (średnia: <strong>{}</strong>, mediana: <strong>{}</strong>). Jednocześnie u dziewcząt występuje <strong>większe zróżnicowanie wyników IQ</strong>, o czym świadczą wyższe wartości wariancji, odchylenia standardowego oraz współczynnika zmienności. Boxploty pokazują ponadto, że w obu grupach występują <strong>pojedyncze niższe obserwacje odstające</strong>, przy czym u dziewcząt rozkład jest bardziej rozproszony.<br><br>W przypadku <strong>średniej ocen (ŚrOc)</strong> sytuacja jest odwrotna - nieco wyższą średnią uzyskały <strong>dziewczęta</strong> (<strong>{}</strong>) niż <strong>chłopcy</strong> (<strong>{}</strong>). Mediany obu grup są jednak bardzo zbliżone (<strong>{}</strong> dla dziewcząt i <strong>{}</strong> dla chłopców), co oznacza, że typowy poziom ocen jest podobny, lecz w grupie chłopców pojawia się <strong>większy rozrzut wyników</strong>. Potwierdzają to wyraźnie większe wartości wariancji, odchylenia standardowego i współczynnika zmienności u chłopców. Dodatkowo u chłopców występuje bardzo niska wartość minimalna (<strong>{}</strong>), która może być obserwacją odstającą i obniżać średnią tej grupy.<br><br>W przypadku <strong>punktacji psychologicznej (PH)</strong> różnice między grupami są niewielkie. Chłopcy osiągnęli nieco wyższą średnią (<strong>{}</strong>) niż dziewczęta (<strong>{}</strong>), natomiast mediany są bardzo podobne (<strong>{}</strong> i <strong>{}</strong>). Zmienność wyników PH w obu grupach jest zbliżona, choć minimalnie większa u dziewcząt. Na wykresach ramkowych widać również pojedyncze niższe obserwacje odstające w obu grupach, ale ogólny rozkład punktacji psychologicznej jest podobny.<br><br>Podsumowując, <strong>chłopcy uzyskali wyższe wyniki IQ</strong>, <strong>dziewczęta osiągnęły nieco wyższą średnią ocen</strong>, natomiast w przypadku <strong>PH</strong> różnice są niewielkie. Największe różnice między grupami dotyczą więc <strong>poziomu IQ</strong> oraz <strong>zmienności wyników szkolnych</strong>.",
                        iq_boys.mean(),
                        iq_boys.median(),
                        iq_girls.mean(),
                        iq_girls.median(),
                        sroc_girls.mean(),
                        sroc_boys.mean(),
                        sroc_girls.median(),
                        sroc_boys.median(),
                        sroc_boys.min(),
                        ph_boys.mean(),
                        ph_girls.mean(),
                        ph_boys.median(),
                        ph_girls.median(),
                    ))
                    .page_break();
            })

            .section("Wpływ liczby klas na histogram (Zad. 1d)", |s| {
                s.subtitle(&format!(
                    "Zbyt mało klas (liczba klas: <strong>{}</strong>)",
                    3
                ))
                    .text("Zbyt mała liczba klas prowadzi do nadmiernego uproszczenia wykresu. Histogram traci szczegóły, przez co rzeczywisty kształt rozkładu staje się słabiej widoczny.")
                    .plot(iq_bins[0].clone())

                    .subtitle(&format!(
                        "Optymalna liczba klas (liczba klas: <strong>{}</strong>)",
                        bins_opt
                    ))
                    .text("W tej wersji wykorzystano liczbę klas wyznaczoną na podstawie <strong>reguły Sturgesa</strong>. Taki dobór pozwala zachować równowagę pomiędzy czytelnością wykresu a ilością informacji o rozkładzie danych.")
                    .plot(iq_bins[1].clone())
                    .page_break()

                    .subtitle(&format!(
                        "Zbyt dużo klas (liczba klas: <strong>{}</strong>)",
                        30
                    ))
                    .text("Zbyt duża liczba klas powoduje „poszarpanie” histogramu i zbyt dużą ilość słupków. Na wykresie pojawiają się przypadkowe wahania oraz puste luki, które utrudniają ocenę rzeczywistego rozkładu zmiennej.")
                    .plot(iq_bins[2].clone())

                    .conclusion("Wybór liczby klas ma istotny wpływ na interpretację histogramu. <strong>Zbyt mała liczba klas</strong> nadmiernie upraszcza obraz danych, natomiast <strong>zbyt duża</strong> eksponuje przypadkowy szum i utrudnia odczytanie ogólnych tendencji. Spośród przedstawionych wariantów najbardziej użyteczny jest histogram z liczbą klas wyznaczoną według <strong>reguły Sturgesa</strong>, ponieważ najlepiej zachowuje równowagę między przejrzystością wykresu a dokładnością odwzorowania rozkładu danych.")
                    .page_break();
            })
    )
}