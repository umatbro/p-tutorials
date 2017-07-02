//
// Created by umat on 02.07.17.
//

#ifndef BUCKY_SALLY_H
#define BUCKY_SALLY_H

class Sally {
private:
    int regVar; // regular variable
    const int constVar; // constant variable
public:
    Sally(int a, int b);
    void print();
    void printShiz();
    void printShiz2() const;
};


#endif //BUCKY_SALLY_H
