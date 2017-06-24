//
// Created by umat on 24.06.17.
//
#include "27_random_number_generator.h"

int rndnum(int range_begin, int range_end){
    return (rand()%(range_end - range_begin + 1) + range_begin);
}