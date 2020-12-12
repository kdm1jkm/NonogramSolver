#pragma once

#include <vector>
#include <iostream>
#include <memory>

using namespace std;

class solver {
 public:
  //각 블럭의 상태를 나타내는 enum
  enum class cell {
    block = 0b10,
    blank = 0b01,
    crash = 0b11,
    none = 0b00
  };

  //기본 생성자
  solver(int width, int height, vector<vector<int>> verticalBlockLengths, vector<vector<int>> horizentalBlockLengths);

  //복사 생성자
  solver(const solver &original);

  //줄의 상태와 블럭 정보를 받아 확실히 놓거나 놓지 않아야 할 곳을 알려주는 함수
  static vector<cell> solveLine(vector<cell> line, vector<int> blockLengths);

  //줄의 길이와 블럭 정보를 받아 블럭을 놓을 수 있는 모든 경우의 수를 나타내주는 함수
  static vector<vector<cell>> getEveryCellCombination(vector<int> blockLength, int lineLength);

  //한 줄을 출력해주는 함수
  static void printOneLine(const vector<cell> &line);

  //전체 맵을 인자로 받아 출력해주는 함수
  static void printMap(const vector<vector<cell>> &map);

  //나 자신을 출력하는 함수
  void print();

  //가로 한 줄을 반환하는 함수
  vector<cell> getOneVerticalLine(int num);

  //세로 한 줄을 반환하는 함수
  vector<cell> getOneHorizetalLine(int num);

  //가로 한 줄을 설정하는 함수
  void setOneVerticalLine(int num, const vector<cell> &line);

  //세로 한 줄을 설정하는 함수
  void setOneHorizentalLine(int num, const vector<cell> &line);

  //가로 한 줄을 푸는 함수
  void solveOneVerticalLine(int num);

  //세로 한 줄을 푸는 함수
  void solveOneHorizentalLine(int num);

  //맵에 none, crash가 없는지 반환하는 함수
  bool isMapClear();

  //mMap을 vector로 바꿔서 반환하는 함수
  vector<vector<cell>> getMap();

  //두 객체가 같은지 판단하는 연산자 오버로딩
  bool operator==(const solver &right);

  //두 객체가 다른지 판단하는 연산자 오버로딩
  bool operator!=(const solver &right);

  //소멸자
  ~solver();

 private:
  //너비와 높이
  const int mWidth, mHeight;
  //블럭 정보
  const vector<vector<int>> mHorizentalBlockLengths, mVerticalBlockLengths;
  //현재 상태(맵)
  cell *mMap;
};

//cell 비트 연산자 오버로딩
static solver::cell operator|(solver::cell left, solver::cell right);