#[cfg(test)]
mod tests {
    use std::{env, sync::Once};

    static INIT: Once = Once::new();
    fn call_once() {
        INIT.call_once(|| {
            env::set_var("RUST_LOG", "debug"); // off / error / warn / info / debug / trace
                                               // env::set_var("RUST_BACKTRACE", "1");
            env::set_var("RUST_BACKTRACE", "full");
        })
    }

    #[tokio::test]
    async fn signup_test() {
        call_once();
    }
}
