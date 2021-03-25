use iter_vec::ExactSizedIterVec;

fn main() {
    let content = vec![1, 2, 3, 4];
    let content2 = vec![5, 6, 7, 8];
    let mut iter_vec = ExactSizedIterVec::new();
    iter_vec.push(&content);
    iter_vec.push(&content2);
    println!("{:#?} , len {}", iter_vec, iter_vec.len());
    for x in iter_vec {
        println!("x={}", x);
    }
    //println!("{:#?}", iter_vec);
}
