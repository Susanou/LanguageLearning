#include <algorithm>
#include <stdexcept>

#include "triangle.h"

using namespace std;

namespace triangle {

	flavor kind(double a, double b, double c) {
		if (a < b) swap(a, b);
		if (a < c) swap(a, c);
		if (b < c) swap(b, c); // make it so that we have a <= b <= c in that order

		if (a > b + c) throw domain_error("Triangle inequality not respected");
		if (c <= 0) throw domain_error("Number can't be equal or lower than 0");

		if (a == c) return flavor::equilateral;
		if (a == b || b == c) return flavor::isosceles;
		if (a == b + c) return flavor::degenerate;
		else return flavor::scalene;
	}

}  // namespace triangle
