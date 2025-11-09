use worker::*;

pub fn get_d1(env: &Env) -> Result<D1Database> {
    env.d1("DB")
}
