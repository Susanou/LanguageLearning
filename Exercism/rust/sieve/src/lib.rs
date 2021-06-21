pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let list = eratostenese_sieve(upper_bound as usize);

    let mut ret = Vec::new();

    for (i, b) in list.iter().enumerate(){
        if *b {
            ret.push(i as u64);
        }
    }

    ret
}

fn eratostenese_sieve(n: usize) -> Vec<bool> {
    let mut primes = vec![true; n+1];
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