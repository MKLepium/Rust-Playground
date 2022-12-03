#![feature(bench_black_box)]

use rand::prelude::*;
use core::f64;
use std::{ops::Mul, time::Instant};


macro_rules! make_row_major_matrix {
    ($name:ident, $row_count:literal) => {
        #[derive(Copy, Clone)]
        struct $name {
            vals: [f64; $row_count*$row_count], // Use f64 values in a single, flat array. 
        }
        impl $name {
            pub const SIZE : usize = $row_count;
            pub fn zeroed() -> Self {
                Self {
                    vals: [0.0_f64; $row_count*$row_count],
                }
            }
            pub fn new(vals: [f64; $row_count*$row_count]) -> Self {
                Self { 
                    vals
                 }
            }
            pub fn random() -> Self {
                let mut vals = [0.0; Self::SIZE * Self::SIZE];
                let mut rng = rand::thread_rng();
                (0..Self::SIZE*Self::SIZE).for_each(|idx| {
                    vals[idx] = rng.gen();
                });
                Self { vals }
            }
        }
        impl Mul for &$name {
            type Output = $name;
            fn mul(self, rhs: Self) -> $name {
                let mut vals = [0.0; $name::SIZE * $name::SIZE];
                for col in 0..$name::SIZE {
                    for row in 0..$name::SIZE {
                        let dst_idx = idx_row_major(row, col, $name::SIZE);
                        for idx in 0..$name::SIZE {
                            let l_idx = idx_row_major(row, idx, $name::SIZE);
                            let r_idx = idx_row_major(idx, col, $name::SIZE);
        
                            vals[dst_idx] += self.vals[l_idx] * rhs.vals[r_idx];
                        }
                    }
                }
                $name::new(vals)
            }
        }
    };
}

macro_rules! make_col_major_matrix {
    ($name:ident, $row_count:literal) => {
        #[derive(Copy, Clone)]
        struct $name {
            vals: [f64; $row_count*$row_count], // Use f64 values in a single, flat array. 
        }
        impl $name {
            pub const SIZE : usize = $row_count;
            pub fn zeroed() -> Self {
                Self {
                    vals: [0.0_f64; $row_count*$row_count],
                }
            }
            pub fn new(vals: [f64; $row_count*$row_count]) -> Self {
                Self { 
                    vals
                 }
            }
            pub fn random() -> Self {
                let mut vals = [0.0; Self::SIZE * Self::SIZE];
                let mut rng = rand::thread_rng();
                (0..Self::SIZE*Self::SIZE).for_each(|idx| {
                    vals[idx] = rng.gen();
                });
                Self { vals }
            }
        }
        impl Mul for &$name {
            type Output = $name;
            fn mul(self, rhs: Self) -> $name {
                let mut vals = [0.0; $name::SIZE * $name::SIZE];
                for col in 0..$name::SIZE {
                    for row in 0..$name::SIZE {
                        let dst_idx = idx_col_major(row, col, $name::SIZE);
                        for idx in 0..$name::SIZE {
                            let l_idx = idx_col_major(row, idx, $name::SIZE);
                            let r_idx = idx_col_major(idx, col, $name::SIZE);
        
                            vals[dst_idx] += self.vals[l_idx] * rhs.vals[r_idx];
                        }
                    }
                }
                $name::new(vals)
            }
        }
    };
}

macro_rules! benchmark_setup {
    ($name:ident, $struct:ty) => {
        fn $name(msg:String ) {
            let count = 2 << 12;
            let matrices = (0..count)
                .map(|_| <$struct>::random())
                .collect::<Vec<_>>();
        
            let runtimes = (0..10)
                .map(|_| {
                    let mut dst_matrices = (0..(count - 1))
                        .map(|_| <$struct>::zeroed())
                        .collect::<Vec<_>>();
        
                    let t_start = Instant::now();
                    for idx in 0..(count - 1) {
                        let l = &matrices[idx];
                        let r = &matrices[idx + 1];
                        dst_matrices[idx] = l * r;
                    }
                    let time = t_start.elapsed().as_secs_f64();
                    core::hint::black_box(dst_matrices);
        
                    time
                })
                .collect::<Vec<_>>();
        
            let avg_runtime = runtimes.into_iter().sum::<f64>() / 10.0;
        
            println!("{}: {}s", msg ,  avg_runtime);
        }
    };
}


fn idx_row_major(row: usize, col: usize, dimension: usize) -> usize {
    (row * dimension) + col
}

fn idx_col_major(row: usize, col: usize, dimension: usize) -> usize {
    (col * dimension) + row
}



make_row_major_matrix!(Matrix8x8RowStruct, 8);
make_row_major_matrix!(Matrix16x16RowStruct, 16);
make_row_major_matrix!(Matrix32x32RowStruct, 32);

make_col_major_matrix!(Matrix8x8ColStruct, 8);
make_col_major_matrix!(Matrix16x16ColStruct, 16);
make_col_major_matrix!(Matrix32x32ColStruct, 32);

benchmark_setup!(profile_col_matrix_multiplication_8x8, Matrix8x8ColStruct);
benchmark_setup!(profile_col_matrix_multiplication_16x16, Matrix16x16ColStruct);
benchmark_setup!(profile_col_matrix_multiplication_32x32, Matrix32x32ColStruct);


benchmark_setup!(profile_row_matrix_multiplication_8x8, Matrix8x8RowStruct);
benchmark_setup!(profile_row_matrix_multiplication_16x16, Matrix16x16RowStruct);
benchmark_setup!(profile_row_matrix_multiplication_32x32, Matrix32x32RowStruct);

fn main() {
    profile_col_matrix_multiplication_8x8("Matrix 8x8 Col_Major".to_string());
    profile_col_matrix_multiplication_16x16("Matrix 16x16 Col_Major".to_string());
    profile_col_matrix_multiplication_32x32("Matrix 32x32 Col_Major".to_string());
    profile_row_matrix_multiplication_8x8("Matrix 8x8 Row_Major".to_string());
    profile_row_matrix_multiplication_16x16("Matrix 16x16 Row_Major".to_string());
    profile_row_matrix_multiplication_32x32("Matrix 32x32 Row_Major".to_string());


}




