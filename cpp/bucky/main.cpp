#include <iostream>
#include "odc/12_13_14_introduction_to_objects.h"

int main() {
    Klasa obiekt("ELO");
    cout << obiekt.getName() << endl;
    obiekt.coolSaying();

    obiekt.setName("Sir Ser");
    cout << obiekt.getName() << endl;
    return 0;
}