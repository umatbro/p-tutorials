#pragma once
#include <string>

using std::string;

enum BetType
{
    STRAIGHT_UP,
    DOZEN,
    PARITY
};

class Player
{
    public:
        Player(string name, int init_money=0) : 
            _name(name),
            _money(init_money) {}
        void        bet(BetType, int);
        int         get_bet();
        BetType     get_bet_type();
        string      to_string();

    private:
        string      _name;
        int         _bet;
        int         _money;
};