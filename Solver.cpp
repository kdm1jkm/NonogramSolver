#include "Solver.h"


//#define CONSOLE


solver::solver(int width,
               int height,
               vector<vector<int>> verticalBlockLengths,
               vector<vector<int>> horizentalBlockLengths) :
        mWidth(width),
        mHeight(height),
        mHorizentalBlockLengths(std::move(horizentalBlockLengths)),
        mVerticalBlockLengths(std::move(verticalBlockLengths)),
        mMap(new cell[width * height]) {
    for (int i = 0; i < mWidth * mHeight; i++) {
        mMap[i] = cell::none;
    }
}

solver::solver(const solver &original) :
        mWidth(original.mWidth), mHeight(original.mHeight),
        mHorizentalBlockLengths(original.mHorizentalBlockLengths),
        mVerticalBlockLengths(original.mVerticalBlockLengths) {
    mMap = new cell[mWidth * mHeight];

    for (int i = 0; i < mWidth * mHeight; i++) {
        mMap[i] = original.mMap[i];
    }
}

vector<solver::cell> solver::solveLine(vector<cell> line, vector<int> blockLengths) {
    //모든 조합을 가져온다
    vector<vector<cell>> everyCombinations = getEveryCellCombination(std::move(blockLengths), line.size());

    //어떤 원소를 바꿀지 저장
    vector<cell> change;
    //none은 다른 cell과 |연산하면 그 상태가 됨
    change.assign(line.size(), cell::none);

    //디버깅
#ifdef CONSOLE

    for(int length : blockLengths)
    {
        cout << length << endl;
    }

    printOneLine(line);
    cout << endl;

#endif // CONSOLE

    //각 조합에 대해
    for (auto &everyCombination : everyCombinations) {
        //충돌이 있는가?
        bool isThereCrash = false;

        //각 원소 비교
        for (unsigned int j = 0; j < line.size(); j++) {
            //충돌이 있는가?
            if ((line[j] | everyCombination[j]) == cell::crash) {
                isThereCrash = true;
            }
        }

        //기존 라인과 충돌이 있는 조합은 건너뜀
        if (isThereCrash) {
            continue;
        }

        //디버깅
#ifdef CONSOLE
        printOneLine(everyCombinations[i]);
#endif // CONSOLE

        //변화에 비트연산(block, blank모두가 change에 비트연산된 부분은 crash로 기록, 추후 반영이 되지 않음)
        for (unsigned int j = 0; j < line.size(); j++) {
            change[j] = change[j] | everyCombination[j];
        }
    }

    //디버깅
#ifdef CONSOLE
    cout << endl;
    printOneLine(line);
    printOneLine(change);
#endif // CONSOLE

    //각 원소에 대해
    for (unsigned int i = 0; i < line.size(); i++) {
        //확실한 부분만을 반영(change가 crash가 아닌 부분)
        if ((line[i] | change[i]) != cell::crash) {
            line[i] = line[i] | change[i];
        }
    }

    //디버깅
#ifdef CONSOLE
    printOneLine(line);
    cout << "-----------------------------------" << endl;
#endif // CONSOLE

    return line;
}

vector<vector<solver::cell>> solver::getEveryCellCombination(vector<int> blockLengths, int lineLength) {
    vector<vector<cell>> result;

    //길이가 1이면
    if (blockLengths.size() == 1) {
        for (int i = 0; i < lineLength - blockLengths[0] + 1; i++) {
            //처음부터 끝까지 경우의 수
            vector<cell> temp;
            temp.assign(lineLength, cell::blank);
            for (int j = i; j < blockLengths[0] + i; j++) {
                temp[j] = cell::block;
            }

            result.push_back(temp);
        }
    }
        //길이가 2 이상이면
    else {
        int blockLengthSum = 0;

        for (int blockLength : blockLengths) {
            blockLengthSum += blockLength;
        }

        //줄 길이 - (첫 번째 블록 길이 - 1) - 나머지 블록 길이 합 - (블록갯수 - 1)
        for (unsigned int i = 0; i < lineLength - (blockLengthSum - 1) - (blockLengths.size() - 1); i++) {
            vector<cell> frontwardCombination;
            frontwardCombination.assign(i + blockLengths[0], cell::blank);

            for (unsigned int j = i; j < i + blockLengths[0]; j++) {
                frontwardCombination[j] = cell::block;
            }

            vector<int> backwardBlockLengths(blockLengths.begin() + 1, blockLengths.end());
            vector<vector<cell>> backwardCombinations = getEveryCellCombination(backwardBlockLengths,
                                                                                lineLength - i - blockLengths[0] - 1);

            for (auto &backwardCombination : backwardCombinations) {
                vector<cell> temp;

                temp.reserve(frontwardCombination.size());
                for (auto &k : frontwardCombination) {
                    temp.push_back(k);
                }

                temp.push_back(cell::blank);

                for (auto &k : backwardCombination) {
                    temp.push_back(k);
                }

                result.push_back(temp);
            }
        }
    }

    return result;
}

