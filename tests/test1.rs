#[derive(Default, bbgun::Builder, Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    dbg!(User::builder()
        .username("Foo".to_string())
        .email("Bar@biz.baz".to_string())
        .sign_in_count(25043)
        .active(true));
}
