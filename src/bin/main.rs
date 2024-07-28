use cda_cache_sim::user_input::all_user_input;
use cda_cache_sim::cache::Cache;

use plotly::{Plot, Scatter};

fn main() {
    let ui = all_user_input();
    let mut c = Cache::new(&ui);
    let sim_results = c.simulate_trace_file("trace_files/gcc.trace");

    let mut plot = Plot::new();
    let trace = Scatter::new(sim_results.accesses_history, sim_results.hit_history);
    plot.add_trace(trace);

    plot.show();
}
