mod gen_code;

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

struct LbfgsAnswer {
    gradfxs_preconditioned: Vec<f64>,
    updated_lbfgs_info: LbfgsParams,
}

struct FnEvaled {
    f: f64,
    gradf: Vec<f64>,
    obj_engs: Vec<f64>,
    constr_engs: Vec<f64>,
}

#[derive(Clone)]
struct Params {
    opt_status: OptStatus,
    weight: f64,
    uo_round: i32,
    last_uo_state: Option<Vec<f64>>,
    last_uo_energy: Option<f64>,
    last_obj_energies: Option<Vec<f64>>,
    last_constr_energies: Option<Vec<f64>>,

    ep_round: i32,
    last_ep_state: Option<Vec<f64>>,
    last_ep_energy: Option<f64>,

    last_gradient: Vec<f64>,
    last_gradient_preconditioned: Vec<f64>,

    lbfgs_info: LbfgsParams,
}

struct State {
    varying_values: Vec<f64>,
    params: Params,
}

struct OptInfo {
    xs: Vec<f64>,
    energy_val: f64,
    obj_engs: Vec<f64>,
    constr_engs: Vec<f64>,
    norm_grad: f64,
    new_lbfgs_info: LbfgsParams,
    gradient: Vec<f64>,
    gradient_preconditioned: Vec<f64>,
    failed: bool,
}

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

const WEIGHT_GROWTH_FACTOR: f64 = 10.0;

const EP_STOP: f64 = 1e-3;

const EPSD: f64 = 1e-11;

const UO_STOP: f64 = 1e-2;

fn norm_list(xs: &[f64]) -> f64 {
    let sum_squares: f64 = xs.iter().map(|e| e * e).sum();
    sum_squares.sqrt()
}

fn scalev(c: f64, xs: &[f64]) -> Vec<f64> {
    xs.iter().map(|x| c * x).collect()
}

fn addv(xs: &[f64], ys: &[f64]) -> Vec<f64> {
    xs.iter().zip(ys).map(|(x, y)| x + y).collect()
}

fn subv(xs: &[f64], ys: &[f64]) -> Vec<f64> {
    xs.iter().zip(ys).map(|(x, y)| x - y).collect()
}

fn negv(xs: &[f64]) -> Vec<f64> {
    xs.iter().map(|e| -e).collect()
}

fn dot(xs: &[f64], ys: &[f64]) -> f64 {
    xs.iter().zip(ys).map(|(x, y)| x * y).sum()
}

fn unconstrained_converged(norm_grad: f64) -> bool {
    norm_grad < UO_STOP
}

fn objective_and_gradient(weight: f64, xs: &[f64]) -> FnEvaled {
    let mut inputs = [0.0; gen_code::LEN_GRADIENT];
    inputs[..gen_code::LEN_INPUTS].copy_from_slice(xs);
    inputs[gen_code::LEN_INPUTS] = weight;
    let mut gradient = [0.0; gen_code::LEN_GRADIENT];
    let mut secondary = [0.0; gen_code::LEN_SECONDARY];
    let f = gen_code::f(&inputs, &mut gradient, &mut secondary);
    FnEvaled {
        f,
        gradf: gradient
            .into_iter()
            .zip(gen_code::INPUTS)
            .map(|(x, meta)| {
                if meta == gen_code::InputMeta::Sampler {
                    x
                } else {
                    0.0
                }
            })
            .collect(),
        obj_engs: secondary[..gen_code::LEN_OBJ_ENGS].to_vec(),
        constr_engs: secondary[gen_code::LEN_OBJ_ENGS..].to_vec(),
    }
}

fn ep_converged(xs0: &[f64], xs1: &[f64], fxs0: f64, fxs1: f64) -> bool {
    let state_change = norm_list(&subv(xs1, xs0));
    let energy_change = (fxs1 - fxs0).abs();
    println!(
        "epConverged?: stateChange: {} | energyChange: {}",
        state_change, energy_change
    );

    state_change < EP_STOP || energy_change < EP_STOP
}

fn step(state: State, steps: i32) -> State {
    let mut opt_params = state.params.clone();
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

            let OptInfo {
                energy_val,
                norm_grad,
                new_lbfgs_info,
                gradient,
                gradient_preconditioned,
                failed,
                obj_engs,
                constr_engs,
                ..
            } = res;

            opt_params.last_uo_state = Some(xs.clone());
            opt_params.last_uo_energy = Some(energy_val);
            opt_params.uo_round = opt_params.uo_round + 1;
            opt_params.lbfgs_info = new_lbfgs_info;
            opt_params.last_gradient = gradient;
            opt_params.last_gradient_preconditioned = gradient_preconditioned;
            opt_params.last_constr_energies = Some(constr_engs);
            opt_params.last_obj_energies = Some(obj_engs);

            if unconstrained_converged(norm_grad) {
                opt_params.opt_status = OptStatus::UnconstrainedConverged;
                opt_params.lbfgs_info = DEFAULT_LBFGS_PARAMS;
                println!(
                    "Unconstrained converged with energy {} gradient norm {}",
                    energy_val, norm_grad
                );
            } else {
                opt_params.opt_status = OptStatus::UnconstrainedRunning;
                println!(
                    "Took {} steps. Current energy {} gradient norm {}",
                    steps, energy_val, norm_grad
                );
            }
            if failed {
                eprintln!("Error detected after stepping");
                opt_params.opt_status = OptStatus::Error;
                return State {
                    params: opt_params,
                    ..state
                };
            }
        }
        OptStatus::UnconstrainedConverged => {
            if opt_params.ep_round > 1
                && ep_converged(
                    &opt_params.last_ep_state.unwrap(),
                    &opt_params.last_uo_state.as_ref().unwrap(),
                    opt_params.last_ep_energy.unwrap(),
                    opt_params.last_uo_energy.unwrap(),
                )
            {
                opt_params.opt_status = OptStatus::EPConverged;
                println!(
                    "EP converged with energy {}",
                    opt_params.last_uo_energy.unwrap()
                );
            } else {
                println!("step: UO converged but EP did not converge; starting next round");
                opt_params.opt_status = OptStatus::UnconstrainedRunning;

                opt_params.weight = WEIGHT_GROWTH_FACTOR * weight;
                opt_params.ep_round = opt_params.ep_round + 1;
                opt_params.uo_round = 0;

                println!(
                    "increased EP weight to {} in compiled energy and gradient",
                    opt_params.weight
                );
            }

            opt_params.last_ep_state = opt_params.last_uo_state.clone();
            opt_params.last_ep_energy = opt_params.last_uo_energy;
        }
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

