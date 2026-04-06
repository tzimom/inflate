use std::fmt::Display;

const DAYS_PER_CYCLE: u32 = 365;
const SIMULATION_DAYS: u32 = 10;

const INITIAL_BALANCE: f64 = 100_000_000_000.;
const INITIAL_DAILY_BUDGET: f64 = 1000.;

const RATE_OF_INFLATION_PER_CYCLE: f64 = 1.08;
const RATE_OF_INTEREST_PER_CYCLE: f64 = 0.01;

struct SimulationOptions {
    daily_rate_of_inflation: f64,
    daily_rate_of_interest: f64,
}

struct SimulationState {
    balance: f64,
    daily_budget: f64,
    interest: f64,
    cycle: u32,
    day_in_cycle: u32,
}

fn simulate_day(state: &SimulationState, options: &SimulationOptions) -> SimulationState {
    let daily_budget = state.daily_budget * options.daily_rate_of_inflation;
    let mut balance = state.balance - daily_budget;
    let mut interest = state.interest + balance * options.daily_rate_of_interest;
    let mut day_in_cycle = state.day_in_cycle + 1;
    let mut cycle = state.cycle;

    if day_in_cycle == DAYS_PER_CYCLE {
        balance += interest;
        interest = 0.;
        day_in_cycle = 0;
        cycle += 1;
    }

    SimulationState { daily_budget, balance, interest, day_in_cycle, cycle }
}

impl Display for SimulationState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[CYCLE {}, DAY {}] Balance: {}, Daily budget: {}, Current interest: {}",
            self.cycle, self.day_in_cycle, self.balance, self.daily_budget, self.interest
        )
    }
}

fn main() {
    let options = SimulationOptions {
        daily_rate_of_inflation: RATE_OF_INFLATION_PER_CYCLE.powf(1. / f64::from(DAYS_PER_CYCLE)),
        daily_rate_of_interest: RATE_OF_INTEREST_PER_CYCLE / f64::from(DAYS_PER_CYCLE),
    };

    let mut state = SimulationState {
        balance: INITIAL_BALANCE,
        daily_budget: INITIAL_DAILY_BUDGET,
        interest: 0.,
        cycle: 0,
        day_in_cycle: 0,
    };

    println!("{}", state);

    for _ in 0..SIMULATION_DAYS {
        state = simulate_day(&state, &options);
        println!("{}", state);

        if state.balance <= 0. {
            println!("You went broke in cycle {} on day {}", state.cycle, state.day_in_cycle);
            break;
        }
    }
}
