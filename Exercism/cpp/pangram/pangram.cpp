#include "pangram.h"

#include <string>
#include <algorithm>

using namespace std;

namespace pangram {

	bool is_pangram(string text) {

		for_each(text.begin(), text.end(), [](char& c) {c = ::tolower(c); });

		for (char c = 'a'; c <= 'z'; c++) {
			if (text.find(c) == string::npos) return false;
		}

		return true;

	}

}  // namespace pangram
