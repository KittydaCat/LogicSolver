use std::rc::Rc;

#[derive(Clone, Debug)]
enum Sentence {
    Atomic(usize),
    Conditional(Rc<Sentence>, Rc<Sentence>),
    Not(Rc<Sentence>),
    And(Rc<Sentence>, Rc<Sentence>),
    Or(Rc<Sentence>, Rc<Sentence>),
}

impl PartialEq<Self> for Sentence {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Sentence::Atomic(x) => {
                if let Sentence::Atomic(y) = other {
                    x == y
                } else {
                    false
                }
            }
            Sentence::Conditional(prem, con) => {
                if let Sentence::Conditional(prem2, con2) = other {
                    prem == prem2 && con == con2
                } else {
                    false
                }
            }

            Sentence::Not(x) => {
                if let Sentence::Not(y) = other {
                    x == y
                } else {
                    false
                }
            }
            Sentence::And(prem, con) => {
                if let Sentence::And(prem2, con2) = other {
                    prem == prem2 && con == con2
                } else {
                    false
                }
            }
            Sentence::Or(prem, con) => {
                if let Sentence::Or(prem2, con2) = other {
                    prem == prem2 && con == con2
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Rule {
    ModusPonens(usize, usize),
    ModusTollens(usize, usize),
    Adjunction(usize, usize),
    Simplification(usize),
    Addition(usize),
    ModusTollendoPonens(usize, usize),
    Bijection(usize, usize),
    Equivication(usize, usize),

    // these scopes should have a premise
    ConditionalDerivation(usize, usize, Scope),
    IndirectDerivation(usize, usize, Scope),
}

// steps are read only until the sub-scope closes
#[derive(Clone, Debug)]
struct Scope {
    premise: Option<Sentence>,
    open_sub_scope: Option<Box<Scope>>,
    steps: Vec<(Sentence, Rule, Option<usize>)>
}

impl Scope {

    fn find(&self, condition: &dyn Fn(&Sentence) -> bool)
        -> Option<(Sentence, usize)> {

        match self.steps.iter().find(condition) {

            // todo add numbers
            Some(x) => {return Some((x.0.clone(), 0));}

            None => {
                if let Some(scope) = &self.open_sub_scope {

                    return scope.find(condition)

                } else {

                    return None
                }
            }
        };

    }

}
/*
Sentence
    It exists
    MP
    Simp
    Eq
    Mtp
Connection
    Cond - CD
    Negation - MT, Negative Eq
    And - Adj,
    Or - Add,
    Bijunction - Bijection,
Indirect Derivation
*/

// solve should move the solution along until the goal is reached
fn solve(goal: Sentence, scope: &mut Scope) -> Option<Scope> {

    if scope.find(&|x| *x == goal)

    if let Some((cond, _)) = scope.find( & |x|
        if let Sentence::Conditional(prem, con) = x {**con == goal} else {false}
    ) {



    };

    todo!()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {

    }
}