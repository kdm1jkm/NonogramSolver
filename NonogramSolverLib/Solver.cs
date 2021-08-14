﻿using System;
using System.Collections.Generic;
using System.Linq;

namespace NonogramSolverLib
{
    public class Solver
    {
        [Flags]
        public enum Cell
        {
            BLOCK = 0b10,
            BLANK = 0b01,
            CRASH = 0b11,
            NONE = 0b00
        }

        public Solver(int width, int height)
        {
            Board = new Board<Cell>(width, height, Cell.NONE);
        }

        public Board<Cell> Board { get; }

        public SolveResult SolveLine(int index, Board<Cell>.Direction direction, List<int> nums)
        {
            List<Cell> line = Board.GetLine(index, direction).ToList();
            List<List<Cell>> possibilities =
                GetPossibilities(nums, line.Count)
                    .FindAll(possibility => !MergeLine(line, possibility)
                        .Contains(Cell.CRASH));

            List<Cell> changes = possibilities.Aggregate(MergeLine)
                .Select(cell => cell == Cell.CRASH ? Cell.NONE : cell).ToList();
            List<Cell> appliedLine = MergeLine(line, changes);
            Board.SetLine(index, direction, appliedLine);
            return new SolveResult(line.Select((cell, i) => new { cell, i })
                .Where(x => x.cell != appliedLine[x.i])
                .Select(x => x.i).ToList());
        }

        public bool IsMapClear()
        {
            return !Board.Any(cell => cell is Cell.NONE or Cell.CRASH);
        }

        private static List<Cell> MergeLine(List<Cell> a, List<Cell> b)
        {
            if (a.Count != b.Count) throw new ArgumentException($"List size must be same, but {a.Count} != {b.Count}");

            return Enumerable.Range(0, a.Count).Select(i => a[i] | b[i]).ToList();
        }

        private static List<List<Cell>> GetPossibilities(List<int> cell, int lineLength)
        {
            List<List<Cell>> result = new List<List<Cell>>();

            switch (cell.Count)
            {
                case 0:
                    result.Add(Enumerable.Repeat(Cell.BLANK, lineLength).ToList());
                    break;
                case 1:
                {
                    int length = cell[0];

                    // 5칸에 4개짜리면 2개 넣을 수 있음 (5 - 4 + 1 = 2)
                    for (var startPos = 0; startPos < lineLength - length + 1; startPos++)
                    {
                        List<Cell> line = Enumerable.Repeat(Cell.BLANK, lineLength).ToList();

                        for (var i = 0; i < length; i++) line[startPos + i] = Cell.BLOCK;

                        result.Add(line);
                    }

                    break;
                }
                default:
                {
                    int remainingLength = cell[0];

                    // 블록길이 합 + 마지막거 빼고 사이사이 간격
                    List<int> otherCell = cell.GetRange(1, cell.Count - 1);
                    int otherLengthSum = otherCell.Sum() + (cell.Count - 2);

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

                    break;
                }
            }

            return result;
        }

        public readonly struct SolveResult
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