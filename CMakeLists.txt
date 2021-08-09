cmake_minimum_required(VERSION 3.17)
project(NonogramSolver)

# C++17
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# 루트 폴더에 실행
set(EXECUTABLE_OUTPUT_PATH ${CMAKE_SOURCE_DIR})

# 소스파일
file(GLOB sources
"src/*.cpp"
)

# 정적 파일 생성
set(CMAKE_EXE_LINKER_FLAGS "-static")

# 헤더파일
include_directories(${CMAKE_SOURCE_DIR}/include)

# 컴파일
add_executable(NonogramSolver ${sources})