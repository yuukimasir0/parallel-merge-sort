use rand::seq::SliceRandom;
use std::thread;

fn main() {
    let mut a = Vec::<i32>::new();
    for i in 0..10000 {
        a.push(i);
    }
    let mut rng = rand::thread_rng();
    a.shuffle(&mut rng);
    m_sort(&mut a);
    println!("{:?}", a);
}

const BANPEI: i32 = 1e18 as i32;

fn m_sort(a: &mut Vec<i32>) {
    let left = 0_usize;
    let right = a.len();
    if left + 1 < right {
        let mid = (left + right) >> 1;
        let mut a_r = a.split_off(mid);
        let hundle = thread::spawn(move || {
            m_sort(&mut a_r);
            a_r
        });
        m_sort(a);
        let a_r = hundle.join().unwrap();
        a.extend_from_slice(&a_r);
        merge(a, left, mid, right);
    }
}

fn merge(a: &mut [i32], left: usize, mid: usize, right: usize) {
    let n_l = mid - left;
    let n_r = right - mid;
    let mut l_vec = vec![0; n_l];
    let mut r_vec = vec![0; n_r];
    l_vec.copy_from_slice(&a[left..(n_l + left)]);
    r_vec.copy_from_slice(&a[mid..(n_r + mid)]);
    l_vec.push(BANPEI);
    r_vec.push(BANPEI);
    let mut i = 0_usize;
    let mut j = 0_usize;
    for x in a.iter_mut().skip(left) {
        if l_vec[i] <= r_vec[j] {
            *x = l_vec[i];
            i += 1;
        } else {
            *x = r_vec[j];
            j += 1;
        }
    }
}
