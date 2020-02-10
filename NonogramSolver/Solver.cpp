#include "Solver.h"

//#define CONSOLE


solver::solver(int width, int height, vector<vector<int>> verticalBlockLengths, vector<vector<int>> horizentalBlockLengths):
	mWidth(width), mHeight(height),
	mHorizentalBlockLengths(horizentalBlockLengths), mVerticalBlockLengths(verticalBlockLengths),
	mMap(new cell[width * height])
{
	for(int i = 0; i < mWidth * mHeight; i++)
	{
		mMap[i] = cell::none;
	}
}

solver::solver(const solver& original):
	mWidth(original.mWidth), mHeight(original.mHeight),
	mHorizentalBlockLengths(original.mHorizentalBlockLengths), 
	mVerticalBlockLengths(original.mVerticalBlockLengths)
{
	mMap = new cell[mWidth * mHeight];

	for(int i = 0; i < mWidth * mHeight; i++)
	{
		mMap[i] = original.mMap[i];
	}
}

vector<solver::cell> solver::solveLine(vector<cell> line, vector<int> blockLengths)
{
	//��� ������ �����´�
	vector<vector<cell>> everyCombinations = getEveryCellCombination(blockLengths, line.size());

	//� ���Ҹ� �ٲ��� ����
	vector<cell> change;
	//none�� �ٸ� cell�� |�����ϸ� �� ���°� ��
	change.assign(line.size(), cell::none);

	//�����
#ifdef CONSOLE

	for(int length : blockLengths)
	{
		cout << length << endl;
	}

	printOneLine(line);
	cout << endl;

#endif // CONSOLE

	//�� ���տ� ����
	for(unsigned int i = 0; i < everyCombinations.size(); i++)
	{
		//�浹�� �ִ°�?
		bool isThereCrash = false;

		//�� ���� ��
		for(unsigned int j = 0; j < line.size(); j++)
		{
			//�浹�� �ִ°�?
			if((line[j] | everyCombinations[i][j]) == cell::crash)
			{
				isThereCrash = true;
			}
		}

		//���� ���ΰ� �浹�� �ִ� ������ �ǳʶ�
		if(isThereCrash)
		{
			continue;
		}

		//�����
#ifdef CONSOLE
		printOneLine(everyCombinations[i]);
#endif // CONSOLE

		//��ȭ�� ��Ʈ����(block, blank��ΰ� change�� ��Ʈ����� �κ��� crash�� ���, ���� �ݿ��� ���� ����)
		for(unsigned int j = 0; j < line.size(); j++)
		{
			change[j] = change[j] | everyCombinations[i][j];
		}
	}

	//�����
#ifdef CONSOLE
	cout << endl;
	printOneLine(line);
	printOneLine(change);
#endif // CONSOLE

	//�� ���ҿ� ����
	for(unsigned int i = 0; i < line.size(); i++)
	{
		//Ȯ���� �κи��� �ݿ�(change�� crash�� �ƴ� �κ�)
		if((line[i] | change[i]) != cell::crash)
		{
			line[i] = line[i] | change[i];
		}
	}
	
	//�����
#ifdef CONSOLE
	printOneLine(line);
	cout << "-----------------------------------" << endl;
#endif // CONSOLE

	return line;
}

vector<vector<solver::cell>> solver::getEveryCellCombination(vector<int> blockLengths, int lineLength)
{
	vector<vector<cell>> result;

	//���̰� 1�̸�
	if(blockLengths.size() == 1)
	{
		for(int i = 0; i < lineLength - blockLengths[0] + 1; i++)
		{
			//ó������ ������ ����� ��
			vector<cell> temp;
			temp.assign(lineLength, cell::blank);
			for(int j = i; j < blockLengths[0] + i; j++)
			{
				temp[j] = cell::block;
			}

			result.push_back(temp);
		}
	}
	//���̰� 2 �̻��̸�
	else
	{
		int blockLengthSum = 0;

		for(unsigned int i = 0; i < blockLengths.size(); i++)
		{
			blockLengthSum += blockLengths[i];
		}

		//�� ���� - (ù ��° ��� ���� - 1) - ������ ��� ���� �� - (��ϰ��� - 1)
		for(unsigned int i = 0; i < lineLength - (blockLengthSum - 1) - (blockLengths.size() - 1); i++)
		{
			vector<cell> frontwardCombination;
			frontwardCombination.assign(i + blockLengths[0], cell::blank);

			for(unsigned int j = i; j < i + blockLengths[0]; j++)
			{
				frontwardCombination[j] = cell::block;
			}

			vector<int> backwardBlockLengths(blockLengths.begin() + 1, blockLengths.end());
			vector<vector<cell>> backwardCombinations = getEveryCellCombination(backwardBlockLengths, lineLength - i - blockLengths[0] - 1);

			for(unsigned int j = 0; j < backwardCombinations.size(); j++)
			{
				vector<cell> temp;

				for(unsigned int k = 0; k < frontwardCombination.size(); k++)
				{
					temp.push_back(frontwardCombination[k]);
				}

				temp.push_back(cell::blank);

				for(unsigned int k = 0; k < backwardCombinations[j].size(); k++)
				{
					temp.push_back(backwardCombinations[j][k]);
				}

				result.push_back(temp);
			}
		}
	}

	return result;
}

