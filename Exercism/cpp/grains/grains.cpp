#include "grains.h"

#include <math.h>
#include <stdint.h>

namespace grains {

	unsigned long long square(int n) {
		return 1ULL << (n - 1);
	}

	unsigned long long total() {
		unsigned long long total = 0ULL;

		for (int i = 1; i <= 64; ++i) {
			total += square(i);
		}

		return total;
	}

}  // namespace grains
