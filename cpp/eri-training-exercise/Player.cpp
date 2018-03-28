#include "Player.h"
#include <iostream>
#include <string>

using std::cout;
using std::endl;
using std::string;

string Player::to_string()
{
    return "Player name: " + this -> _name;
}