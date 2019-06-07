#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

//! The `par-array-init` crate allows you to initialize arrays with an initializer closure that will be called in parallel to fill the array.
//!
//! This crate mirrors the api of crate (array-init)[https://crates.io/crates/array-init] with the caveat that initialization is performed in parallel.
//! An important departure from `array-init`, initialization order is not deterministic and should not be relied on.
//!
//! Parallelization is achieved using (rayon)[https://https://crates.io/crates/rayon] and it's `ParallelIterator` api.
//!
//! # Examples:
//! ```rust
//! # extern crate par_array_init;
//!
//! // Initialize an array of length 10 containing successive squares
//!
//! let arr: [usize; 50] = par_array_init::par_array_init(|i| i * i);
//!
//! // Initialize an array from an iterator producing an array of 34 repeated
//!
//! let mut iter = rayon::iter::repeat(34u32);
//! let arr: [u32; 50] = par_array_init::from_iter(iter);
//! ```
//!
extern crate array_init;
extern crate rayon;

use array_init::IsArray;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

/// Initialize an array given a function from index to element
pub fn par_array_init<Array, F>(initializer: F) -> Array
where
    Array: Send + Sync + IsParArray,
    F: Send + Sync + Fn(usize) -> Array::Item,
    Array::Item: Send + Sync + Copy,
{
    let elems = (0..Array::len()).into_par_iter().map(|i| initializer(i));
    // Since our iterator is 0..len here we're safe to unwrap
    from_iter(elems).unwrap()
}

/// Initialize an array given a parallel iterator.
///
/// IndexedParallelIterator is required (as opposed to just ParallelIterator) so we know we have atleast as many elements as array length.
///
/// Takes elements from the iterator until the Array is full, returns full array on completion. Returns None if there are not enough elements.
pub fn from_iter<Array, I>(into_iter: I) -> Option<Array>
where
    I: IntoParallelIterator<Item = Array::Item>,
    I::Iter: IndexedParallelIterator<Item = Array::Item>,
    Array: IsParArray,
    Array::Item: Copy + Send + Sync,
{
    let iter = into_iter.into_par_iter();
    if Array::len() > iter.len() {
        return None;
    }
    let mut ret: Array = unsafe { std::mem::uninitialized() };
    ret.mut_slice()
        .into_par_iter()
        .zip(iter)
        .for_each(|(dst, src)| {
            *dst = src;
        });
    Some(ret)
}

/// Extension of `array_init::IsArray` that includes method to convert the array to a mutable slice.
/// `&mut [T]` implements parallel iterator so this allows us to write our elements in parallel without having to explicitly pass a `*mut T` across threads.
pub trait IsParArray: IsArray {
    /// Convert array to a mutable slice.
    fn mut_slice(&mut self) -> &mut [Self::Item];
}

macro_rules! impl_is_par_array {
    ($($size:expr)+) => ($(
        impl<T> IsParArray for [T; $size] {
            fn mut_slice(&mut self) -> &mut [Self::Item] {
                self.as_mut()
            }
        }
    )+);
}

// This is like straight up copied from (array-init's impl)[https://github.com/Manishearth/array-init/blob/master/src/lib.rs#L270]
// let the legacy live on
impl_is_par_array! {
     0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
    32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47
    48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63
    64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79
    80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95
    96 97 98 99 100 101 102 103 104 105 106 107 108
    109 110 111 112 113 114 115 116 117 118 119 120
    121 122 123 124 125 126 127 128 129 130 131 132
    133 134 135 136 137 138 139 140 141 142 143 144
    145 146 147 148 149 150 151 152 153 154 155 156
    157 158 159 160 161 162 163 164 165 166 167 168
    169 170 171 172 173 174 175 176 177 178 179 180
    181 182 183 184 185 186 187 188 189 190 191 192
    193 194 195 196 197 198 199 200 201 202 203 204
    205 206 207 208 209 210 211 212 213 214 215 216
    217 218 219 220 221 222 223 224 225 226 227 228
    229 230 231 232 233 234 235 236 237 238 239 240
    241 242 243 244 245 246 247 248 249 250 251 252
    253 254 255 256 257 258 259 260 261 262 263 264
    265 266 267 268 269 270 271 272 273 274 275 276
    277 278 279 280 281 282 283 284 285 286 287 288
    289 290 291 292 293 294 295 296 297 298 299 300
    301 302 303 304 305 306 307 308 309 310 311 312
    313 314 315 316 317 318 319 320 321 322 323 324
    325 326 327 328 329 330 331 332 333 334 335 336
    337 338 339 340 341 342 343 344 345 346 347 348
    349 350 351 352 353 354 355 356 357 358 359 360
    361 362 363 364 365 366 367 368 369 370 371 372
    373 374 375 376 377 378 379 380 381 382 383 384
    385 386 387 388 389 390 391 392 393 394 395 396
    397 398 399 400 401 402 403 404 405 406 407 408
    409 410 411 412 413 414 415 416 417 418 419 420
    421 422 423 424 425 426 427 428 429 430 431 432
    433 434 435 436 437 438 439 440 441 442 443 444
    445 446 447 448 449 450 451 452 453 454 455 456
    457 458 459 460 461 462 463 464 465 466 467 468
    469 470 471 472 473 474 475 476 477 478 479 480
    481 482 483 484 485 486 487 488 489 490 491 492
    493 494 495 496 497 498 499 500 501 502 503 504
    505 506 507 508 509 510 511 512
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_array_init_in_parallel_works() {
        let array: [usize; 10] = par_array_init(|i| i);
        for (indx, i) in array.iter().enumerate() {
            assert_eq!(*i, indx)
        }
    }

    #[test]
    fn test_array_init_with_capturing_lambda_works() {
        let vec = vec![40; 10];
        let array: [usize; 10] = par_array_init(|i| vec[i]);
        for i in array.iter() {
            assert_eq!(*i, 40);
        }
    }

    #[test]
    fn test_from_iter_works() {
        let vec = vec![1, 2, 3, 4];
        let array: Option<[usize; 4]> = from_iter(vec.into_par_iter());
        assert_eq!(array, Some([1, 2, 3, 4]));
    }

    #[test]
    fn test_from_iter_fails_when_len_is_different() {
        let vec = vec![1, 2, 3, 4];
        let array: Option<[usize; 10]> = from_iter(vec.into_par_iter());
        assert_eq!(array, None);
    }

    #[test]
    fn test_initialization_works_with_larger_types() {
        let vec = vec![
            "A neat string",
            "A lame string",
            "A longish string abcdefghijklmnopqrstuvwxyz",
        ];
        let array: Option<[&str; 3]> = from_iter(vec.into_par_iter());
        assert_eq!(
            array,
            Some([
                "A neat string",
                "A lame string",
                "A longish string abcdefghijklmnopqrstuvwxyz"
            ])
        )
    }
}