fn aw_line_search(xs0: &[f64], weight: f64, gradfxs0: &[f64], fxs0: f64) -> f64 {
    let max_steps = 10;

    let descent_dir = negv(gradfxs0);

    let duf_at_x0 = dot(&descent_dir, &objective_and_gradient(weight, xs0).gradf);
    let min_interval = 10e-10;

    let c1 = 0.001;
    let c2 = 0.9;

    let armijo = |ti: f64, objective: f64| -> bool {
        let cond1 = objective;
        let cond2 = fxs0 + c1 * ti * duf_at_x0;
        cond1 <= cond2
    };

    let weak_wolfe = |ti: f64, gradient: &[f64]| {
        let cond1 = dot(&descent_dir, gradient);
        let cond2 = c2 * duf_at_x0;
        cond1 >= cond2
    };

    let wolfe = weak_wolfe;

    let should_stop = |num_updates: i32, ai: f64, bi: f64, t: f64| {
        let interval_too_small = (ai - bi).abs() < min_interval;
        let too_many_steps = num_updates > max_steps;

        let need_to_stop = interval_too_small || too_many_steps;

        need_to_stop
    };

    let mut a = 0.0;
    let mut b = f64::INFINITY;
    let mut t = 1.0;
    let mut i = 0;

    while !should_stop(i, a, b, t) {
        let FnEvaled {
            f: obj,
            gradf: grad,
            ..
        } = objective_and_gradient(weight, &addv(xs0, &scalev(t, &descent_dir)));
        let is_armijo = armijo(t, obj);
        let is_wolfe = wolfe(t, &grad);

        if !is_armijo {
            b = t;
        } else if !is_wolfe {
            a = t;
        } else {
            break;
        }

        if b < f64::INFINITY {
            t = (a + b) / 2.0;
        } else {
            t = 2.0 * a;
        }

        i += 1;
    }

    t
}

fn lbfgs_inner(grad_fx_k: Matrix, ss: &[Matrix], ys: &[Matrix]) -> Matrix {
    todo!()
}

fn lbfgs(xs: &[f64], grad_fxs: &[f64], lbfgs_info: LbfgsParams) -> LbfgsAnswer {
    todo!()
}

fn minimize(xs0: &[f64], weight: f64, lbfgs_info: LbfgsParams, num_steps: i32) -> OptInfo {
    println!("-------------------------------------");
    println!("minimize, num steps, {}", num_steps);

    let min_steps = 1;
    if num_steps < min_steps {
        panic!("must step at least {} times in the optimizer", min_steps);
    }

    let mut xs = xs0.to_vec();
    let mut fxs = 0.0;
    let mut gradfxs = vec![0.0; gen_code::LEN_INPUTS];
    let mut gradient_preconditioned = vec![0.0; gen_code::LEN_INPUTS];
    let mut norm_grad_fxs = 0.0;
    let mut i = 0;
    let mut t = 0.0001;
    let mut failed = false;

    let mut obj_engs = vec![];
    let mut constr_engs = vec![];

    let mut new_lbfgs_info = lbfgs_info.clone();

    while i < num_steps {
        todo!();
    }

    return OptInfo {
        xs,
        energy_val: fxs,
        norm_grad: norm_grad_fxs,
        new_lbfgs_info,
        gradient: gradfxs,
        gradient_preconditioned,
        failed,
        obj_engs,
        constr_engs,
    };
}

fn gen_opt_problem() -> Params {
    let weight = INIT_CONSTRAINT_WEIGHT;

    Params {
        last_gradient: vec![0.0; gen_code::LEN_INPUTS],
        last_gradient_preconditioned: vec![0.0; gen_code::LEN_INPUTS],

        weight,
        uo_round: 0,
        ep_round: 0,
        opt_status: OptStatus::UnconstrainedRunning,

        lbfgs_info: DEFAULT_LBFGS_PARAMS,

        last_uo_state: None,
        last_uo_energy: None,
        last_obj_energies: None,
        last_constr_energies: None,

        last_ep_state: None,
        last_ep_energy: None,
    }
}

fn contains_nan(number_list: &[f64]) -> bool {
    number_list.iter().any(|n| n.is_nan())
}

fn step_until_convergence(state: State, num_steps: i32) -> Option<State> {
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

fn state_converged(state: &State) -> bool {
    state.params.opt_status == OptStatus::EPConverged
}

fn main() {
    let initial_state = State {
        varying_values: gen_code::VARYING_VALUES.to_vec(),
        params: gen_opt_problem(),
    };
    let optimized_state = step_until_convergence(initial_state, 10000).unwrap();
    println!("{:#?}", optimized_state.varying_values);
}
