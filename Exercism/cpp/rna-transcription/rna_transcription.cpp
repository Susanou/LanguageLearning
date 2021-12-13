#include "rna_transcription.h"

#include <iterator>

namespace rna_transcription {

	char to_rna(char c) {
		switch (c) {
			case 'A':
				return 'U';
				break;
			case 'G':
				return 'C';
				break;
			case 'C':
				return 'G';
				break;
			case 'T':
				return 'A';
				break;
			default:
				throw std::invalid_argument("Letter not in scope");
		}
	}

	std::string to_rna(const std::string& dna) {

		std::string rna{};

		std::transform(dna.begin(), dna.end(), std::back_inserter(rna),
			[](const auto& ch) {return to_rna(ch); });
							
		return rna;
	}

}  // namespace rna_transcription
