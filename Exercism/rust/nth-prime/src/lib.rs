pub fn nth(n: u32) -> u32 {

    let prime: Vec<bool> = eratostenese_sieve(n as usize);
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

    return res;
}

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