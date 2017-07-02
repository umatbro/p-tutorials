//
// Created by umat on 02.07.17.
//

#include "Sally.h"
#include <iostream>

void Sally::printShiz(){
    std::cout << "I am regular function" << std::endl;
}

Sally::Sally() {

}

void Sally::printShiz2() const {
    std::cout << "I am a constant function" << std::endl;
}