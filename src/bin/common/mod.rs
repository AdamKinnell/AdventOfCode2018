
// Input //////////////////////////////////////////////////////////////////////

/*
 Represents the input to a puzzle.
*/
pub struct Input {
    raw: String
}

impl Input {

    pub fn new(path: String) -> Input {
        let raw = std::fs::read_to_string(path)
            .unwrap();

        Input { raw:raw }
    }

    /*
     Get the raw input as a string.
    */
    #[allow(dead_code)]
    pub fn raw(&self) -> &String {
        &self.raw
    }

    /*
     Get the input as a series of lines.
    */
    #[allow(dead_code)]
    pub fn to_lines(&self) -> Vec<String> {
        self.raw.lines()
            .map(String::from)
            .collect()
    }
}

// Harness ////////////////////////////////////////////////////////////////////

#[allow(unused_macros)]
macro_rules! run_without_benchmark {
    ($d:expr, $f:expr) => {
        fn main() {
            let path = ["res/input/", $d, ".txt"].join("");
            let input = common::Input::new(path);

            $f(&input);
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
            let input = common::Input::new(path);

            c.bench_function("benchmark", move |b| {
                b.iter(|| {
                    $f(&input);
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