void solver::printOneLine(const vector<cell>& line) {
    for (auto &i : line) {
        switch (i) {
            case cell::block:cout << "■";
                break;

            case cell::none:cout << "  ";
                break;

            case cell::blank:cout << "×";
                break;

            case cell::crash:cout << "≠";
        }
    }
    cout << endl;
}

void solver::printMap(const vector<vector<cell>>& map) {
    for (auto &i : map) {
        printOneLine(i);
    }
}

void solver::print() {
    for (int i = 0; i < mHeight; i++) {
        for (int j = 0; j < mWidth; j++) {
            switch (mMap[j + i * mWidth]) {
                case cell::block:cout << "■";
                    break;

                case cell::none:
                    //cout << "□";
                    cout << "  ";
                    break;

                case cell::blank:cout << "×";
                    break;

                case cell::crash:cout << "≠";
            }
        }
        cout << endl;
    }
}

vector<solver::cell> solver::getOneVerticalLine(int num) {
    vector<cell> result;
    result.reserve(mWidth);

    for (int i = 0; i < mWidth; i++) {
        result.push_back(mMap[num * mWidth + i]);
    }

    return result;
}

vector<solver::cell> solver::getOneHorizetalLine(int num) {
    vector<cell> result;
    result.reserve(mHeight);

    for (int i = 0; i < mHeight; i++) {
        result.push_back(mMap[i * mWidth + num]);
    }

    return result;
}

void solver::setOneVerticalLine(int num, const vector<cell> &line) {
    for (int i = 0; i < mWidth; i++) {
        mMap[num * mWidth + i] = line[i];
    }
}

void solver::setOneHorizentalLine(int num, const vector<cell> &line) {
    for (int i = 0; i < mHeight; i++) {
        mMap[i * mWidth + num] = line[i];
    }
}

void solver::solveOneVerticalLine(int num) {
    this->setOneVerticalLine(num, solveLine(getOneVerticalLine(num), mVerticalBlockLengths[num]));
}

void solver::solveOneHorizentalLine(int num) {
    this->setOneHorizentalLine(num, solveLine(getOneHorizetalLine(num), mHorizentalBlockLengths.at(num)));
}

bool solver::isMapClear() {
    bool result = true;

    for (int i = 0; i < mWidth * mHeight; i++) {
        if (mMap[i] == cell::none || mMap[i] == cell::crash) {
            result = false;
        }
    }

    return result;
}

vector<vector<solver::cell>> solver::getMap() {
    vector<vector<cell>> result(mHeight);

    for (int i = 0; i < mHeight; i++) {
        vector<cell> line(mWidth);

        for (int j = 0; j < mWidth; j++) {
            line.push_back(mMap[j + i * mWidth]);
        }

        result.push_back(line);
    }

    return result;
}

bool solver::operator==(const solver &right) {
    if (mWidth != right.mWidth || mHeight != right.mHeight) {
        return false;
    }
    for (int i = 0; i < mWidth * mHeight; i++) {
        if (mMap[i] != right.mMap[i]) {
            return false;
        }
    }

    return true;
}

bool solver::operator!=(const solver &right) {
    return !(*this == right);
}

solver::~solver() {
    delete[] mMap;
}

solver::cell operator|(solver::cell left, solver::cell right) {
    return static_cast<solver::cell>(static_cast<int>(left) | static_cast<int>(right));
}