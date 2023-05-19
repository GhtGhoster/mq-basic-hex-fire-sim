
pub fn cube_round((q, r): (f32, f32)) -> (isize, isize) {
    let s = -q-r;

    let mut rounded_q = q.round();
    let mut rounded_r = r.round();
    let rounded_s = s.round();

    let q_diff = (rounded_q - q).abs();
    let r_diff = (rounded_r - r).abs();
    let s_diff = (rounded_s - s).abs();

    if q_diff > r_diff && q_diff > s_diff {
        rounded_q = -rounded_r-rounded_s;
    } else if r_diff > s_diff {
        rounded_r = -rounded_q-rounded_s;
    }
    // else {
    //     rounded_s = -rounded_q-rounded_r;
    // }

    (rounded_q as isize, rounded_r as isize)
}