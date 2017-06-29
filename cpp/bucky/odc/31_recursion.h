//
// Created by umat on 30.06.17.
//

#ifndef BUCKY_31_RECURSION_H
#define BUCKY_31_RECURSION_H

int factorialFinder(int x){
    if (x==1){
        return 1;
    }
    else {
        return x * factorialFinder(x - 1);
    }
}

#endif //BUCKY_31_RECURSION_H
