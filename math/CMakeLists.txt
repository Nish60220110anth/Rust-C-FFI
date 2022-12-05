cmake_minimum_required(VERSION 3.25.1)
project(MyTestLib LANGUAGES C)
add_library(MyMathLib STATIC myTest.c)
install(TARGETS MyMathLib DESTINATION .)