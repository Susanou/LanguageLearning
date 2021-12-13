#include "collatz_conjecture.h"

#include <stdexcept>

namespace collatz_conjecture {

	int steps(int n) {
		
		if (n < 1) {
			throw std::domain_error("number should be positive non 0 int");
		}
		
		int s = 0;

		while (n != 1) {
			if (n % 2 == 0) {
				n /= 2;
			}
			else {
				n = 3 * n + 1;
			}

			s++;
		}

		return s;
	}

}  // namespace collatz_conjecture
