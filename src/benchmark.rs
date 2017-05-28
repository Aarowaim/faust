macro_rules! benchmark {
	() => { 0u64 };
    ($b:block) => {{
    	use std::time::Instant;
    	let initial_t = Instant::now();

    	$b

    	let delta_t = initial_t.elapsed();
    	let ms_elapsed = (delta_t.as_secs() * 1_000) +
    						(delta_t.subsec_nanos() / 1_000_000) as u64;
    	ms_elapsed
    }}
}

macro_rules! benchmark_avg {
    ($e:expr, $b:block) => {{
    	let mut total_elapsed = 0u64;
    	for _ in 0..$e {
            total_elapsed += benchmark!{ $b };
        }
        total_elapsed as f64 / $e as f64
    }}
}
