use std::fmt::Debug;
use std::str::FromStr;

pub fn resolve<T>(name: &str) -> T
where T: FromStr, <T as FromStr>::Err: Debug {
        std::env::var(name)
                .expect(&format!("{name} env var should be set"))
                .parse()
                .expect(&format!("{name}'s value should be parseable"))
}

pub fn resolve_opt<T>(name: &str) -> Option<T>
where T: FromStr, <T as FromStr>::Err: Debug {
        std::env::var(name)
                .map_or(None, |var| Some(var.parse().expect(&format!("{name}'s value should be parseable"))))
}

pub fn resolve_or<T>(name: &str, default: T) -> T
where T: FromStr, <T as FromStr>::Err: Debug {
        std::env::var(name)
                .map_or(default, |var| var.parse().expect(&format!("{name}'s value should be parseable")))
}

pub fn resolve_with<T, F>(name: &str, parser: F) -> T
where T: FromStr, <T as FromStr>::Err: Debug,
        F: FnOnce(String) -> T {
        std::env::var(name)
                .map(parser)
                .expect(&format!("{name} env var should be set"))
}

pub fn resolve_with_or<F, T: Clone>(name: &str, parser: F, default: T) -> T
where T: FromStr, <T as FromStr>::Err: Debug,
        F: FnOnce(String) -> T {
        std::env::var(name).map_or(default, parser)
}