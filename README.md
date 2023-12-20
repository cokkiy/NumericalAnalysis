# Numerical Analysis Function and Method

This repo contains numerical analysis function implements by Rust and C#.

## Polynomial

1. Polynomial Value Evaluates
   Evaluates a polynomial function with given coefficients and input values. The polynomial is evaluated using the following formula:
   `y = c[0] + (x - b[0]) * (c[1] + (x - b[1]) * (c[2] + ...))`
   > where c is the coefficients, b is the bias and x in the input values. x can be an array contains as many as values.

   PloyEval implements both in C# and Rust in platform `X86_64`and `aarch64`. If platform supports SIMD (in `aarch64`is `neon`and `x86_64`is `sse2`or `avx2`or `avx512`) then using SIMD when numbers of x is larger than 8.

## Equation Solving Function

1. Bisect
   Using the bisection method to find the root of a function within a given interval.
2. Newton method
   Using the Newton's method for finding the root of a function or improved Newton's method for finding m order multiple root.
3. fpi
   Using fixed-point iteration to find the root of a function.

## Vector

## Matrix
