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

        public IEnumerable<T> GetLine(LineInfo lineInfo)
        {
            if (lineInfo.Direction == Direction.VERTICAL)
                for (var i = 0; i < Height; i++)
                    yield return this[lineInfo.Index, i];
            else
                for (var i = 0; i < Width; i++)
                    yield return this[i, lineInfo.Index];
        }

        public void SetLine(LineInfo lineInfo, List<T> values)
        {
            // ReSharper disable once ConvertIfStatementToSwitchStatement
            if (lineInfo.Direction == Direction.VERTICAL && values.Count != Height)
                throw new ArgumentException($"line.Count and Height must be same, but {values.Count != Height}");
            if (lineInfo.Direction == Direction.HORIZONTAL && values.Count != Width)
                throw new ArgumentException($"line.Count and Width must be same, but {values.Count != Width}");

            if (lineInfo.Direction == Direction.VERTICAL)
                for (var i = 0; i < Height; i++)
                    this[lineInfo.Index, i] = values[i];
            else
                for (var i = 0; i < Width; i++)
                    this[i, lineInfo.Index] = values[i];
        }
    }
}