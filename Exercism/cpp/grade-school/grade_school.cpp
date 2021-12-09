#include "grade_school.h"

#include <map>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

namespace grade_school {

	void school::add(string name, int i) {
		if (students.find(i) != students.end()) {
			students[i].push_back(name);
			sort(students[i].begin(), students[i].end());
		}
		else {
			students[i] = vector<string> { name };
		}
	}

	map<int, vector<string>> school::roster() const{
		return students;
	}

	vector<string> school::grade(int i) const{
		if (students.find(i) != students.end()) return students.at(i);
		else return {};
	}
}  // namespace grade_school
