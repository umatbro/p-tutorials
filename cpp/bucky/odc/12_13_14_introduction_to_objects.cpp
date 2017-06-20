//
// Created by umat on 20.06.17.
// video no 12 & 13 & 14
// access modifiers, constructors etc
//
#include <iostream>
#include <string>
using namespace std;

class Klasa {
private:
    string name;
public:
    Klasa(string n){
        setName(n);
    }
    void coolSaying(){
        cout << "Preaching to the choir" << endl;
    }
    void setName(string n){
        name = n;
    }
    string getName(){
        return name;
    }
};
