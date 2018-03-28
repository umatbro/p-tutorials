#include <iostream>
#include "Roulette.h"
#include "Player.h"

using std::cout;
using std::endl;

int main(int argc, char** argv)
{
	Roulette roulette;
	roulette.print_players();
	// roulette.roll();
	// cout << roulette.to_string() << endl;
	return 0;
}
