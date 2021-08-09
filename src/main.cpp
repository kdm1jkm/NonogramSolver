#include <iostream>
#include <fstream>
#include <sstream>
#include <Windows.h>
#include "Solver.h"

std::vector<std::string> tokenize(const std::string &originalString, char delimiter);

int main(int argc, char *argv[])
{
    if (argc < 3)
    {
        std::cout << "Usage: " << argv[0] << " <file> <millisecond>" << std::endl;
        return EXIT_FAILURE;
    }
    std::ifstream in(argv[1]);

    std::string fileString;

    float interval = std::stof(argv[2]);

    int width, height;

    std::vector<std::string> tokenizedString;

    if (in.is_open())
    {
        in.seekg(0, std::ios::end);
        int fileSize = in.tellg();

        fileString.resize(fileSize);

        in.seekg(0, std::ios::beg);

        in.read(&fileString[0], fileSize);

        tokenizedString = tokenize(fileString, '\n');

        width = stoi(tokenizedString[0]);
        height = stoi(tokenizedString[1]);

        if (tokenizedString.size() < width + height + 2)
        {
            std::cout << "Invalid file format" << std::endl;
            return -1;
        }
    }
    else
    {
        std::cout << "Can't read " << argv[1] << std::endl;
        return EXIT_FAILURE;
    }

    std::vector<std::vector<int>> verticalBlockLengths;
    verticalBlockLengths.reserve(width);
    for (int i = 0; i < width; i++)
    {
        std::vector<std::string> tokenizedLengths = tokenize(tokenizedString[i + 2], ' ');

        std::vector<int> verticalBlockLength;
        verticalBlockLength.reserve(tokenizedLengths.size());

        for (const std::string &tokenizedLength : tokenizedLengths)
        {
            verticalBlockLength.push_back(stoi(tokenizedLength));
        }

        verticalBlockLengths.push_back(verticalBlockLength);
    }

    std::vector<std::vector<int>> horizontalBlockLengths;
    horizontalBlockLengths.reserve(width);
    for (int i = 0; i < width; i++)
    {
        std::vector<std::string> tokenizedLengths = tokenize(tokenizedString[i + 2 + width], ' ');

        std::vector<int> horizontalBlockLength;
        horizontalBlockLength.reserve(tokenizedLengths.size());

        for (const std::string &tokenizedLength : tokenizedLengths)
        {
            horizontalBlockLength.push_back(stoi(tokenizedLength));
        }

        horizontalBlockLengths.push_back(horizontalBlockLength);
    }

    Solver s(width, height, verticalBlockLengths, horizontalBlockLengths);

    Solver preMap1(s);
    Solver preMap2(s);

    system("cls");
    s.print();

    while (!s.isMapClear())
    {
        for (int i = 0; i < height; i++)
        {
            s.solveOneVerticalLine(i);

            if (s != preMap1)
            {
                Sleep(interval);
                system("cls");
                s.print();
            }
            preMap1.copyFrom(s);

            if (s.isMapClear())
                break;
        }

        for (int i = 0; i < width; i++)
        {
            s.solveOneHorizontalLine(i);

            if (s != preMap1)
            {
                Sleep(interval);
                system("cls");
                s.print();
            }
            preMap1.copyFrom(s);

            if (s.isMapClear())
                break;
        }

        if (preMap2 == s)
        {
            std::cout << "Can't Solve." << std::endl;
            break;
        }

        preMap2.copyFrom(s);
    }

    system("cls");
    s.print();

    return 0;
}

std::vector<std::string> tokenize(const std::string &originalString, const char delimiter)
{
    std::vector<std::string> result;

    std::string token;

    std::stringstream stream(originalString);

    while (getline(stream, token, delimiter))
    {
        result.push_back(token);
    }

    return result;
}