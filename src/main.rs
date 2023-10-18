use honggfuzz_mutator::mutator::mutate::Mutator;

fn main() {

    let mut mutator = Mutator::new()
        .printable(false)
        .max_sz(4)
        .offset(0)
        .seed(0xabcdef1234567890)
        .exp_disabled(false);

    // for _ in 0..20 {
    loop {
        mutator.mangle_random_insert();
        let original = {
            let dyn_file_data = &mutator.run.dyn_file.data.clone();
            mutator.dyn_file(dyn_file_data);
            dyn_file_data.clone() // Release the borrow here
        };
        mutator.mutate(1);
        println!("Original:  {:?} ,  Mutated :  {:?}", &original, &mutator.run.dyn_file.data);
        mutator.run.dyn_file.data.clear();
    }
}
