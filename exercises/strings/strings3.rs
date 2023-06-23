// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.


fn trim_me(input: &str) -> String {
    let mut x=input.to_string();
    x=x.trim_start();
    x=x.trim_end();
    x
}

fn compose_me(input: &str) -> String {
    let x=input.to_string();
    x+" world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons    ???
	let x=input.to_string();
    let y=x.replace("cars","balloons");
    y

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
