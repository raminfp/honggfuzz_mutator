## Honggfuzz Mutator

The Honggfuzz mutator is responsible for generating input data to feed into the target program. It uses various techniques to create mutated input files based on the initial seed files. These mutations can include flipping bits, modifying data, and introducing random changes to the input.


#### Algorithms :

- mangle inc byte
- mangle neg byte
- mangle dec byte
- mangle ins byte
- mangle ovw byte
- mangle del byte
- mangle repeat insert
- mangle repeat ovw
- Todo (Soon)


#### Sample Output : 
```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/honggfuzz_mutator`
Original:  [81, 62, 231, 156] ,  Mutated :  [81, 62, 156]
Original:  [99, 104, 73, 214] ,  Mutated :  [99, 104, 73, 214]
Original:  [66, 155, 64, 65] ,  Mutated :  [66, 155, 64, 65, 65, 65, 65]
Original:  [93, 90, 211, 24] ,  Mutated :  [93, 90, 44, 24]
Original:  [15, 164, 213, 114] ,  Mutated :  [49, 15, 164, 213, 114]
Original:  [158, 199, 252, 77] ,  Mutated :  [158, 199, 252, 76]
Original:  [200, 41, 54, 127] ,  Mutated :  [200, 162, 54, 127]
Original:  [123, 160, 33, 206] ,  Mutated :  [123, 123, 33, 206]
Original:  [186, 179, 120, 25] ,  Mutated :  [186, 179, 120, 25, 25, 25, 25, 25, 25, 25, 25]
Original:  [181, 210, 235, 80] ,  Mutated :  [181, 210, 20, 80]
Original:  [167, 92, 45, 234] ,  Mutated :  [9, 167, 92, 45, 234]
Original:  [150, 95, 180, 165] ,  Mutated :  [150, 95, 180, 164]
Original:  [0, 1, 46, 23] ,  Mutated :  [0, 26, 46, 23]
Original:  [147, 216, 249, 198] ,  Mutated :  [147, 216, 249, 198]
Original:  [50, 203, 176, 241] ,  Mutated :  [50, 203, 176, 241, 241, 241, 241, 241, 241, 241, 241]
Original:  [13, 74, 3, 136] ,  Mutated :  [13, 74, 252, 136]
Original:  [63, 20, 133, 98] ,  Mutated :  [225, 63, 20, 133, 98]
Original:  [142, 247, 108, 253] ,  Mutated :  [142, 247, 108, 252]
Original:  [56, 217, 38, 175] ,  Mutated :  [56, 146, 38, 175]
Original:  [171, 16, 209, 190] ,  Mutated :  [171, 16, 209, 190]
```
