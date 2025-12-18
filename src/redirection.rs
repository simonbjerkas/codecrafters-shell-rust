#[derive(Debug, PartialEq)]
pub enum Redirect {
    StdOut(bool),
    StdErr(bool),
}

#[derive(Debug, PartialEq)]
pub struct Redirection<'a> {
    pub redirect: Redirect,
    pub path: &'a str,
}

impl<'a> Redirection<'a> {
    pub fn new(redirection: Redirect, path: &'a str) -> Redirection<'a> {
        Redirection {
            redirect: redirection,
            path,
        }
    }
}

pub fn eval_redirect(redirect: &str) -> Redirect {
    let mut redirect = redirect.chars();
    let first = redirect
        .next()
        .expect("Redirects should always have minimum one char");
    let count = redirect.count();

    let append = (first == '>' && count == 1) || count == 2;

    if first == '2' {
        return Redirect::StdErr(append);
    }
    Redirect::StdOut(append)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redirection() {
        assert_eq!(eval_redirect("1>"), Redirect::StdOut(false));
        assert_eq!(eval_redirect("1>>"), Redirect::StdOut(true));
        assert_eq!(eval_redirect("2>"), Redirect::StdErr(false));
        assert_eq!(eval_redirect(">"), Redirect::StdOut(false));
        assert_eq!(eval_redirect(">>"), Redirect::StdOut(true));
        assert_eq!(eval_redirect("2>>"), Redirect::StdErr(true));
    }
}
