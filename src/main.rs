use chi_squared::Mode;

fn main() {
    // code in src/lib.rs
    match Mode::get_mode() {
        Mode::OE => chi_squared::observed_expected(),
        Mode::Binomial => chi_squared::binomial(),
        Mode::Poisson => chi_squared::poission(),
        Mode::ContingencyTable => chi_squared::contingency_table()
    }
}   
