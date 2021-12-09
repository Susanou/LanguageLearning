#if !defined(GRADE_SCHOOL_H)
#define GRADE_SCHOOL_H

#include <map>
#include <string>
#include <vector>

using namespace std;

namespace grade_school {

	class school {
	private:
		map<int, vector<string>> students;
	public:
		school() : students() {}
		void add(string name, int i);
		map<int, vector<string>> roster() const;
		vector<string> grade(int i) const;
	};

}  // namespace grade_school

#endif // GRADE_SCHOOL_H