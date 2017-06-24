//
// Created by umat on 24.06.17.
//

#ifndef BUCKY_27_RANDOM_NUMBER_GENERATOR_H
#define BUCKY_27_RANDOM_NUMBER_GENERATOR_H
#include <cstdlib>
#include <ctime>

/**
 * Function to return random number within given range
 * Use random seed srand() before calling this function
 * @param range_begin (included)
 * @param range_end (included)
 * @return random number within given range
 */
int rndnum(int range_begin, int range_end);

#endif //BUCKY_27_RANDOM_NUMBER_GENERATOR_H
