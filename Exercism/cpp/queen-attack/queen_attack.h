#if !defined(QUEEN_ATTACK_H)
#define QUEEN_ATTACK_H

#include <iostream>
#include <string>

namespace queen_attack {

	class chess_board {
	private:
		std::pair<int, int> white_pos;
		std::pair<int, int> black_pos;
	public:
		chess_board(std::pair<int, int> w = std::make_pair(0, 3), std::pair<int, int> b = std::make_pair(7, 3));
		const std::pair<int, int>& white() const;
		const std::pair<int, int>& black() const;
		operator std::string() const;
		bool can_attack() const;
	};

}  // namespace queen_attack

#endif // QUEEN_ATTACK_H