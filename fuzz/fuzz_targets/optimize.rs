#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ignore_err_on_multiple_installs = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global();

    let opts = oxipng::Options::default();
    let _ = oxipng::optimize_from_memory(data, &opts);
});
