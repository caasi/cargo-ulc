use std::fmt;

// recursive enum should have its lifetime
enum Term<'a> {
    App(&'a Term<'a>, &'a Term<'a>),
    Lam(String, &'a Term<'a>),
    Var(String),
}

impl<'a> fmt::Display for Term<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Term::App(ref a, ref b) => write!(f, "({})({})", a, b),
            &Term::Lam(ref s, ref a) => write!(f, "\\{} {}", s, a),
            &Term::Var(ref s) => write!(f, "{}", s),
        }
    }
}

fn main() {
    let var1 = &Term::Var("a".to_string());
    let lam1 = &Term::Lam("b".to_string(), var1);
    let lam2 = &Term::Lam("a".to_string(), lam1);
    let var2 = &Term::Var("x".to_string());
    let app = &Term::App(lam2, var2);
    println!("{}", app);
}
