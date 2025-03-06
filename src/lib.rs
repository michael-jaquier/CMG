use std::fmt::Display;

use rand::Rng;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Clone, Debug, Error)]
pub enum CgmError {
    #[error("invalid integer token {0}")]
    ParseIntegerError(String),
    #[error("invalid float token {0}")]
    ParseFloatError(String),
    #[error("unknown token {0}")]
    UnknownToken(String),
}

pub type CgmResult<T> = Result<T, CgmError>;

#[derive(Serialize, Clone, Default, Copy)]
pub enum ProblemLevel {
    #[default]
    One = 1,
    Two = 2,
    Three = 3,
}

#[derive(Serialize, Clone, Default, Copy)]
pub enum ProblemType {
    #[default]
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl From<ProblemType> for Symb {
    fn from(value: ProblemType) -> Self {
        match value {
            ProblemType::Addition => Symb::Plus,
            ProblemType::Subtraction => Symb::Minus,
            ProblemType::Multiplication => Symb::Multiplication,
            ProblemType::Division => Symb::Division,
        }
    }
}

fn generate_numbers(pl: ProblemLevel) -> Vec<u32> {
    let mut rng = rand::rng();
    let max_value = 10_u32.pow(pl as u32);
    let ns = (pl as usize).max(2);
    (0..ns).map(|_| rng.random_range(1..=max_value)).collect()
}

fn problems(pl: ProblemLevel, pt: ProblemType) -> Question {
    let n = generate_numbers(pl);
    let v = n.into_iter().map(|zn| (zn, pt)).collect();
    let tokens = TokenRepr::from_problem(v);
    Question {
        stringified: TokenRepr::to_expression(&tokens),
        repr: tokens,
    }
}

#[derive(Serialize, Clone, Default)]
pub struct Request {
    plevel: ProblemLevel,
    ptype: ProblemType,
}

#[allow(dead_code)]
pub fn request(r: Option<Request>) -> Var {
    let (pl, pt) = match r {
        Some(r) => (r.plevel, r.ptype),
        None => (ProblemLevel::default(), ProblemType::default()),
    };
    let question = problems(pl, pt);
    Var {
        answer: TokenRepr::to_answer(&question),
        question,
        choices: Vec::new(),
    }
}

#[derive(Serialize, Clone, Default, Debug, Copy, PartialEq)]
enum Symb {
    Plus,
    Minus,
    Multiplication,
    Division,
    #[default]
    End,
}

impl Display for Symb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Symb::Plus => "+",
            Symb::Minus => "-",
            Symb::Multiplication => "*",
            Symb::Division => "/",
            Symb::End => "\n",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
enum TokenRepr {
    Symbol(Symb),
    Number(u32),
    #[default]
    End,
}

impl TokenRepr {
    fn to_answer(question: &Question) -> Answer {
        let mut n: f32 = 0.0;
        let mut action = Symb::End;
        let tokens = question.tokens();
        for token in tokens {
            match token {
                TokenRepr::Symbol(symb) => action = *symb,
                TokenRepr::Number(zn) => match action {
                    Symb::Plus => n += *zn as f32,
                    Symb::Minus => n -= *zn as f32,
                    Symb::Multiplication => n *= *zn as f32,
                    Symb::Division => n /= *zn as f32,
                    Symb::End => n += *zn as f32,
                },
                TokenRepr::End => {}
            }
        }
        Answer {
            stringified: n.to_string(),
            numerical: n,
        }
    }

    fn from_problem(n: Vec<(u32, ProblemType)>) -> Vec<TokenRepr> {
        let mut tokens = Vec::new();
        for (number, pt) in n.into_iter() {
            tokens.push(TokenRepr::Number(number));
            tokens.push(TokenRepr::Symbol(pt.into()))
        }
        tokens.pop();

        tokens
    }

    fn to_expression(tokens: &Vec<TokenRepr>) -> String {
        let mut s = "".to_string();
        for token in tokens {
            match token {
                TokenRepr::Symbol(symb) => s.push_str(&symb.to_string()),
                TokenRepr::Number(i) => s.push_str(&i.to_string()),
                TokenRepr::End => {
                    println!("end");
                    break;
                }
            }
        }
        s
    }
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct Question {
    stringified: String,
    repr: Vec<TokenRepr>,
}
impl Question {
    fn tokens(&self) -> &Vec<TokenRepr> {
        &self.repr
    }
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct Answer {
    stringified: String,
    numerical: f32,
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct Var {
    question: Question,
    answer: Answer,
    choices: Vec<Answer>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn json_output() {
        let var = Var::default();
        let output = serde_json::to_string(&var).ok();
        if let Some(ref out) = output {
            println!("{:?}", out);
        }
    }

    #[test]
    fn generate_problems() {
        for _ in 0..=100 {
            let n = generate_numbers(ProblemLevel::One);
            assert!(!n.iter().any(|n| *n > 10), "{:?}", n);
        }
    }

    #[test]
    fn make_request() {
        for _ in 0..100 {
            let req = Request {
                plevel: ProblemLevel::One,
                ptype: ProblemType::Addition,
            };
            let r = request(Some(req));
            assert!(r.answer.numerical <= 20.0, "{:?}", r);
        }
    }

    #[test]
    fn problem_three() {
        let pl = ProblemLevel::Three;
        let r = problems(pl, ProblemType::Addition);
        assert_eq!(r.tokens().len(), 5, "{:?}", r);
    }
}
