#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Mat3;
use std::ops::Mul;
use support::*;

bench_unop!(
    mat3_transpose,
    "mat3 transpose",
    op => transpose,
    from => random_mat3
);
bench_unop!(
    mat3_determinant,
    "mat3 determinant",
    op => determinant,
    from => random_mat3
);
bench_unop!(mat3_inverse, "mat3 inverse", op => inverse, from => random_mat3);
bench_binop!(mat3_mul_mat3, "mat3 * mat3", op => mul, from => random_mat3);
bench_from_ypr!(mat3_from_ypr, "mat3 from ypr", ty => Mat3);

criterion_group!(
    benches,
    mat3_transpose,
    mat3_determinant,
    mat3_inverse,
    mat3_mul_mat3,
    mat3_from_ypr,
);

criterion_main!(benches);
