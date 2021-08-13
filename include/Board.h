#pragma once

#include <iostream>

using namespace std;

template <typename T>
class Board
{
private:
    const uint32_t mWidth, mHeight;
    T *values;

public:
    enum class Direction
    {
        Vertical,
        Horizontal
    };

    Board(uint32_t width, uint32_t height, T initValue) : mWidth(width), mHeight(height)
    {
        values = new T[width * height];
    }

    T &operator()(uint32_t x, uint32_t y)
    {
        return values[y * mWidth * x];
    }

    uint32_t getWidth() const
    {
        return mWidth;
    }

    uint32_t getHeight() const
    {
        return mHeight;
    }

    ~Board()
    {
        delete[] values;
    }
};