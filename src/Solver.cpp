#include <algorithm>
#include <numeric>

#include "Solver.h"

using namespace std;

typedef vector<Solver::Cell> CellLine;

vector<CellLine> getPossibilities(const vector<unsigned int> &cell, uint32_t lineLength)
{
    vector<CellLine> result;

    if (cell.size() == 0)
    {
        CellLine line;
        line.assign(lineLength, Solver::Cell::blank);
        result.push_back(line);
    }
    else if (cell.size() == 1)
    {
        uint32_t length = cell[0];

        // 5칸에 4개짜리면 2개 넣을 수 있음 (5 - 4 + 1 = 2)
        for (uint32_t startPos = 0; startPos < lineLength - length + 1; startPos++)
        {
            CellLine line;
            line.assign(lineLength, Solver::Cell::blank);

            for (uint32_t i = 0; i < length; i++)
            {
                line[startPos + i] = Solver::Cell::block;
            }

            result.push_back(line);
        }
    }
    else
    {
        const auto remainingLength = cell[0];

        // 블록길이 합 + 마지막거 빼고 사이사이 간격
        const auto otherLengthSum = accumulate(cell.begin() + 1, cell.end(), 0) + (cell.size() - 2);
        const auto otherCell = vector<unsigned int>(cell.begin() + 1, cell.end());

        // 필요한 최소 칸 수?
        const auto wholeLength = remainingLength + otherLengthSum + 1;

        // startPos는 뒤쪽 조합들(otherLengthSum)의 시작 위치. remainingLength + 1부터 시작(한칸 띄우고 시작)
        // 해서 길이를 생각했을 때 끝까지(lineLength - otherLengthSum)까지 반복.
        //
        // startPos - 1 - x = remainingLength
        // x = startPos - 1 - remainingLength
        //
        // value            range                                           length                          startPos = remainingLangth + 1      startPos = lineLength - otherLengthSome
        // Solver::blank    [0, startPos - 1 - remainingLength)             startPos - 1 - remainingLength  0                                   lineLength - otherLengthSum - 1 - remainingLength
        // Solver::block    [startPos - 1 - remainingLength, startPos - 1)  remainingLength                 remainingLength                     remainingLength
        // Solver::blank    [startPos - 1, startPos)                        1                               1                                   1
        // otherResults     [startPos, lineLength)                          lineLength - startPos           lineLength - remainingLength - 1    otherLengthSum
        // sum                                                              lineLength                      lineLength                          lineLength
        for (uint32_t startPos = remainingLength + 1; startPos <= lineLength - otherLengthSum; startPos++)
        {
            auto blanks = CellLine(startPos - 1 - remainingLength, Solver::Cell::blank);
            auto blocks = CellLine(remainingLength, Solver::Cell::block);

            CellLine line;

            line.insert(line.end(), blanks.begin(), blanks.end());
            line.insert(line.end(), blocks.begin(), blocks.end());
            line.push_back(Solver::blank);

            const auto otherResults = getPossibilities(otherCell, lineLength - startPos);
            for_each(otherResults.begin(), otherResults.end(), [&line, &result](CellLine otherResult)
                     {
                         auto l = CellLine(line);
                         l.insert(l.end(), otherResult.begin(), otherResult.end());
                         result.push_back(l);
                     });
        }
    }

    return result;
}

Solver::Solver(uint32_t width, uint32_t height) : mBoard(width, height, Cell::none)
{
}

Solver::SolveResult Solver::solveLine(const vector<unsigned int> &cell, Direction direction)
{
}

uint32_t Solver::getLineLength(Direction direction)
{
    switch (direction)
    {
    case Direction::Horizontal:
        return mBoard.getWidth();

    case Direction::Vertical:
        return mBoard.getHeight();

    default:
        return 0;
    }
}

bool Solver::isMapClear()
{
    for (uint32_t y = 0; y < mBoard.getHeight(); y++)
    {
        for (uint32_t x = 0; x < mBoard.getWidth(); x++)
        {
            if (mBoard(x, y) == Cell::none || mBoard(x, y) == Cell::crash)
                return false;
        }
    }

    return true;
}
