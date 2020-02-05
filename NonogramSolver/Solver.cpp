#include "Solver.h"

Solver::Solver(int width, int height, vector<vector<int>> verticalNum, vector<vector<int>> horizentalNum)
	:mWidth(width), mHeight(height), mHorizentalNum(horizentalNum), mVerticalNum(verticalNum), map(new cell[width * height]) {}

Solver::~Solver()
{
	delete[] map;
}
