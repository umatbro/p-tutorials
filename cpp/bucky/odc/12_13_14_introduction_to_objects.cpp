//
// Created by umat on 20.06.17.
// video no 12 & 13 & 14
// access modifiers, constructors etc
//
#include "12_13_14_introduction_to_objects.h"
Klasa::Klasa(string n) {
    setName(n);
}

void Klasa::coolSaying(){
    cout << "Preaching to the choir" << endl;
}

void Klasa::setName(string n) {
    name = n;
}

string Klasa::getName(){
    return name;
}