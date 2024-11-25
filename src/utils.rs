use rand::{distributions::Alphanumeric, Rng};

pub fn generate_random_string() -> Option<String> {
    let random_id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    Some(random_id)
}
