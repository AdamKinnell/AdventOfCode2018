
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

macro_rules! run {
    (input = $input:expr, run = $f_run:expr, bench = $f_bench:expr) => {

        fn main_run(input: &Input) {
            $f_run(&input);
        }

        fn main_bench(input: Input) {
            let mut criterion = criterion::Criterion::default()
                .warm_up_time(std::time::Duration::new(2,0))
                .measurement_time(std::time::Duration::new(5, 0))
                .sample_size(25)
                .configure_from_args();

            criterion.bench_function(file!(), move |b| {
                b.iter(|| {
                    $f_bench(&input);
                })
            });

            criterion.final_summary();
        }

        fn main() {

            // Setup
            let path = ["res/input/", $input, ".txt"].join("");
            let input = common::Input::new(path);

            println!("\n======== RUN ========\n");
            main_run(&input);

            println!("\n======== BENCH ========\n");
            main_bench(input);
        }
    }
}