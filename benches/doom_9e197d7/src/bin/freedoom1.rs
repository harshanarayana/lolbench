//{"name":"freedoom1","crate":"doom_9e197d7"}
extern crate doom_9e197d7 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; doom_9e197d7 :: freedoom1 ( & mut crit ) ; }