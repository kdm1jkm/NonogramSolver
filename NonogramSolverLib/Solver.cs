using System;
using System.Collections.Generic;
using System.Linq;

namespace NonogramSolverLib
{
    public class Solver
    {
        [Flags]
        public enum Cell : byte
        {
            BLOCK = 0b10,
            BLANK = 0b01,
            CRASH = 0b11,
            NONE = 0b00
        }

        /// <summary>
        ///     가능성 캐시 저장소이다. 해당 라인에서 나올 수 있는 가능성을 모두 저장하고 있다. 그러나 <see cref="Board" /> 클래스에서
        ///     저장하고 있는 해당 라인과 병합했을 때 <see cref="Cell.CRASH" />가 나타나는 라인은 제거된다.
        /// </summary>
        private readonly Dictionary<(int, Direction), List<List<Cell>>> _calculatedPossibilities;

        /// <summary>
        ///     수평 방향 라인의 정보를 담고 있다.
        /// </summary>
        private readonly List<List<int>> _horizontalInfos;

        /// <summary>
        ///     수직 방향 라인의 정보를 담고 있다.
        /// </summary>
        private readonly List<List<int>> _verticalInfos;

        /// <summary>
        ///     맵 전체를 나타낸다
        /// </summary>
        public readonly Board<Cell> Board;

        /// <summary>
        ///     네모로직을 푸는 클래스이다.
        /// </summary>
        /// <param name="width">네모로직의 너비</param>
        /// <param name="height">네모로직의 높이</param>
        /// <param name="verticalInfos">수직 방향 라인의 정보</param>
        /// <param name="horizontalInfos">수평 방향 라인의 정보</param>
        public Solver(int width, int height, List<List<int>> verticalInfos, List<List<int>> horizontalInfos)
        {
            _verticalInfos = verticalInfos;
            _horizontalInfos = horizontalInfos;

            Board = new Board<Cell>(width, height, Cell.NONE);

            _calculatedPossibilities = new Dictionary<(int, Direction), List<List<Cell>>>();
        }


        /// <summary>
        ///     한 라인에서 알아낼 수 있는 정보를 모두 알아내 <see cref="Board" />에 적용한다.
        /// </summary>
        /// <param name="index">줄의 인덱스</param>
        /// <param name="direction">줄의 방향</param>
        /// <returns>알아낸 정보 요약</returns>
        public SolveResult SolveLine(int index, Direction direction)
        {
            List<int> info = GetInfo(index, direction);
            List<Cell> line = Board.GetLine(index, direction).ToList();

            // 계산되지 않은 경우 계산
            if (!_calculatedPossibilities.ContainsKey((index, direction)))
                _calculatedPossibilities[(index, direction)] = GetPossibilities(info, line.Count).ToList();

            // 현재 경우에 가능한 경우만 거름(Merge해서 CRASH가 생기지 않는 경우)
            IEnumerable<List<Cell>> possibilities
                = _calculatedPossibilities[(index, direction)].Where(possibility =>
                    !MergeLine(line, possibility).Contains(Cell.CRASH)).ToList();

            // 걸러진 경우는 다음에도 무조건 걸러지므로(Board에서 None->BLOCK/BLANK->CRASH순서로만 변함) 걸러진 목록으로 업데이트
            _calculatedPossibilities[(index, direction)] = possibilities.ToList();

            // 현재 가능한 경우를 모두 병합함(Crash는 변화시키지 않기 위해 None으로 변경) 
            List<Cell> mergedPossibility
                = possibilities.Aggregate(MergeLine).Select(cell => cell == Cell.CRASH ? Cell.NONE : cell).ToList();

            // 변경내역 적용
            List<Cell> mergedLine = MergeLine(line, mergedPossibility);

            // 적용된 라인으로 세팅
            Board.SetLine(index, direction, mergedLine);

            // 메모리 확보
            GC.Collect();

            // 다른 부분의 인덱스
            return new SolveResult(line
                .Select((cell, i) => (cell, i))
                .Where(x => x.cell != mergedLine[x.i])
                .Select(x => x.i).ToList());
        }

