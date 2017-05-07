mod outermost {
    pub fn middle_function() {
        middle_secret_function();
    }

    fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {
            secret_function();
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::inside::inner_function();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
