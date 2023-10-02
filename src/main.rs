fn main() {
    let mut a = vec![1, 4, 3, 5, 2];
    println!("{:?}", a);
    m_sort(&mut a, 0, 5);
    println!("{:?}", a);
}

const BANPEI : i32 = 1e18 as i32;

fn m_sort(a: &mut[i32], left: usize, right: usize) {
    if left + 1 < right {
        let mid = (left + right) >> 1;
        // let _ = crossbeam::scope(|spawner| {
        //     spawner.spawn(move |_| {
        //         m_sort(a, left, mid);
        //     });
        //     spawner.spawn(move |_| {
        //         m_sort(a, mid, right);
        //     });
        // });
        m_sort(a, left, mid);
        m_sort(a, mid, right);
        merge(a, left, mid, right);
    }
}

fn merge(a: &mut[i32], left: usize, mid: usize, right: usize) {
    let n_l = mid - left;
    let n_r = right - mid;
    let mut l_vec = vec![0; n_l];
    let mut r_vec = vec![0; n_r];
    l_vec[..n_l].copy_from_slice(&a[left..(n_l + left)]);
    r_vec[..n_r].copy_from_slice(&a[mid..(n_r + mid)]);
    l_vec.push(BANPEI);
    r_vec.push(BANPEI);
    let mut i = 0_usize;
    let mut j = 0_usize;
    for k in left..right {
        if l_vec[i] <= r_vec[j] {
            a[k] = l_vec[i];
            i+=1;
        } else {
            a[k] = r_vec[j];
            j+=1;
        }
    }
}