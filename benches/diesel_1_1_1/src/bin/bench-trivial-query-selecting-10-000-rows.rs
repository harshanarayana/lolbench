//{"name":"bench_trivial_query_selecting_10_000_rows","crate":"diesel_1_1_1"}
extern crate diesel_1_1_1 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; diesel_1_1_1 :: bench_trivial_query_selecting_10_000_rows ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; diesel_1_1_1 :: bench_trivial_query_selecting_10_000_rows ( & mut crit ) ; }