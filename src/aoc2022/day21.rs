use std::{collections::HashMap, hash::Hash, rc::Rc};

struct Action<'a> {
    lhs: &'a str,
    rhs: &'a str,
    op: fn(i64, i64) -> i64,
}

fn add(a: i64, b: i64) -> i64 { a + b }
fn sub(a: i64, b: i64) -> i64 { a - b }
fn mul(a: i64, b: i64) -> i64 { a * b }
fn div(a: i64, b: i64) -> i64 { a / b }

fn reverse_op(op: fn(i64, i64) -> i64) -> fn(i64, i64) -> i64 {
    if op == add {
        sub
    }
    else if op == sub {
        add
    }
    else if op == mul {
        div
    }
    else if op == div {
        mul
    }
    else{ panic!() }
}

fn is_associative(op: fn(i64, i64) -> i64) -> bool {
    if op == add || op == mul { true }
    else if op == sub || op == div { false }
    else { panic!() }
}

fn print_op(op: fn(i64, i64) -> i64) -> &'static str {
    if op == add {
        "+"
    }
    else if op == sub {
        "-"
    }
    else if op == mul {
        "*"
    }
    else if op == div {
        "/"
    }
    else{ panic!() }
}

fn resolve_monkey<'a, 'b>(name: &'a str, monkeys: &'b HashMap<&'a str, Action<'a>>, resolved: &'b mut HashMap<&'a str, i64>) -> i64 {
    if resolved.contains_key(name) {
       *&resolved[name]
    }
    else {
        let action = &monkeys[name];
        let lhs = resolve_monkey(action.lhs, monkeys, resolved);
        let rhs = resolve_monkey(action.rhs, monkeys, resolved);

        let result = (action.op)(lhs, rhs);
        resolved.insert(name, result);
        result
    }
}

fn create_initial_setup(input: &str) -> (HashMap<&str, Action>, HashMap<&str, i64>) {
    let mut monkeys = HashMap::new();
    let mut map = HashMap::new();
    for line in input.split("\n") {
        let name = &line[0..4];
        let rest = line.split(": ").skip(1).next().unwrap();
        if let Ok(num) = rest.parse() {
            map.insert(name, num);
        } 
        else {
            let lhs = &rest[..4];
            let rhs = &rest[(rest.len() - 4)..];
            let op = &rest[5..6];
            let func = match op {
                "+" => {
                    add
                },
                "-" => {
                    sub
                },
                "*" => {
                    mul
                },
                "/" => {
                    div
                },
                _ => unreachable!(),
            };
            let action = Action { lhs, rhs, op: func };
            monkeys.insert(name, action);
        }
    }
    (monkeys, map)
}

#[derive(Debug, Clone)]
struct HumnAction {
    lhs: HumnResult,
    rhs: HumnResult,
    op: fn(i64, i64) -> i64,
} 

impl HumnAction {
    fn resolve_literal(&self) -> Option<i64> {
        if let HumnResult::Literal(lhs) = self.lhs 
        && let HumnResult::Literal(rhs) = self.rhs 
        {
            Some((self.op)(lhs, rhs))
        } else { None }
    }

    fn resolve_with_humn_val(&self, humn_val: i64) -> i64 {
        let (lhs, rhs) = match (&self.lhs, &self.rhs) {
            (HumnResult::Literal(lhs), HumnResult::Literal(rhs)) => {
                (*lhs, *rhs)
            },
            (HumnResult::Dependent(act1), HumnResult::Dependent(act2)) => {
                (act1.resolve_with_humn_val(humn_val), act2.resolve_with_humn_val(humn_val))
            },

            (HumnResult::Dependent(act), HumnResult::Literal(rhs)) => {
                (act.resolve_with_humn_val(humn_val), *rhs)
            },
            (HumnResult::Dependent(act), HumnResult::Humn) => {
                (act.resolve_with_humn_val(humn_val), humn_val)
            },
            (HumnResult::Literal(lhs), HumnResult::Humn) => {
                (*lhs, humn_val)
            },

            (HumnResult::Literal(lhs), HumnResult::Dependent(act)) => {
                (*lhs, act.resolve_with_humn_val(humn_val))
            },
            (HumnResult::Humn, HumnResult::Dependent(act)) => {
                (humn_val, act.resolve_with_humn_val(humn_val))
            },
            (HumnResult::Humn, HumnResult::Literal(rhs)) => {
                (humn_val, *rhs)
            },

            _ => panic!("Illegal humn resolve state"),
        };
        (self.op)(lhs, rhs)
    }
}

#[derive(Debug, Clone)]
enum HumnResult {
    Literal(i64),
    Dependent(Rc<HumnAction>),
    Humn,
}

