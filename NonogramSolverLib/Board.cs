using System;
using System.Collections.Generic;
using System.Linq;

namespace NonogramSolverLib
{
    public class Board<T>
    {
        private readonly T[] _values;
        public readonly int Height;
        public readonly int Width;

        public Board(int width, int height, T initValue)
        {
            Width = width;
            Height = height;
            _values = Enumerable.Repeat(initValue, width * height).ToArray();
        }

        public T this[int x, int y]
        {
            get => _values[y * Width + x];
            private set => _values[y * Width + x] = value;
        }

        public bool Any(Func<T, bool> predicate)
        {
            return _values.Any(predicate);
        }

        public IEnumerable<T> GetLine(int index, Direction direction)
        {
            if (direction == Direction.VERTICAL)
                for (var i = 0; i < Height; i++)
                    yield return this[index, i];
            else
                for (var i = 0; i < Width; i++)
                    yield return this[i, index];
        }

        public void SetLine(int index, Direction direction, List<T> line)
        {
            // ReSharper disable once ConvertIfStatementToSwitchStatement
            if (direction == Direction.VERTICAL && line.Count != Height)
                throw new ArgumentException($"line.Count and Height must be same, but {line.Count != Height}");
            if (direction == Direction.HORIZONTAL && line.Count != Width)
                throw new ArgumentException($"line.Count and Width must be same, but {line.Count != Width}");

            if (direction == Direction.VERTICAL)
                for (var i = 0; i < Height; i++)
                    this[index, i] = line[i];
            else
                for (var i = 0; i < Width; i++)
                    this[i, index] = line[i];
        }
    }
}