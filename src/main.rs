use rand::seq::SliceRandom;
use std::thread;

fn main() {
    let mut a = Vec::<i32>::new();
    for i in 0..100000000 {
        a.push(i);
    }
    let mut rng = rand::thread_rng();
    a.shuffle(&mut rng);
    m_sort_p(&mut a, 0);
    // a.sort();
    // m_sort(&mut a);
    // println!("{:?}", a);
}

const BANPEI: i32 = 1e18 as i32;
const MAX_THREAD: usize = 5;


/*
右側のみがスレッド生成される．
スレッドの数は(1 << depth)
*/
fn m_sort_p(a: &mut Vec<i32>, depth: usize) {
    let left = 0_usize;
    let right = a.len();
    if left + 1 < right {
        let mid = (left + right) >> 1;
        let mut a_r = a.split_off(mid);
        if depth < MAX_THREAD {
            let hundle = thread::spawn(move || {
                m_sort_p(&mut a_r, depth + 1);
                a_r
            });
            m_sort_p(a,depth + 1);
            let a_r = hundle.join().unwrap();
            a.extend_from_slice(&a_r);
        } else {
            m_sort_p(&mut a_r, depth);
            m_sort_p(a, depth);
            a.extend_from_slice(&a_r);
        }
        merge(a, left, mid, right);
    }
}

fn m_sort(a: &mut Vec<i32>) {
    let left = 0_usize;
    let right = a.len();
    if left + 1 < right {
        let mid = (left + right) >> 1;
        let mut a_r = a.split_off(mid);
        m_sort(a);
        m_sort(&mut a_r);
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
    for x in a.iter_mut().take(right).skip(left) {
        if l_vec[i] <= r_vec[j] {
            *x = l_vec[i];
            i += 1;
        } else {
            *x = r_vec[j];
            j += 1;
        }
    }
}
