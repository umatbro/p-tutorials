#include <iostream>
#include "coordin.h"

int main()
{
  using namespace std;
  rect rplace;
  polar pplace;

  cout << "Enter the x and y values: ";
  while (cin >> rplace.x >> rplace.y)
  {
    pplace = rect_to_polar(rplace);
    show_polar(pplace);
    cout << "Next two numbers (q to quit): ";
  }

  cout << "Done\n";
  return 0;
}
