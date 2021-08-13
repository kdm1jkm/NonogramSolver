﻿using System;
using System.Collections.Generic;
using System.Linq;

namespace NonogramSolverLib
{
    public class Board<T>
    {
        public enum Direction
        {
            VERTICAL,
            HORIZONTAL
        }

        private readonly T[] _values;

        public Board(int width, int height, T initValue)
        {
            Width = width;
            Height = height;
            _values = new T[width * height];
            Array.Fill(_values, initValue);
        }

        public int Width { get; }
        public int Height { get; }

        public T this[int x, int y]
        {
            get => _values[y * Width + x];
            set => _values[y * Width + x] = value;
        }

        public T this[int i] => _values[i];

        public bool Any(Func<T, bool> predicate)
        {
            return _values.Any(predicate);
        }


        public IEnumerable<T> GetLine(int index, Direction direction)
        {
            switch (direction)
            {
                case Direction.VERTICAL:
                    for (var i = 0; i < Height; i++) yield return this[index, i];

                    break;

                case Direction.HORIZONTAL:
                    for (var i = 0; i < Width; i++) yield return this[i, index];

                    break;
                default:
                    throw new ArgumentOutOfRangeException(nameof(direction), direction, null);
            }
        }

        public void SetLine(int index, Direction direction, List<T> line)
        {
            if (direction == Direction.VERTICAL && line.Count != Height)
                throw new ArgumentException($"line.Count and Height must be same, but {line.Count != Height}");
            if (direction == Direction.HORIZONTAL && line.Count != Width)
                throw new ArgumentException($"line.Count and Width must be same, but {line.Count != Width}");

            switch (direction)
            {
                case Direction.VERTICAL:
                    for (var i = 0; i < Height; i++) this[index, i] = line[i];

                    break;

                case Direction.HORIZONTAL:
                    for (var i = 0; i < Width; i++) this[i, index] = line[i];

                    break;
                default:
                    throw new ArgumentOutOfRangeException(nameof(direction), direction, null);
            }
        }
    }
}