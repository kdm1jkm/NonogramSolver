#include <iostream>
#include <fstream>
#include <sstream>
#include <Windows.h>
#include "Solver.h"

using namespace std;

vector<string> tokenize(const string &originalString, char delimiter);

int main(int argc, char *argv[]) {
    if (argc < 3) {
        cout << "Usage: " << argv[0] << " <file> <millisecond>" << endl;
        return EXIT_FAILURE;
    }
    ifstream in(argv[1]);

    string fileString;

    float interval = stof(argv[2]);

    int width, height;

    vector<string> tokenizedString;

    if (in.is_open()) {
        in.seekg(0, ios::end);
        int fileSize = in.tellg();

        fileString.resize(fileSize);

        in.seekg(0, ios::beg);

        in.read(&fileString[0], fileSize);

        tokenizedString = tokenize(fileString, '\n');

        width = stoi(tokenizedString[0]);
        height = stoi(tokenizedString[1]);

        if (tokenizedString.size() < width + height + 2) {
            cout << "Invalid file format" << endl;
            return -1;
        }
    } else {
        cout << "Can't read " << argv[1] << endl;
        return EXIT_FAILURE;
    }

    vector<vector<int>> verticalBlockLengths;
    verticalBlockLengths.reserve(width);
    for (int i = 0; i < width; i++) {
        vector<string> tokenizedLengths = tokenize(tokenizedString[i + 2], ' ');

        vector<int> verticalBlockLength;
        verticalBlockLength.reserve(tokenizedLengths.size());

        for (const string &tokenizedLength : tokenizedLengths) {
            verticalBlockLength.push_back(stoi(tokenizedLength));
        }

        verticalBlockLengths.push_back(verticalBlockLength);
    }

    vector<vector<int>> horizontalBlockLengths;
    horizontalBlockLengths.reserve(width);
    for (int i = 0; i < width; i++) {
        vector<string> tokenizedLengths = tokenize(tokenizedString[i + 2 + width], ' ');

        vector<int> horizontalBlockLength;
        horizontalBlockLength.reserve(tokenizedLengths.size());

        for (const string &tokenizedLength : tokenizedLengths) {
            horizontalBlockLength.push_back(stoi(tokenizedLength));
        }

        horizontalBlockLengths.push_back(horizontalBlockLength);
    }

    Solver s(width, height, verticalBlockLengths, horizontalBlockLengths);

    Solver preMap1(s);
    Solver preMap2(s);

    system("cls");
    s.print();

    while (!s.isMapClear()) {
        for (int i = 0; i < height; i++) {
            s.solveOneVerticalLine(i);

            if (s != preMap1) {
                Sleep(interval);
                system("cls");
                s.print();
            }
            preMap1.copyFrom(s);

            if (s.isMapClear())break;
        }

        for (int i = 0; i < width; i++) {
            s.solveOneHorizontalLine(i);

            if (s != preMap1) {
                Sleep(interval);
                system("cls");
                s.print();
            }
            preMap1.copyFrom(s);

            if (s.isMapClear())break;
        }

        if (preMap2 == s) {
            cout << "Can't Solve." << endl;
            break;
        }

        preMap2.copyFrom(s);
    }

    system("cls");
    s.print();

    return 0;
}

vector<string> tokenize(const string &originalString, const char delimiter) {
    vector<string> result;

    string token;

    stringstream stream(originalString);

    while (getline(stream, token, delimiter)) {
        result.push_back(token);
    }

    return result;
}