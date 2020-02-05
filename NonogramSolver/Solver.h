#pragma once

#include <vector>
#include <iostream>

using namespace std;

class Solver
{
private:
	enum class cell {
		TRUE,
		FALSE,
		NONE
	};
	const int mWidth, mHeight;
	const vector<vector<int>> mHorizentalNum, mVerticalNum;
	const cell* map;
public:
	Solver(int width, int height, vector<vector<int>> verticalNum, vector<vector<int>> horizentalNum);

	~Solver();
};

