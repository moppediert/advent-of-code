pub mod _2015;

use std::{collections::BTreeMap, env, fs, path::PathBuf};

fn main() {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    let mut path: PathBuf = [project_dir, "input"].iter().collect();

    let args = env::args().collect::<Vec<String>>();
    if args.len() < 4 {
        panic!("cmd year day problem");
    }

    let year = &args[1];
    let day = &args[2];
    let problem = &args[3];
    path.push(year);
    path.push(day);
    path.push(problem);
    let input = read_input(path);
    let solver_key = format!(
        "{}::_{}::_{}::_{}",
        env!("CARGO_CRATE_NAME"),
        year,
        day,
        problem
    );

    let mut solvers: BTreeMap<String, fn(String)> = BTreeMap::new();

    solvers.insert(fn_name(_2015::_1::_1).to_string(), _2015::_1::_1);
    solvers.insert(fn_name(_2015::_1::_2).to_string(), _2015::_1::_2);

    solvers.get(&solver_key).unwrap()(input);
}

fn read_input(path: PathBuf) -> String {
    let input: String = fs::read_to_string(path).unwrap();
    input
}

fn fn_name<F>(_: F) -> &'static str
where
    F: Fn(String),
{
    std::any::type_name::<F>()
}
