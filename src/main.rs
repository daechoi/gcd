use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(long)]
    m: i64,
    #[arg(long)]
    n: i64,
}

fn gcd(m: i64, n: i64) -> i64 {
    let r = m % n;
    if r == 0 {
        return n;
    }
    if r == n {
        gcd(n, m)
    } else {
        gcd(n, r)
    }
}

fn main() {
    let cli = Cli::parse();
    println!(
        "The GCD of {} and {} is {}",
        cli.m,
        cli.n,
        gcd(cli.m, cli.n)
    );
}
