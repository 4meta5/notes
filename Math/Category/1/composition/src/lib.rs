
// identity function
pub fn id<T>(object: T) -> T {
    object
}

// composition
pub fn compose<F: 'static, G: 'static, Fg, Gh, H>(f: F, g: G) -> Box<Fn(Fg) -> H>
where
    F: Fn(Fg) -> Gh,
    G: Fn(Gh) -> H,
{
    Box::new(move |x| g(f(x)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
