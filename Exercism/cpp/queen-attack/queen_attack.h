#if !defined(QUEEN_ATTACK_H)
#define QUEEN_ATTACK_H

#include <iostream>
#include <string>
#include <stdexcept>

namespace queen_attack {

	class chess_board {
	private:
		std::pair<int, int> white_pos;
		std::pair<int, int> black_pos;
	public:
		using position = std::pair<int, int>;

		explicit chess_board(const position& w = std::make_pair(0, 3), const position& b = std::make_pair(7, 3)) : white_pos(w), black_pos(b) {

			if(w == b)
				throw std::domain_error("Positions must be different");

		};
		const position& white() const;
		const position& black() const;
		explicit operator std::string() const;
		bool can_attack() const;
	};

}  // namespace queen_attack

#endif // QUEEN_ATTACK_H