
/*
 Read lines from a file.
*/
pub fn get_input(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

/*
 Run
*/
#[allow(unused_macros)]
macro_rules! run_without_benchmark {
    ($d:expr, $f:expr) => {
        fn main() {
            let path = ["res/input/", $d, ".txt"].join("");
            let lines = get_input(&path);
            $f(&lines);
        }
    }
}

#[allow(unused_macros)]
macro_rules! run_with_benchmark {

    ($d:expr, $f:expr) => {
        #[macro_use]extern crate criterion;
        use criterion::Criterion;

        fn criterion_benchmark(c: &mut Criterion) {
            let path = ["res/input/", $d, ".txt"].join("");
            let lines = get_input(&path);
            c.bench_function("benchmark", move |b| {
                b.iter(|| {
                    $f(&lines)
                })
            });
        }

        criterion_group!{
            name = benches;
            config = Criterion::default()
                .warm_up_time(std::time::Duration::new(1,0))
                .sample_size(2);
            targets = criterion_benchmark
        }
        criterion_main!(benches);
    }
}