void solver::printOneLine(vector<cell> line)
{
	for(unsigned int i = 0; i < line.size(); i++)
	{
		switch(line[i])
		{
		case cell::block:
			cout << "��";
			break;

		case cell::none:
			cout << "  ";
			break;

		case cell::blank:
			cout << "��";
			break;

		case cell::crash:
			cout << "��";
		}
	}
	cout << endl;
}

void solver::printMap(vector<vector<cell>> map)
{
	for(unsigned int i = 0; i < map.size(); i++)
	{
		printOneLine(map[i]);
	}
}

void solver::print()
{
	for(int i = 0; i < mHeight; i++)
	{
		for(int j = 0; j < mWidth; j++)
		{
			switch(mMap[j + i * mWidth])
			{
			case cell::block:
				cout << "��";
				break;

			case cell::none:
				//cout << "��";
				cout << "  ";
				break;

			case cell::blank:
				cout << "��";
				break;

			case cell::crash:
				cout << "��";
			}
		}
		cout << endl;
	}
}

vector<solver::cell> solver::getOneVerticalLine(int num)
{
	vector<cell> result;
	result.reserve(mWidth);

	for(int i = 0; i < mWidth; i++)
	{
		result.push_back(mMap[num * mWidth + i]);
	}

	return result;
}

vector<solver::cell> solver::getOneHorizetalLine(int num)
{
	vector<cell> result;
	result.reserve(mHeight);

	for(int i = 0; i < mHeight; i++)
	{
		result.push_back(mMap[i * mWidth + num]);
	}

	return result;
}

void solver::setOneVerticalLine(int num, const vector<cell> line)
{
	for(int i = 0; i < mWidth; i++)
	{
		mMap[num * mWidth + i] = line[i];
	}
}

void solver::setOneHorizentalLine(int num, const vector<cell> line)
{
	for(int i = 0; i < mHeight; i++)
	{
		mMap[i * mWidth + num] = line[i];
	}
}

void solver::solveOneVerticalLine(int num)
{
	this->setOneVerticalLine(num, solveLine(getOneVerticalLine(num), mVerticalBlockLengths[num]));
}

void solver::solveOneHorizentalLine(int num)
{
	this->setOneHorizentalLine(num, solveLine(getOneHorizetalLine(num), mHorizentalBlockLengths.at(num)));
}

bool solver::isMapClear()
{
	bool result = true;

	for(int i = 0; i < mWidth * mHeight; i++)
	{
		if(mMap[i] == cell::none || mMap[i] == cell::crash)
		{
			result = false;
		}
	}

	return result;
}

vector<vector<solver::cell>> solver::getMap()
{
	vector<vector<cell>> result(mHeight);

	for(int i = 0; i < mHeight; i++)
	{
		vector<cell> line(mWidth);

		for(int j = 0; j < mWidth; j++)
		{
			line.push_back(mMap[j + i * mWidth]);
		}

		result.push_back(line);
	}

	return result;
}

bool solver::operator==(solver right)
{
	if(mWidth != right.mWidth || mHeight != right.mHeight)
	{
		return false;
	}
	for(int i = 0; i < mWidth * mHeight; i++)
	{
		if(mMap[i] != right.mMap[i])
		{
			return false;
		}
	}

	return true;
}

bool solver::operator!=(solver right)
{
	return !(*this == right);
}

solver::~solver()
{
	delete[] mMap;
}

solver::cell operator|(solver::cell left, solver::cell right)
{
	return static_cast<solver::cell>(static_cast<int>(left) | static_cast<int>(right));
}