        public int GetCachedLength()
        {
            return _calculatedPossibilities.Values
                .Sum(calculatedPossibility => calculatedPossibility.Count * calculatedPossibility[0].Count);
        }

        /// <summary>
        ///     맵이 클리어 상태인지 확인한다. 맵에 <see cref="Cell.NONE" /> 또는 <see cref="Cell.CRASH" />가 없으면 클리어로 판단한다.
        /// </summary>
        /// <returns>클리어 여부</returns>
        public bool IsMapClear()
        {
            return !Board.Any(cell => cell is Cell.NONE or Cell.CRASH);
        }

        public int CountDetermined()
        {
            var sum = 0;
            for (var i = 0; i < Board.Height; i++)
                sum += Board
                    .GetLine(i, Direction.HORIZONTAL)
                    .Count(cell => cell is Cell.BLANK or Cell.BLOCK);

            return sum;
        }

        public List<int> GetInfo(int index, Direction direction)
        {
            return direction == Direction.VERTICAL
                ? _verticalInfos[index]
                : _horizontalInfos[index];
        }

        private static List<Cell> MergeLine(List<Cell> a, List<Cell> b)
        {
            if (a.Count != b.Count) throw new ArgumentException($"List size must be same, but {a.Count} != {b.Count}");

            return Enumerable.Range(0, a.Count).Select(i => a[i] | b[i]).ToList();
        }

        public static uint GetNumberOfCases(List<int> info, int lineLength)
        {
            switch (info.Count)
            {
                case 0:
                    return (uint)lineLength;

                case 1:
                    return (uint)(lineLength - info[0] + 1);

                default:
                {
                    int remainingLength = info[0];

                    // 블록길이 합 + 마지막거 빼고 사이사이 간격
                    List<int> otherCell = info.GetRange(1, info.Count - 1);
                    int otherLengthSum = otherCell.Sum() + (info.Count - 2);

                    uint sum = 0;
                    for (int startPos = remainingLength + 1; startPos <= lineLength - otherLengthSum; startPos++)
                    {
                        sum += GetNumberOfCases(otherCell, lineLength - startPos);
                    }

                    return sum;
                }
            }
        }

        /// <summary>
        ///     <see cref="info" />정보를 가지고 길이가 <see cref="lineLength" />인 라인에서 나올 수 있는 모든 가짓수를 계산한다.
        /// </summary>
        /// <param name="info">라인의 정보</param>
        /// <param name="lineLength">라인의 길이</param>
        /// <returns>모든 정보</returns>
        private static IEnumerable<List<Cell>> GetPossibilities(List<int> info, int lineLength)
        {
            switch (info.Count)
            {
                case 0:
                    yield return Enumerable.Repeat(Cell.BLANK, lineLength).ToList();
                    yield break;

                case 1:
                {
                    int length = info[0];

                    // 5칸에 4개짜리면 2개 넣을 수 있음 (5 - 4 + 1 = 2)
                    for (var startPos = 0; startPos < lineLength - length + 1; startPos++)
                    {
                        List<Cell> line = Enumerable.Repeat(Cell.BLANK, lineLength).ToList();

                        for (var i = 0; i < length; i++) line[startPos + i] = Cell.BLOCK;

                        yield return line;
                    }

                    yield break;
                }
                default:
                {
                    int remainingLength = info[0];

                    // 블록길이 합 + 마지막거 빼고 사이사이 간격
                    List<int> otherCell = info.GetRange(1, info.Count - 1);
                    int otherLengthSum = otherCell.Sum() + (info.Count - 2);

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

                        IEnumerable<IEnumerable<Cell>> combined = GetPossibilities(otherCell, lineLength - startPos)
                            .Select(otherResult => line.Concat(otherResult));

                        foreach (IEnumerable<Cell> cells in combined) yield return cells.ToList();
                    }

                    yield break;
                }
            }
        }

        public record SolveResult
        {
            public SolveResult(List<int> changePos)
            {
                ChangePos = changePos;
            }

            public int ChangeCount => ChangePos.Count;
            public List<int> ChangePos { get; }
        }
    }
}