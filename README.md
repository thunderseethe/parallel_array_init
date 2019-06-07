# par-array-init
The `par-array-init` crate allows you to initialize arrays with an initializer closure that will be called in parallel to fill the array.

This crate mirrors the api of crate (array-init)[https://crates.io/crates/array-init] with the caveat that initialization is performed in parallel.
An important departure from `array-init`, initialization order is not deterministic and should not be relied on.

Parallelization is achieved using (rayon)[https://https://crates.io/crates/rayon] and it's `ParallelIterator` api.

 # Examples:
 ```rust
 # extern crate par_array_init;
 # extern crate rayon;

 // Initialize an array of length 10 containing successive squares

 let arr: [usize; 50] = par_array_init::par_array_init(|i| i * i);

 // Initialize an array from an iterator producing an array of 34 repeated

 let mut iter = rayon::iter::repeat(34u32);
 let arr: [u32; 50] = par_array_init::from_iter(iter);
 ```
