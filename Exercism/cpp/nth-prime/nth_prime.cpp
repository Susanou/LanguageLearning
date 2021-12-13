#include "nth_prime.h"

#include <list>
#include <math.h>
#include <stdexcept>

namespace nth_prime {

	int nth(int n) {

		if (n < 1) throw std::domain_error("n should be positive non null");
		if (n == 1) return 2;
		
		std::list<int> primes;
		primes.push_back(2);

		int num = 3; 

		while (primes.size() < (long unsigned int)n) {
			bool isPrime = true;

			for (int p : primes) {
				if (!(num % p)) {
					isPrime = false;
					break;
				}
			}

			if (isPrime) {
				primes.push_back(num);
			}
			num += 2;

		}

		return primes.back();
	}

}  // namespace nth_prime
