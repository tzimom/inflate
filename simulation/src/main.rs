use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    #[arg(short = 'b', long, default_value_t = 1_000_000.)]
    balance: f64,

    #[arg(short = 'B', long, default_value_t = 50.)]
    daily_budget: f64,

    #[arg(short = 'C', long, default_value_t = 365)]
    cycle_length: u64,

    #[arg(short = 'i', long, default_value_t = 0.01)]
    rate_of_interest: f64,

    #[arg(short = 'I', long, default_value_t = 0.02)]
    rate_of_inflation: f64,

    #[arg(short = 'l', long, default_value_t = 0.)]
    overdraft_limit: f64,

    #[arg(short = 'T', long)]
    duration: Option<u64>,
}

struct SimulationOptions {
    cycle_length: u64,
    eff_rate_of_interest: f64,
    eff_rate_of_inflation: f64,
}

struct SimulationState {
    balance: f64,
    daily_budget: f64,
    interest: f64,
}

struct Simulation {
    options: SimulationOptions,
    state: SimulationState,
    time: u64,
}

impl From<&CliArgs> for Simulation {
    fn from(args: &CliArgs) -> Self {
        let options = SimulationOptions {
            cycle_length: args.cycle_length,
            eff_rate_of_interest: args.rate_of_interest / (args.cycle_length as f64),
            eff_rate_of_inflation: (1. + args.rate_of_inflation)
                .powf(1. / (args.cycle_length as f64))
                - 1.,
        };

        let state = SimulationState {
            balance: args.balance,
            daily_budget: args.daily_budget,
            interest: 0.,
        };

        Self {
            options,
            state,
            time: 0,
        }
    }
}

impl Simulation {
    pub fn simulate_step(&mut self) {
        if self.time % self.options.cycle_length == 0 {
            self.state.balance += self.state.interest;
            self.state.interest = 0.;
        }

        self.state.daily_budget *= self.options.eff_rate_of_inflation + 1.;
        self.state.balance -= self.state.daily_budget;
        self.state.interest += self.state.balance * self.options.eff_rate_of_interest;
        self.time += 1;
    }
}

fn main() {
    let args = CliArgs::parse();
    let mut simulation = Simulation::from(&args);

    println!("balance,daily_budget");

    loop {
        if args.duration.map_or(false, |limit| simulation.time >= limit) {
            break;
        }

        if simulation.state.balance <= 0. {
            break;
        }

        simulation.simulate_step();
        println!("{},{}", simulation.state.balance, simulation.state.daily_budget);
    }
}
