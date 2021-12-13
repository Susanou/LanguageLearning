#include "grade_school.h"

#include <map>
#include <string>
#include <vector>
#include <algorithm>

namespace grade_school {

	void school::add(const std::string& name, int level) {
		std::vector<std::string> &ref = students[level];

		ref.insert(std::lower_bound(ref.begin(), ref.end(), name), name);

	}

	const std::map<int, std::vector<std::string>>& school::roster() const {
		return students;
	}

	std::vector<std::string> school::grade(int level) const{
		if (students.find(level) != students.end()) return students.at(level);
		else return {};
	}
}  // namespace grade_school
