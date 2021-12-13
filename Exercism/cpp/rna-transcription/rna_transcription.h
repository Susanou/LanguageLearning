#if !defined(RNA_TRANSCRIPTION_H)
#define RNA_TRANSCRIPTION_H

#include <stdexcept>
#include <string>
#include <algorithm>

namespace rna_transcription {

	char to_rna(char c);
	std::string to_rna(const std::string& dna);

}  // namespace rna_transcription

#endif // RNA_TRANSCRIPTION_H