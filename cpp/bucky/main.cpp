#include <iostream>
#include "odc/namespaces.cpp"
#include "odc/12_13_14_introduction_to_objects.h"
#include "odc/27_random_number_generator.h"
#include "odc/31_recursion.h"
#include "odc/35_arrays_in_functions.h"
#include "odc/44_45_const_objects_member_initializers/Sally.h"

using namespace mat_std;

int main() {
    srand(time(NULL));

    Klasa obiekt("ELO");
    cout << obiekt.getName() << endl;
    obiekt.coolSaying();

    obiekt.setName("Sir Ser");
    cout << obiekt.getName() << endl;
    cout << "Losowa liczba z przedziału 10-13: " << rndnum(10,14) << endl;

    int tab[5];
    for (int i = 0; i<5; i++){
        tab[i] = 0;
    }
    for (int i = 0; i < 10000; i++){
        int rnd = rndnum(10,14);
        switch(rnd){
            case 10:
                tab[0]++;
                break;
            case 11:
                tab[1]++;
                break;
            case 12:
                tab[2]++;
                break;
            case 13:
                tab[3]++;
                break;
            case 14:
                tab[4]++;
                break;
        }
    }

    cout << "10: " << tab[0] << endl;
    cout << "11: " << tab[1] << endl;
    cout << "12: " << tab[2] << endl;
    cout << "13: " << tab[3] << endl;
    cout << "14: " << tab[4] << endl;
    cout << "----------" << endl;
    cout << "Recursion" << endl;
    cout << "Factorial 5: " << factorialFinder(5) << endl;

    cout << "Print array: " << endl;
    printArray(tab, 5);
    cout << "Size of array: " << sizeof(tab) / sizeof(tab[0]) << endl;
    cout << "sizeof(int): " << sizeof(int) << endl;

    Sally constObj(3,4);
    constObj.printShiz2();
    constObj.print();
    return 0;
}