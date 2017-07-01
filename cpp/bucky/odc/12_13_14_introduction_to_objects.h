//
// Created by umat on 22.06.17.
//

#ifndef BUCKY_12_13_14_INTRODUCTION_TO_OBJECTS_H
#define BUCKY_12_13_14_INTRODUCTION_TO_OBJECTS_H
#include <iostream>
#include <string>
#include "namespaces.cpp"
using namespace mat_std;

class Klasa {
private:
    string name;
public:
    Klasa(string n);
    void coolSaying();
    void setName(string n);
    string getName();
};

#endif //BUCKY_12_13_14_INTRODUCTION_TO_OBJECTS_H
