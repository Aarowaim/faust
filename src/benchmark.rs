macro_rules! benchmark {
    ($($s:stmt);*;) => {
    	println!("[BENCHMARK] Begin.");
    	use std::time::Instant;
    	let initial_t = Instant::now();

    	$($s;)*;

    	let delta_t = initial_t.elapsed();
    	let ms_elapsed = (delta_t.as_secs() * 1_000) +
    						(delta_t.subsec_nanos() / 1_000_000) as u64;
    	println!("[BENCHMARK] Complete. {} milliseconds", $name, ms_elapsed);
    }
}

macro_rules! named_benchmark {
    ($name:expr, $($s:stmt);*;) => {
    	println!("[BENCHMARK: {}] Begin.", $name);
    	use std::time::Instant;
    	let initial_t = Instant::now();

    	$($s;)*;

    	let delta_t = initial_t.elapsed();
    	let ms_elapsed = (delta_t.as_secs() * 1_000) +
    						(delta_t.subsec_nanos() / 1_000_000) as u64;
    	println!("[BENCHMARK: {}] Complete. {} milliseconds", $name, ms_elapsed);
    }
}