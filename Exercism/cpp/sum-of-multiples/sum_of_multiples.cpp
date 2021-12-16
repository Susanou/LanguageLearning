#include "sum_of_multiples.h"

namespace sum_of_multiples {

	int to(std::vector<int> factors, int n) {

		int sum = 0;

		for (int i = 1; i < n; ++i) {
			for (auto f : factors) {
				if (!(i % f)) {
					sum += i;
					break;
				}
			}
		}

		return sum;
	}

}  // namespace sum_of_multiples
