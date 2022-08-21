mod gen_code;

use std::time::Instant;

use gen_code::Outputs;

use crate::gen_code::{LEN_GRADIENT, LEN_SECONDARY};

#[derive(Debug)]
pub struct FnEvaled {
    f: f64,
}

#[derive(Clone)]
struct Matrix;

#[derive(Clone, Copy, PartialEq)]
enum OptStatus {
    NewIter,
    UnconstrainedRunning,
    UnconstrainedConverged,
    EPConverged,
    Error,
}

#[derive(Clone)]
struct LbfgsParams {
    last_state: Option<Matrix>,
    last_grad: Option<Matrix>,
    s_list: Vec<Matrix>,
    y_list: Vec<Matrix>,
    num_unconstr_steps: i32,
    mem_size: i32,
}

#[derive(Clone)]
struct Params {
    opt_status: OptStatus,
    weight: f64,
    uo_round: i32,

    ep_round: i32,

    lbfgs_info: LbfgsParams,
}

struct State {
    varying_values: Vec<f64>,
    params: Params,
}

struct OptInfo {
    xs: Vec<f64>,
}

const OBJ_ENGS_LENGTH: usize = 1208 - 910;

const INIT_CONSTRAINT_WEIGHT: f64 = 10e-3;

const DEFAULT_LBFGS_MEM_SIZE: i32 = 17;

const DEFAULT_LBFGS_PARAMS: LbfgsParams = LbfgsParams {
    last_state: None,
    last_grad: None,
    s_list: vec![],
    y_list: vec![],
    num_unconstr_steps: 0,
    mem_size: DEFAULT_LBFGS_MEM_SIZE,
};

fn minimize(xs0: &[f64], weight: f64, lbfgs_info: LbfgsParams, num_steps: i32) -> OptInfo {
    println!("-------------------------------------");
    println!("minimize, num steps, {}", num_steps);

    let xs = xs0.to_vec();

    return OptInfo { xs };
}

fn step(state: State, steps: i32) -> State {
    let opt_params = state.params.clone();
    let Params {
        opt_status, weight, ..
    } = opt_params;
    let mut xs = state.varying_values.clone();

    println!("===============");
    println!(
        "step | weight: {} | EP round: {} | UO round: {}",
        weight, opt_params.ep_round, opt_params.uo_round
    );

    match opt_status {
        OptStatus::NewIter => {
            println!("step newIter, xs {:#?}", xs);

            return State {
                params: Params {
                    weight: INIT_CONSTRAINT_WEIGHT,
                    uo_round: 0,
                    ep_round: 0,
                    opt_status: OptStatus::UnconstrainedRunning,
                    lbfgs_info: DEFAULT_LBFGS_PARAMS,
                    ..state.params
                },
                ..state
            };
        }
        OptStatus::UnconstrainedRunning => {
            let res = minimize(&xs, state.params.weight, state.params.lbfgs_info, steps);
            xs = res.xs;
        }
        OptStatus::UnconstrainedConverged => todo!(),
        OptStatus::EPConverged => {
            println!("step: EP converged");
            return state;
        }
        OptStatus::Error => {
            eprintln!("step: Error");
            return state;
        }
    }

    State {
        varying_values: xs.clone(),
        params: opt_params,
        ..state
    }
}

fn state_converged(state: &State) -> bool {
    state.params.opt_status == OptStatus::EPConverged
}

fn step_until_convergence(state: State) -> Option<State> {
    let num_steps = 10000;

    let mut current_state = state;
    while current_state.params.opt_status != OptStatus::Error && !state_converged(&current_state) {
        current_state = step(current_state, num_steps);
    }
    if current_state.params.opt_status == OptStatus::Error {
        None
    } else {
        Some(current_state)
    }
}

fn gen_opt_problem() -> Params {
    let weight = INIT_CONSTRAINT_WEIGHT;

    Params {
        weight,
        uo_round: 0,
        ep_round: 0,
        opt_status: OptStatus::UnconstrainedRunning,

        lbfgs_info: DEFAULT_LBFGS_PARAMS,
    }
}

fn main() {
    let mut inputs = gen_code::VARYING_VALUES.to_vec();
    inputs.push(INIT_CONSTRAINT_WEIGHT);

    let mut gradient = vec![0f64; LEN_GRADIENT];
    let mut secondary = vec![0f64; LEN_SECONDARY];

    println!("start");
    for _ in 0..100000 {
        gen_code::f(&inputs, &mut gradient, &mut secondary);
    }
    println!("end");
}
