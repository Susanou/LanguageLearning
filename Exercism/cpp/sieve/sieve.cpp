#include "sieve.h"

#include <vector>
#include <stdexcept>

namespace sieve {

	std::vector<int> primes(int n) {
		if (n < 1) throw std::domain_error("n should be positive non null");
		if (n == 1) return {};

		std::vector<int> prime;
		prime.push_back(2);

		for (int i = 3; i < n+1; i+=2) {
			bool isPrime = true;

			for (int p : prime) {
				if (!(i % p)) {
					isPrime = false;
				}
			}

			if (isPrime) {
				prime.push_back(i);
			}

		}

		return prime;
	}

}  // namespace sieve
