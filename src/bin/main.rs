use cda_cache_sim::user_input::all_user_input;
use cda_cache_sim::cache::Cache;

fn main() {
    let ui = all_user_input();
    let mut c = Cache::new(&ui);
    let sim_results = c.simulate_trace_file("trace_files/gcc.trace");
    dbg!(sim_results);
}
