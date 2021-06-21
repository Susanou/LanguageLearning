use std::ops::RangeFrom;

pub fn nth(n: u32) -> u32{

/*     let prime: Vec<bool> = eratostenese_sieve((n+1) as usize);
    let mut count = 0;
    let mut res: u32 = 0;

    for i in 0..prime.len(){
        if prime[i]{
            count += 1;
        }

        if count == n{
            res = i as u32;
            break;
        }
    }

    return res; */

    (2..).filter(|x| prime(*x)).nth(n as usize).unwrap()
}

fn prime(n: u32) -> bool {
    !(2..n).any(|i| n % i as u32 == 0)
}

//Wanted to use this because it is one of the best ways to generate primes and for training as well
//but coudn't make it work at all
fn eratostenese_sieve(n: usize) -> Vec<bool> {
    let mut primes = vec![true; n];
    let mut p: usize = 2;

    while p.pow(2) <= n {
        
        if primes[p] {

            for i in (p*2..n+1).step_by(p){
                primes[i] = false;
            }
        }

        p += 1
    }

    primes[0] = false;
    primes[1] = false;

    return primes;

}