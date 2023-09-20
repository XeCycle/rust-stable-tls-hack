fn main() {
    tls::set_var(42);
    println!("{}", tls::get_var());
}
