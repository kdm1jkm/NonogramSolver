#pragma once

#include <iostream>
#include <vector>

#include "Board.h"

using namespace std;

class Solver
{
public:
    enum class Direction
    {
        Horizontal,
        Vertical
    };

    //각 블럭의 상태를 나타내는 enum
    enum Cell
    {
        block = 0b10,
        blank = 0b01,
        crash = 0b11,
        none = 0b00
    };

    struct SolveResult
    {
        const uint32_t changeCount;
        const vector<uint32_t> changePos;
        SolveResult(uint32_t changeCount, vector<uint32_t> changePos) : changeCount(changeCount), changePos(changePos) {}
    };

public:
    //기본 생성자
    Solver(uint32_t width, uint32_t height);

    //한 줄을 푸는 함수
    SolveResult solveLine(const vector<unsigned int> &cell, Direction direction);

    uint32_t getLineLength(Direction direction);

    //맵에 none, crash가 없는지 반환하는 함수
    bool isMapClear();

private:
    //현재 상태(맵)
    Board<Cell> mBoard;
};
