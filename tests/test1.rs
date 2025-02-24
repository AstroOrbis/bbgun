#[derive(Default, bbgun::Builder, Debug)]
struct User {
    active: bool,
    #[bbgun(transform = "email_parser")]
    email: String,
}

pub fn email_parser(input: String) -> String {
    input.to_lowercase()
}

fn main() {
    dbg!(User::builder()
        .active(true)
        .email("Bar@BBz.bAZ".to_string()));
}
