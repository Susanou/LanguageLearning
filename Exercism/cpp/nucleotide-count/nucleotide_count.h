#if !defined(NUCLEOTIDE_COUNT_H)
#define NUCLEOTIDE_COUNT_H

#include <map>
#include <string>

namespace nucleotide_count {

	class counter {
	private:
		std::map<char, int> nucleotides{ {'A', 0}, {'T', 0}, {'C', 0}, {'G', 0} };
	public:
		counter(const std::string&);

		int count(char c) const;
		std::map<char, int> nucleotide_counts() const;
	};

}  // namespace nucleotide_count

#endif // NUCLEOTIDE_COUNT_H