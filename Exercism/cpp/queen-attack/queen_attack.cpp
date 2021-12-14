#include "queen_attack.h"

#include <stdexcept>
#include <cstdlib>

namespace queen_attack {


	chess_board::chess_board(std::pair<int, int> w, std::pair<int, int> b) {
		if (w.first == b.first && w.second == b.second)
			throw std::domain_error("Positions must be different");

		else {
			white_pos = w;
			black_pos = b;
		}
	}

	const std::pair<int, int>& chess_board::white() const{
		return white_pos;
	}

	const std::pair<int, int>& chess_board::black() const{
		return black_pos;
	}

	bool chess_board::can_attack() const {
		return (white_pos.first == black_pos.first) ||
			(white_pos.second == black_pos.second) ||
			(std::abs(white_pos.first - black_pos.first) == std::abs(white_pos.second - black_pos.second));
	}

	chess_board::operator std::string() const {

		std::string board_str = "";

		for (int i = 0; i < 8; i++) {
			for (int j = 0; j < 8; j++) {
				if (i == white_pos.first && j == white_pos.second)
					board_str += 'W';
				else if (i == black_pos.first && j == black_pos.second)
					board_str += 'B';
				else
					board_str += '_';

				if (j == 7)
					board_str += '\n';
				else
					board_str += ' ';
			}
		}

		return board_str;
	}

}  // namespace queen_attack
