cmake_minimum_required(VERSION 3.17)
project(NonogramSolver)

set(CMAKE_CXX_STANDARD 14)

add_executable(NonogramSolver main.cpp Solver.cpp Solver.h)