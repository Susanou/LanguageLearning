#include "raindrops.h"

#include <string>

using namespace std;

namespace raindrops {

	string convert(int i) {
		string ret = "";

		if (i % 3 == 0) ret += "Pling";
		if (i % 5 == 0) ret += "Plang";
		if (i % 7 == 0) ret += "Plong";

		if (ret == "") {
			return to_string(i);
		}
		else {
			return ret;
		}
	}

}  // namespace raindrops
