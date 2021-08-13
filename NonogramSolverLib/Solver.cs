using System.Collections.Generic;
using System.Linq;

namespace NonogramSolverLib
{
    public class Solver
    {
        public enum Cell
        {
            BLOCK = 0b10,
            BLANK = 0b01,
            CRASH = 0b11,
            NONE = 0b00
        }

        private readonly Board<Cell> _board;

        public Solver(int width, int height)
        {
            _board = new Board<Cell>(width, height, Cell.NONE);
        }

        public SolveResult SolveLine(List<int> cell, Board<Cell>.Direction direction)
        {
            return new SolveResult(new List<int> { 1, 2, 3, 4 });
        }

        public bool IsMapClear()
        {
            return !_board.Any(cell => cell is Cell.NONE or Cell.CRASH);
        }

        public static List<List<Cell>> GetPossibilities(List<int> cell, int lineLength)
        {
            List<List<Cell>> result = new List<List<Cell>>();

            if (cell.Count == 0)
            {
                List<Cell> line = Enumerable.Repeat(Cell.BLANK, lineLength).ToList();
                for (var i = 0; i < lineLength; i++) line.Add(Cell.BLANK);

                result.Add(line);
            }
            else if (cell.Count == 1)
            {
                int length = cell[0];

                // 5칸에 4개짜리면 2개 넣을 수 있음 (5 - 4 + 1 = 2)
                for (var startPos = 0; startPos < lineLength - length + 1; startPos++)
                {
                    List<Cell> line = Enumerable.Repeat(Cell.BLANK, lineLength).ToList();

                    for (var i = 0; i < length; i++) line[startPos + i] = Cell.BLOCK;

                    result.Add(line);
                }
            }
            else
            {
                int remainingLength = cell[0];

                // 블록길이 합 + 마지막거 빼고 사이사이 간격
                int otherLengthSum = cell.GetRange(1, cell.Count - 1).Sum() + (cell.Count - 2);
                List<int> otherCell = cell.GetRange(1, cell.Count - 1);

                // startPos는 뒤쪽 조합들(otherLengthSum)의 시작 위치. remainingLength + 1부터 시작(한칸 띄우고 시작)
                // 해서 길이를 생각했을 때 끝까지(lineLength - otherLengthSum)까지 반복.
                //
                // startPos - 1 - x = remainingLength
                // x = startPos - 1 - remainingLength
                //
                // value            range                                           length                          startPos = remainingLength + 1      startPos = lineLength - otherLengthSome
                // Solver::blank    [0, startPos - 1 - remainingLength)             startPos - 1 - remainingLength  0                                   lineLength - otherLengthSum - 1 - remainingLength
                // Solver::block    [startPos - 1 - remainingLength, startPos - 1)  remainingLength                 remainingLength                     remainingLength
                // Solver::blank    [startPos - 1, startPos)                        1                               1                                   1
                // otherResults     [startPos, lineLength)                          lineLength - startPos           lineLength - remainingLength - 1    otherLengthSum
                // sum                                                              lineLength                      lineLength                          lineLength
                for (int startPos = remainingLength + 1; startPos <= lineLength - otherLengthSum; startPos++)
                {
                    List<Cell> line = new List<Cell>();
                    line.AddRange(Enumerable.Repeat(Cell.BLANK, startPos - 1 - remainingLength));
                    line.AddRange(Enumerable.Repeat(Cell.BLOCK, remainingLength));
                    line.Add(Cell.BLANK);

                    List<List<Cell>> otherResults = GetPossibilities(otherCell, lineLength - startPos);
                    result.AddRange(otherResults.Select(otherResult => line.Concat(otherResult).ToList()));
                }
            }

            return result;
        }

        public struct SolveResult
        {
            public int ChangeCount => ChangePos.Count;
            public List<int> ChangePos { get; }

            public SolveResult(List<int> changePos)
            {
                ChangePos = changePos;
            }
        }
    }
}