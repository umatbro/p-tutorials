//
// Created by umat on 02.07.17.
//

#include "Sally.h"
#include <iostream>

void Sally::printShiz(){
    std::cout << "I am regular function" << std::endl;
}

Sally::Sally(int a, int b)
: regVar(a), constVar(b)
{

}

void Sally::printShiz2() const {
    std::cout << "I am a constant function" << std::endl;
}

void Sally::print(){
    std::cout << "Regular var is: " << this->regVar << std::endl;
    std::cout << "Const var is : " << this->constVar << std::endl;
}