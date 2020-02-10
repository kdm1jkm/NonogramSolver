#pragma once

#include <vector>
#include <iostream>
#include <memory>

using namespace std;

class solver
{
public:
	//�� ���� ���¸� ��Ÿ���� enum
	enum class cell
	{
		block = 0b10,
		blank = 0b01,
		crash = 0b11,
		none = 0b00
	};

	//�⺻ ������
	solver(int width, int height, vector<vector<int>> verticalBlockLengths, vector<vector<int>> horizentalBlockLengths);
	//���� ������
	solver(const solver& original);

	//���� ���¿� �� ������ �޾� Ȯ���� ���ų� ���� �ʾƾ� �� ���� �˷��ִ� �Լ�
	static vector<cell> solveLine(vector<cell> line, vector<int> blockLengths);
	//���� ���̿� �� ������ �޾� ���� ���� �� �ִ� ��� ����� ���� ��Ÿ���ִ� �Լ�
	static vector<vector<cell>> getEveryCellCombination(vector<int> blockLength, int lineLength);

	//�� ���� ������ִ� �Լ�
	static void printOneLine(vector<cell> line);
	//��ü ���� ���ڷ� �޾� ������ִ� �Լ�
	static void printMap(vector<vector<cell>> map);

	//�� �ڽ��� ����ϴ� �Լ�
	void print();

	//���� �� ���� ��ȯ�ϴ� �Լ�
	vector<cell> getOneVerticalLine(int num);
	//���� �� ���� ��ȯ�ϴ� �Լ�
	vector<cell> getOneHorizetalLine(int num);

	//���� �� ���� �����ϴ� �Լ�
	void setOneVerticalLine(int num, const vector<cell> line);
	//���� �� ���� �����ϴ� �Լ�
	void setOneHorizentalLine(int num, const vector<cell> line);

	//���� �� ���� Ǫ�� �Լ�
	void solveOneVerticalLine(int num);
	//���� �� ���� Ǫ�� �Լ�
	void solveOneHorizentalLine(int num);

	//�ʿ� none, crash�� ������ ��ȯ�ϴ� �Լ�
	bool isMapClear();

	//mMap�� vector�� �ٲ㼭 ��ȯ�ϴ� �Լ�
	vector<vector<cell>> getMap();

	//�� ��ü�� ������ �Ǵ��ϴ� ������ �����ε�
	bool operator==(solver right);
	//�� ��ü�� �ٸ��� �Ǵ��ϴ� ������ �����ε�
	bool operator!=(solver right);

	//�Ҹ���
	~solver();

private:
	//�ʺ�� ����
	const int mWidth, mHeight;
	//�� ����
	const vector<vector<int>> mHorizentalBlockLengths, mVerticalBlockLengths;
	//���� ����(��)
	cell* mMap;
};

//cell ��Ʈ ������ �����ε�
static solver::cell operator|(solver::cell left, solver::cell right);