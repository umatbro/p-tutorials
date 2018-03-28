#include "Roulette.h"
#include "Player.h"

#include <time.h>
#include <stdlib.h>
#include <iostream>
#include <string>

using std::cout;
using std::endl;
using std::cin;

Roulette::Roulette()
{
    cout << "Roulette : init" << endl;
    cout << "Please input number of players: ";
    int num_of_players;
    cin >> num_of_players;
    
    for (int i = 0; i < num_of_players; i++)
    {
        string name;
        cout << "Input name (player " << i << "): ";
        cin >> name;
        Player* p = new Player(name);
        this -> _players.push_back(p);
    }
}

/**
 * Roll random number
 * 
 * @return Result containing info about rolled number,
 * whether it is odd or in which dozen it belongs.
 */
Result Roulette::roll()
{
    srand(time(NULL));
    int rolled_number = rand() % 37;

    cout << "Roulette : rolled number: " << rolled_number << endl;

    Result result;
    result.number = rolled_number;
    result.parity = rolled_number % 2 == 0 ? EVEN : ODD;
    if (rolled_number == 0)
    {
        result.dozen = ZERO;
        result.is_zero = true;

        this -> _result = result;
        return result;
    }

    // we know result is not 0
    result.is_zero = false;
    
    if (0 < rolled_number <= 12)
    {
        result.dozen = DOZEN_1_12;
    }
    if (12 < rolled_number <= 24)
    {
        result.dozen = DOZEN_13_24;
    }
    if (24 < rolled_number <= 36)
    {
        result.dozen = DOZEN_25_36;
    }

    this -> _result = result;
    return result;
}


string Roulette::to_string()
{
    Result roll_res = this -> _result;
    string result = "Number: ";
    result += std::to_string(roll_res.number);
    if (roll_res.is_zero) 
    {
        result += "\n  ZERO";
        return result;
    }

    result += "\n  > is_even: ";
    result += (roll_res.parity == EVEN ? "true" : "false");
        
    switch(roll_res.dozen)
    {
        case DOZEN_1_12:
            result += "\n  > dozen: 1-12";
            return result;
        case DOZEN_13_24:
            result += "\n  > dozen: 13-24";
            return result;
        case DOZEN_25_36:
            result += "\n  > dozen 25-36";
            return result;
    }
    return result;
}

void Roulette::print_players()
{
    for (int i = 0; i < this->_players.size(); i++) 
    {
        string to_print = (this -> _players[i]) -> to_string();
        cout << to_print << endl;
    }
}