fn create_initial_setup_humn(input: &str) -> (HashMap<&str, Action>, HashMap<&str, Rc<HumnAction>>) {
    let mut monkeys = HashMap::new();
    let mut literals = HashMap::new();
    for line in input.split("\n") {
        let name = &line[0..4];
        let rest = line.split(": ").skip(1).next().unwrap();
        if name == "humn" {}
        else if let Ok(num) = rest.parse() {
            let action = HumnAction {
                lhs: HumnResult::Literal(num),
                rhs: HumnResult::Literal(0_i64),
                op: add,
            };

            literals.insert(name, Rc::new(action));
        } 
        else {
            let lhs = &rest[..4];
            let rhs = &rest[(rest.len() - 4)..];
            let op = &rest[5..6];
            let func = match op {
                "+" => {
                    add
                },
                "-" => {
                    sub
                },
                "*" => {
                    mul
                },
                "/" => {
                    div
                },
                _ => unreachable!(),
            };
            let action = Action { lhs, rhs, op: func };
            monkeys.insert(name, action);
        }
    }
    (monkeys, literals)
}

fn create_humn_chain<'a, 'b>(name: &'a str, monkeys: &'b HashMap<&'a str, Action<'a>>, 
    resolved: &'b mut HashMap<&'a str, Rc<HumnAction>>) -> Rc<HumnAction> {
        if resolved.contains_key(name) {
            resolved[name].clone()
        }
        else {
            let action = &monkeys[name];
            let lhs = if resolved.contains_key(action.lhs) {
                let depend = resolved[action.lhs].clone();
                if let Some(res) = depend.resolve_literal() {
                    HumnResult::Literal(res)
                }
                else {
                    HumnResult::Dependent(depend)
                }
            } else if action.lhs == "humn" {
                HumnResult::Humn
            } else {
                let res = create_humn_chain(action.lhs, monkeys, resolved);
                if let Some(res) = res.resolve_literal() {
                    HumnResult::Literal(res)
                }
                else {
                    HumnResult::Dependent(res)
                }
            };

            let rhs = if resolved.contains_key(action.rhs) {
                let depend = resolved[action.rhs].clone();
                if let Some(res) = depend.resolve_literal() {
                    HumnResult::Literal(res)
                }
                else {
                    HumnResult::Dependent(depend)
                }
            } else if action.rhs == "humn" {
                HumnResult::Humn
            } else {
                let res = create_humn_chain(action.rhs, monkeys, resolved);
                if let Some(res) = res.resolve_literal() {
                    HumnResult::Literal(res)
                }
                else {
                    HumnResult::Dependent(res)
                }
            };

            let rc = Rc::new(HumnAction {
                lhs,
                rhs,
                op: action.op.clone(),
            });
            resolved.insert(name, rc.clone());
            rc
        }
}

fn is_tree_solvable(root: &Rc<HumnAction>) -> bool {
    match (&root.lhs, &root.rhs) {
        (HumnResult::Dependent(new_root), HumnResult::Literal(_))
        | (HumnResult::Literal(_), HumnResult::Dependent(new_root)) => {
            is_tree_solvable(new_root)
        },

        
        (HumnResult::Humn, HumnResult::Literal(_)) | (HumnResult::Literal(_), HumnResult::Humn) => true,
        _ => false,
    }
}

fn solve_tree(root: &Rc<HumnAction>, val: i64) -> i64 {
    match (&root.lhs, &root.rhs) {
        (HumnResult::Dependent(new_root), HumnResult::Literal(rhs)) => {
            let true_val = solve_tree(new_root, (reverse_op(root.op))(val, *rhs));
            true_val
        },
        (HumnResult::Literal(lhs), HumnResult::Dependent(new_root)) => {
            let true_val = if is_associative(root.op) {
                solve_tree(new_root, (reverse_op(root.op))(val, *lhs))
            }
            else {
                solve_tree(new_root, (root.op)(*lhs, val))
            };

            true_val
        },
        (HumnResult::Humn, HumnResult::Literal(rhs)) => {
            let true_val = (reverse_op(root.op))(val, *rhs);
            true_val
        },
        (HumnResult::Literal(lhs), HumnResult::Humn) => {
            let true_val = if is_associative(root.op) {
                (reverse_op(root.op))(val, *lhs)
            }
            else {
                reverse_op(root.op)(*lhs, val)
            };
            true_val
        },
        _ => panic!(),
    }
}

pub fn part1(input: &str) {
    let (monkeys, mut resolved) = create_initial_setup(input);
    println!("{}", resolve_monkey("root", &monkeys, &mut resolved));

}

pub fn part2(input: &str) {
    let (monkeys, mut resolved) = create_initial_setup_humn(input);
    let root = create_humn_chain("root", &monkeys, &mut resolved);
    //println!("{:#?}", root);
    assert!(is_tree_solvable(&root));

    let humn = if let HumnResult::Literal(val) = root.rhs 
    && let HumnResult::Dependent(root) = &root.lhs {
        let humn = solve_tree(root, val);
        assert_eq!(val, root.resolve_with_humn_val(humn));
        humn
    }
    else if let HumnResult::Literal(val) = root.lhs
    && let HumnResult::Dependent(root) = &root.rhs {
        let humn = solve_tree(root, val);
        assert_eq!(val, root.resolve_with_humn_val(humn));
        humn
    }
    else {
        panic!();
    };

    println!("{}", humn);


}

pub fn day21(input: &str) {
    //part1(input);
    part2(input);
}