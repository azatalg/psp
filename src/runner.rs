use crate::experiments;
use crate::io;
use crate::analysis::{GroupComparison, VariableReport};
use crate::plot::PlotFactory;
#[derive(Debug, Clone)]
pub struct ExperimentRunner {
    pub seed: u64,
    pub use_cache: bool,
}

impl ExperimentRunner {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            use_cache: true,
        }
    }

    pub fn without_cache(mut self) -> Self {
        self.use_cache = false;
        self
    }

    fn cached<T, F>(
        &self,
        category: &str,
        key: &str,
        compute: F,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
        F: FnOnce() -> T,
    {
        if self.use_cache {
            io::cache().under(category).load_or_compute(key, compute)
        } else {
            Ok(compute())
        }
    }

    pub fn sample_experiment<F>(
        &self,
        category: &str,
        key: &str,
        n: usize,
        sampler: F,
    ) -> Result<experiments::SampleExperimentResult, Box<dyn std::error::Error>>
    where
        F: Fn(usize) -> Vec<f64>,
    {
        let cache_key = format!("{}_n{}_seed{}", key, n, self.seed);
        self.cached(category, &cache_key, || {
            experiments::run_sample_experiment(n, sampler)
        })
    }

    pub fn means_for_ns<F>(
        &self,
        category: &str,
        key: &str,
        ns: &[usize],
        sampler: F,
    ) -> Result<experiments::MeansForNsResult, Box<dyn std::error::Error>>
    where
        F: Fn(usize) -> Vec<f64>,
    {
        let ns_tag = ns.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("-");
        let cache_key = format!("{}_ns{}_seed{}", key, ns_tag, self.seed);

        self.cached(category, &cache_key, || {
            experiments::run_means_for_ns(ns, sampler)
        })
    }

    pub fn convergence<F>(
        &self,
        category: &str,
        key: &str,
        max_n: usize,
        reference_y: f64,
        sampler: F,
    ) -> Result<experiments::ConvergenceResult, Box<dyn std::error::Error>>
    where
        F: Fn(usize) -> Vec<f64>,
    {
        let cache_key = format!("{}_maxn{}_seed{}", key, max_n, self.seed);

        self.cached(category, &cache_key, || {
            experiments::run_convergence_experiment(max_n, reference_y, sampler)
        })
    }

    pub fn clt<F>(
        &self,
        category: &str,
        key: &str,
        n: usize,
        reps: usize,
        mu: f64,
        sigma: f64,
        sampler: F,
    ) -> Result<experiments::CltResult, Box<dyn std::error::Error>>
    where
        F: Fn(usize) -> Vec<f64>,
    {
        let cache_key = format!("{}_n{}_reps{}_seed{}", key, n, reps, self.seed);

        self.cached(category, &cache_key, || {
            experiments::run_clt_experiment(n, reps, mu, sigma, sampler)
        })
    }

    pub fn population_sample<F>(
        &self,
        category: &str,
        key: &str,
        population: &[f64],
        n: usize,
        sampler: F,
    ) -> Result<experiments::PopulationSamplingResult, Box<dyn std::error::Error>>
    where
        F: Fn(&[f64], usize) -> Vec<f64>,
    {
        let cache_key = format!("{}_n{}_seed{}", key, n, self.seed);

        self.cached(category, &cache_key, || {
            experiments::run_population_sampling_experiment(population, n, sampler)
        })
    }

    pub fn population_clt<F>(
        &self,
        category: &str,
        key: &str,
        population: &[f64],
        n: usize,
        reps: usize,
        sampler: F,
    ) -> Result<experiments::CltResult, Box<dyn std::error::Error>>
    where
        F: Fn(&[f64], usize) -> Vec<f64>,
    {
        let cache_key = format!("{}_n{}_reps{}_seed{}", key, n, reps, self.seed);

        self.cached(category, &cache_key, || {
            experiments::run_population_clt_experiment(population, n, reps, sampler)
        })
    }

    pub fn cached_stats(
        &self,
        category: &str,
        key: &str,
        data: &[f64],
    ) -> Result<crate::math::SummaryStats, Box<dyn std::error::Error>> {
        let cache_key = format!("{}_seed{}", key, self.seed);
        self.cached(category, &cache_key, || crate::math::summarize(data))
    }

    pub fn cached_variable_report(
        &self,
        pf: &PlotFactory,
        category: &str,
        key: &str,
        display_name: &str,
        data: &[f64],
    ) -> Result<VariableReport, Box<dyn std::error::Error>> {
        let stats_key = format!("{}_stats_seed{}", key, self.seed);
        let stats = self.cached(category, &stats_key, || crate::math::summarize(data))?;

        Ok(VariableReport {
            name: display_name.to_string(),
            stats,
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

    pub fn cached_group_comparison(
        &self,
        pf: &PlotFactory,
        category: &str,
        key: &str,
        display_name: &str,
        g1: (&str, &[f64]),
        g2: (&str, &[f64]),
    ) -> Result<GroupComparison, Box<dyn std::error::Error>> {
        let s1_key = format!("{}_{}_stats_seed{}", key, g1.0, self.seed);
        let s2_key = format!("{}_{}_stats_seed{}", key, g2.0, self.seed);

        let s1 = self.cached(category, &s1_key, || crate::math::summarize(g1.1))?;
        let s2 = self.cached(category, &s2_key, || crate::math::summarize(g2.1))?;

        let p1_hist = pf.histogram_auto(
            &format!("{}_{}_hist", key, g1.0.to_lowercase()),
            &format!("Rozkład {} - {}", display_name, g1.0),
            g1.1,
        )?;
        let p1_box = pf.boxplot_single(
            &format!("{}_{}_box", key, g1.0.to_lowercase()),
            &format!("Wykres ramkowy {} - {}", display_name, g1.0),
            g1.0,
            g1.1,
        )?;

        let p2_hist = pf.histogram_auto(
            &format!("{}_{}_hist", key, g2.0.to_lowercase()),
            &format!("Rozkład {} - {}", display_name, g2.0),
            g2.1,
        )?;
        let p2_box = pf.boxplot_single(
            &format!("{}_{}_box", key, g2.0.to_lowercase()),
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
            table: crate::raport::Table::compare_two_groups(g1.0, g2.0, &s1, &s2),
            plots: vec![p1_hist, p1_box, p2_hist, p2_box],
            box_compare,
        })
    }
}