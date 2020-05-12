use rand;
use rand::distributions::{Distribution, Uniform};

// 验证码生成
pub fn generate_verify() -> String {
    // let mut ckm_arr: Vec<u8> = vec![];
    // let step = Uniform::from(48..58);
    // for _ in 0..6 {
    //     ckm_arr.push(step.sample(&mut rand::thread_rng()));
    // }
    // let ckm = format!("{}", String::from_utf8_lossy(&ckm_arr));
    let mut ckm_arr: Vec<u8> = vec![];

    let step = Uniform::from(48..58);
    for _ in 0..6 {
        ckm_arr.push(step.sample(&mut rand::thread_rng()));
    }

    format!("{}", String::from_utf8_lossy(&ckm_arr))
}