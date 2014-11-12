use std::iter::count;
use std::fmt;
use std::cmp::min;

enum Term {
    Empty,
    Number(int),
    Operator(char),
}

impl fmt::Show for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Number(num) => write!(f, "{}", num),
            Operator(op) => write!(f, "{}", op),
            Empty => write!(f, "(empty)"),
        }
    }
}

const MAX_EXPR_LEN: uint = 50;
const MAX_RES: uint = 1000;

const TERMS: [Term, ..8] = [
    Number(2),
    Number(22),
    Number(222),
    Number(2222),
    Operator('+'),
    Operator('-'),
    Operator('*'),
    Operator('/'),
];

#[allow(dead_code)]
fn print_expr(expr: &[Term], n: uint) {
    for i in range(0, n) {
        print!("{} ", expr[i]);
    }
    println!("");
}

fn calc(expr: &[Term], n: uint) -> Option<int> {
    let mut stack = [0, ..MAX_EXPR_LEN]; /* FIXME */
    let mut stack_len = 0;

    for i in range(0, n) {
        let term = expr[i];

        match term {
            Number(num) => {
                stack[stack_len] = num;
                stack_len += 1;
            },
            Operator(op) => {
                if stack_len < 2 { return None; }
                let b = stack[stack_len - 1];
                let a = stack[stack_len - 2];
                stack_len -= 2;

                let res = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => {
                        if b == 0 || a % b != 0 { return None; }
                        a / b
                    },
                    _ => panic!("Invalid term"), /* FIXME */
                };

                stack[stack_len] = res;
                stack_len += 1;
            },
            Empty => panic!("Invalid term"),
        }
    }

    match stack_len {
        1 => Some(stack[stack_len - 1]),
        _ => None,
    }
}

#[inline(always)]
fn get_op_prece(op: char) -> int {
    match op {
        '*' | '/' => 2,
        '+' | '-' => 1,
        _ => 99,
    }
}

#[inline(always)]
fn get_seq_prece(seq: &Vec<String>) -> int {
    let mut prece = 99;
    let seq_len = seq.len();
    for i in count(1, 2).take_while(|i| *i < seq_len) {
        prece = min(prece, get_op_prece(seq[i].chars().nth(0).unwrap()));
    }
    return prece;
}


fn post2in(expr: &[Term], n: uint) -> Option<String> {
    let mut stack = Vec::new();
    let mut stack_len: uint = 0;

    for i in range(0, n) {
        let term = expr[i];

        match term {
            Number(num) => {
                stack.push(vec![format!("{}", num)]);
                stack_len += 1;
            },
            Operator(op) => {
                if stack_len < 2 { return None; }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack_len -= 2;

                let a_len = a.len();
                let b_len = b.len();

                let new_a = if a_len >= 2
                    && get_seq_prece(&a)
                    < get_op_prece(op) {

                    vec![format!("({})", a.concat())]
                } else {
                    a
                };

                let new_b = if b_len >= 2
                    && get_seq_prece(&b)
                    <= get_op_prece(op) {

                    vec![format!("({})", b.concat())]
                } else {
                    b
                };

                stack.push(new_a + vec![format!("{}", op)] + new_b);
                stack_len += 1;
            },
            Empty => panic!("Invalid term"),
        }
    }

    match stack_len {
        1 => Some(stack.pop().unwrap().concat()),
        _ => None,
    }
}

fn sel(i: uint, n: uint, expr: &mut [Term], num_cnt: uint, oper_cnt: uint, sols: &mut Vec<String>) {
    if i == n {
        if num_cnt != oper_cnt + 1 { return; }

        let res = calc(expr, n);
        match res {
            Some(num) => {
                if num < 0 || num > (MAX_RES as int) { return; }

                let infix = post2in(expr, n).unwrap();
                let sol = &mut sols[num as uint];

                if sol.len() == 0 || sol.len() > infix.len() {
                    *sol = infix;
                }
            },
            None => (),
        }

        return;
    }

    for term in TERMS.iter() {
        expr[i] = *term;

        match *term {
            Number(..) => sel(i + 1, n, expr, num_cnt + 1, oper_cnt, sols),
            Operator(..) => {
                if num_cnt > oper_cnt + 1 {
                    sel(i + 1, n, expr, num_cnt, oper_cnt + 1, sols)
                }
            },
            Empty => panic!("Invalid term"),
        }
    }
}

pub fn get_seq(size: uint) -> Vec<String> {
    let mut sols = Vec::from_elem(MAX_RES + 1, String::new());
    let mut expr = [Empty, ..MAX_EXPR_LEN]; /* FIXME */

    for i in count(1, 2).take(size) {
        sel(0, i, expr, 0, 0, &mut sols);
    }

    return sols;
}
