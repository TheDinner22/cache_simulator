use cda_cache_sim::user_input::all_user_input;
use cda_cache_sim::cache::Cache;

use plotly::{Plot, Scatter};

fn main() {
    const FILE_PATH: &str = "trace_files/gcc.trace";

    let ui = all_user_input();
    let mut c = Cache::new(&ui);
    let sim_results = c.simulate_trace_file(FILE_PATH);

    println!("hits: {}, accesses: {}, hit rate: {}", sim_results.hits, sim_results.accesses, sim_results.hits as f64 / sim_results.accesses as f64);

    // example plot
    // this isn't really what the assignment asks for, but is a good example of how to use plotly
    // (if you feel like it)
    //
    // you could also run the simulator a bunch of times and move the data to a google sheet to
    // generate the graphs
    let mut plot = Plot::new();
    let trace = Scatter::new(sim_results.accesses_history, sim_results.hit_history);
    plot.add_trace(trace);

    plot.show();
}
