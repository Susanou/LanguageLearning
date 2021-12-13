#include "nucleotide_count.h"

#include <stdexcept>
#include <string>
#include <map>

namespace nucleotide_count {

	counter::counter(const std::string& dna) {
		for (auto c : dna) {
			if (nucleotides.find(c) != nucleotides.end()) {
				nucleotides[c]++;
			}
			else {
				throw std::invalid_argument("Letter outside of possibilities");
			}
		}
	}

	std::map<char, int> counter::nucleotide_counts() const {
		return nucleotides;
	}

	int counter::count(char c) const{
		if (nucleotides.count(c) > 0) return nucleotides.find(c)->second;
		else throw std::invalid_argument("Letter outside of possibilities");
	}

}  // namespace nucleotide_count
