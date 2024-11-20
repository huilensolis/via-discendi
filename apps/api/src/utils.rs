use rand::{distributions::Alphanumeric, Rng};

pub fn generate_random_str(length: u8) -> String {
    let random_string: String = thread_rng()
        .sample_iter(&Alphanumeric) // generates random alphanumeric characters
        .take(length.into()) // take n characters
        .map(char::from) // map them to characters
        .collect(); // collect into a String

    random_string
}
