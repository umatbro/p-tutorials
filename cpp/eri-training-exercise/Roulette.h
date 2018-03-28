#pragma once
#include <vector>
#include <string>
#include "Player.h"

using std::vector;
using std::string;

struct Result
{
    int dozen;
    int parity; // parzystość
    int number;
    bool is_zero;
};

const int DOZEN_1_12 = 0xAAA;
const int DOZEN_13_24 = 0xBBB;
const int DOZEN_25_36 = 0xCCC;

const int EVEN = 0xDDD;
const int ODD = 0xEEE;

const int ZERO = 0x999;


class Roulette
{
    public:
                        Roulette();
        Result          roll();
        void            pay_to_players();
        void            add_player(Player*);
        string          to_string();
        void            print_players();
    protected:
        static bool     did_player_win(Player*);

    private:
        Result           _result;
        vector<Player*>  _players;
};