#include <iostream>
#include <fstream>
#include <sstream>
#include "Solver.h"

//#define CONSOLE

using namespace std;

vector<string> tokenize(const string originalString, const char delimiter);

int main(int argc, char* argv[])
{
	ifstream in("data.txt");
	string fileString;

	int width, height;

	vector<string> tokenizedString;

	if(in.is_open())
	{
		in.seekg(0, ios::end);
		int fileSize = in.tellg();

		fileString.resize(fileSize);

		in.seekg(0, ios::beg);

		in.read(&fileString[0], fileSize);

		tokenizedString = tokenize(fileString, '\n');

		width = stoi(tokenizedString[0]);
		height = stoi(tokenizedString[1]);

		if(tokenizedString.size() < width + height + 2)
		{
			cout << "파일이 형식에 맞지 않음." << endl;
			return -1;
		}
	}
	else
	{
		cout << "\"data.txt\" 파일을 찾을 수 없습니다." << endl;
		return -1;
	}

	vector<vector<int>> verticalBlockLengths;
	verticalBlockLengths.reserve(width);
	for(int i = 0; i < width; i++)
	{
		vector<string> tokenizedLengths = tokenize(tokenizedString[i + 2], ' ');

		vector<int> verticalBlockLength;
		verticalBlockLength.reserve(tokenizedLengths.size());

		for(string tokenizedLength : tokenizedLengths)
		{
			verticalBlockLength.push_back(stoi(tokenizedLength));
		}

		verticalBlockLengths.push_back(verticalBlockLength);
	}

	vector<vector<int>> horizentalBlockLengths;
	horizentalBlockLengths.reserve(width);
	for(int i = 0; i < width; i++)
	{
		vector<string> tokenizedLengths = tokenize(tokenizedString[i + 2 + width], ' ');

		vector<int> horizentalBlockLength;
		horizentalBlockLength.reserve(tokenizedLengths.size());

		for(string tokenizedLength : tokenizedLengths)
		{
			horizentalBlockLength.push_back(stoi(tokenizedLength));
		}

		horizentalBlockLengths.push_back(horizentalBlockLength);
	}

	solver s(width, height, verticalBlockLengths, horizentalBlockLengths);

	solver* preMap = new solver(s);

	while(!s.isMapClear())
	{
		for(int i = 0; i < height; i++)
		{
			s.solveOneVerticalLine(i);

#ifdef CONSOLE
			//system("cls");

			cout << endl << "=================================" << endl;
			s.print();
			cout << endl << "=================================" << endl;

			//system("pause>nul");
#endif // CONSOLE
	}

		for(int i = 0; i < width; i++)
		{
			s.solveOneHorizentalLine(i);

#ifdef CONSOLE
			//system("cls");

			cout << endl << "=================================" << endl;
			s.print();
			cout << endl << "=================================" << endl;

			//system("pause>nul");
#endif // CONSOLE
		}

#ifdef CONSOLE
		//s.print();
		//cout << "============================" << endl;
		//preMap->print();

		//system("pause>nul");
		//system("cls");
#endif // CONSOLE

#ifndef CONSOLE
		if(*preMap == s)
		{
			cout << "Cannot Solve." << endl;
			break;
		}
#endif // !CONSOLE

		delete preMap;
		preMap = new solver(s);
}

	delete preMap;

	s.print();

	return 0;
}

vector<string> tokenize(const string originalString, const char delimiter)
{
	vector<string> result;

	string token;

	stringstream stream(originalString);

	while(getline(stream, token, delimiter))
	{
		result.push_back(token);
	}

	return result;
}