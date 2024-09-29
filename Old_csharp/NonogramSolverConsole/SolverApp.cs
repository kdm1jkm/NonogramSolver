using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading;
using NonogramSolverLib;
using static NonogramSolverLib.Solver;

namespace NonogramSolverConsole
{
    public class SolverApp
    {
        private readonly int _delay;
        private readonly int _length;
        private readonly Solver _solver;

        private readonly int _width, _height;

        public SolverApp(string[] args)
        {
            string file = args.Length > 0
                ? args[0]
                : new InputRefiner<string>(
                    Console.ReadLine,
                    s => (s, s != null && File.Exists(s)),
                    () => Console.Write("Enter file>>")
                ).GetValue();

            int delay = args.Length > 1
                ? int.Parse(args[1])
                : new InputRefiner<int>(
                    Console.ReadLine,
                    s =>
                    {
                        bool isSuccess = int.TryParse(s, out int delay);
                        return (delay, isSuccess);
                    },
                    () => Console.Write("Enter delay>>")
                ).GetValue();

            string[] contents = File.ReadAllLines(file);

            _delay = delay;

            List<int> metaInfo = contents[0].Split(" ").Select(int.Parse).ToList();
            _width = metaInfo[0];
            _height = metaInfo[1];

            if (contents.Length != _width + _height + 1) throw new Exception("File is not valid");

            List<List<int>> convertedContent = contents
                .Where((_, i) => i != 0)
                .Select(s => s.Split(" ").Select(int.Parse).ToList())
                .ToList();

            List<List<int>> horizontalInfo = convertedContent.GetRange(0, _height);
            List<List<int>> verticalInfo = convertedContent.GetRange(_height, _width);

            _solver = new Solver(_width, _height, verticalInfo, horizontalInfo);
            _length = _width * _height;
        }

        public void Start()
        {
            Stack<LineInfo> works = new Stack<LineInfo>(Lines()
                .Select(line => (line, _solver.GetNumberOfCases(line, true)))
                .OrderByDescending(x => x.Item2)
                .Select(x => x.line));

            Console.CursorVisible = false;
            Console.Clear();

            int x = (Console.WindowWidth - _width * 2) / 2;
            int y = (Console.WindowHeight - _height) / 2;

            bool isDrawable = !(x < 0 || y < 1);

            if (isDrawable)
                PrintSolver(x, y);
            else
                Console.Out.WriteLine("Can't draw.");

            while (true)
            {
                if (works.Count == 0)
                {
                    Console.Write("\nCan't Solve");
                    break;
                }

                (int i, var direction) = works.Pop();

                Console.SetCursorPosition(0, isDrawable ? 0 : 1);

                int countDetermined = _solver.CountDetermined();
                Console.Write(
                    "\r" +
                    $"Cached: {_solver.GetCachedLength()}  " +
                    $"Memory: {Process.GetCurrentProcess().WorkingSet64 / 1024 / 1024}mb  " +
                    $"{countDetermined}/{_length}  " +
                    $"{countDetermined * 100 / _length}%  " +
                    $"{i}/{direction.ToString()}"
                );
                while (Console.CursorLeft < Console.BufferWidth - 1) Console.Write(" ");

                if (isDrawable) Thread.Sleep(_delay);

                var result = _solver.SolveLine(new LineInfo(i, direction));

                if (result.ChangeCount == 0) continue;

                var otherDirection =
                    direction == Direction.VERTICAL
                        ? Direction.HORIZONTAL
                        : Direction.VERTICAL;

                IEnumerable<LineInfo> nextLines = result.ChangePos
                    .Select(pos => new LineInfo(pos, otherDirection))
                    .Select(pos => (pos, _solver.GetNumberOfCases(pos)))
                    .OrderByDescending(value => value.Item2)
                    .Select(value => value.pos);

                foreach (var nextLine in nextLines) works.Push(nextLine);

                if (_solver.IsMapClear()) break;

                if (!isDrawable) continue;

                IEnumerable<(int x, int y)> changedPoses = direction == Direction.VERTICAL
                    ? result.ChangePos.Select(pos => (i, pos))
                    : result.ChangePos.Select(pos => (pos, i));

                foreach (var changedPos in changedPoses)
                {
                    Console.SetCursorPosition(x + changedPos.x * 2, y + changedPos.y);
                    PrintCell(_solver.Board[changedPos.x, changedPos.y]);
                }
            }

            if (isDrawable) PrintSolver(x, y);
        }

        private IEnumerable<LineInfo> Lines()
        {
            for (var i = 0; i < _height; i++) yield return new LineInfo(i, Direction.HORIZONTAL);

            for (var i = 0; i < _width; i++) yield return new LineInfo(i, Direction.VERTICAL);
        }

        private void PrintSolver(int x, int y)
        {
            for (var i = 0; i < _solver.Board.Height; i++)
            {
                Console.SetCursorPosition(x, y + i);
                List<Cell> line = _solver.Board.GetLine(new LineInfo(i, Direction.HORIZONTAL)).ToList();
                foreach (var cell in line) PrintCell(cell);
            }
        }

        private static void PrintCell(Cell cell)
        {
            switch (cell)
            {
                case Cell.BLOCK:
                    Console.BackgroundColor = ConsoleColor.White;
                    Console.ForegroundColor = ConsoleColor.White;
                    break;

                case Cell.BLANK:
                    Console.ResetColor();
                    break;

                case Cell.NONE:
                    Console.BackgroundColor = ConsoleColor.DarkGray;
                    Console.ForegroundColor = ConsoleColor.DarkGray;
                    break;

                case Cell.CRASH:
                    Console.BackgroundColor = ConsoleColor.Red;
                    Console.ForegroundColor = ConsoleColor.Red;
                    break;
            }

            Console.Write("  ");
            Console.ResetColor();
        }
    }
}