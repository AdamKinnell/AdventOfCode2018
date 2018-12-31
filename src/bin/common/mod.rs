
/*
 Read lines from a file.
*/
pub fn to_lines(raw: &str) -> Vec<String> {
    raw.lines()
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
            let raw = std::fs::read_to_string(path).unwrap();
            let lines = to_lines(&raw);
            $f(&raw, &lines);
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
            let raw = std::fs::read_to_string(path).unwrap();
            let lines = to_lines(&raw);

            c.bench_function("benchmark", move |b| {
                b.iter(|| {
                    $f(&raw, &lines)
                })
            });
        }

        criterion_group!{
            name = benches;
            config = Criterion::default()
                .warm_up_time(std::time::Duration::new(2,0))
                .sample_size(10);
            targets = criterion_benchmark
        }
        criterion_main!(benches);
    }
}