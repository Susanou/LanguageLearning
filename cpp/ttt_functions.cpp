#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

char board [9] = {'_', '_', '_','_', '_', '_','_', '_', '_'};

bool player = true; // False = X; True = O

void draw() {
    std::cout << "     |     |      \n";
    std::cout << "  " << board[0] << "  |  " << board[1] << "  |  " << board[2] << "\n";
    std::cout << "_____|_____|_____ \n";
    std::cout << "     |     |      \n";
    std::cout << "  " << board[3] << "  |  " << board[4] << "  |  " << board[5] << "\n";
    std::cout << "_____|_____|_____ \n";
    std::cout << "     |     |      \n";
    std::cout << "  " << board[6] << "  |  " << board[7] << "  |  " << board[8] << "\n";
    std::cout << "     |     |      \n";
    std::cout << "\n";
}

void greet(){
    cout << "------------\n";
    cout << "TIC-TAC-TOE\n";
    cout << "------------\n";
    cout << "\n";
    cout << "Rule n°1: there is no rule >:)\n";
    cout << "Rule n°2: basic tic tac toe rules, player 1 is O, player 2, X\n";
    cout << "Rule n°3: the squares of the gametable are numbered from 1 to 9.\n";
    cout << "\n";

    draw();
    cout << "\n";
    cout << "\n";
}

void set_position (int pos){
    if (player){
        board[pos] = 'O';
        player = false;
    }else{
        board[pos] = 'X';
        player = true;
    }
}

bool is_winner(){
    for(int i=0; i<9; i+=3){
        if(board[i]==board[i+1] && board[i+1]==board[i+2] && board[i]!='_') return true;
        if(board[i]==board[i+3] && board[i+3]==board[i+6] && board[i]!='_')  return true;
    }
    if(board[0]==board[4] && board[4]==board[8] && board[0]!='_') return true;
    if(board[2]==board[4] && board[4]==board[6] && board[2]!='_') return true;
    std::cout << "no winner detected\n";
    return false;
}

bool is_filled(){
    for (int i = 0; i < sizeof(board); i++){
        if (board[i] == '_'){
            return false;
        }
    }

    return true;
}

void take_turn(){

    cout << (board[0] != '_' && 1) << "\n";

    while (!is_winner() && !is_filled()){
        cout << "Player " << player << "'s Turn (Enter 1-9): ";

        int position;

        while (!(std::cin >> position)) {

            cout << "Player " << player << ", please enter a valid number between 1 and 9: ";
            cin.clear();
            cin.ignore();

        }

        cout << "\n";

        while (board[position-1] != '_') {

            cout << "Oops, there's already something in that position!\n\n";
            cout << "Player " << player << "'s Turn (Enter 1-9): ";
            cin >> position;
            cout << "\n";
        }

        set_position(position-1);

        draw();
    }
}

void end_game(){
    if (is_winner()){
        cout << "Ther's a winner!\n";
    }else if (is_filled()){
        cout << "It's a tie ladies and gentelman!\n";
    }
}