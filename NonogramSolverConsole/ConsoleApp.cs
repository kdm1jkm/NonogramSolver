using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading;
using NonogramSolverLib;

namespace NonogramSolverConsole
{
    public class ConsoleApp
    {
        private readonly Solver _solver;
        private readonly List<List<int>> _verticalInfo, _horizontalInfo;

        private readonly int _width, _height;

        public ConsoleApp(string[] args)
        {
            string[] contents;

            while (true)
            {
                Console.Write("Enter file>>");
                string input = Console.ReadLine();

                if (input == null || !File.Exists(input)) continue;

                contents = File.ReadAllLines(input);
                break;
            }

            List<int> metaInfo = contents[0].Split(" ").Select(int.Parse).ToList();
            _width = metaInfo[0];
            _height = metaInfo[1];

            if (contents.Length != _width + _height + 1) throw new Exception("File is not valid");

            List<List<int>> convertedContent = contents
                .Where((_, i) => i != 0)
                .Select(s => s.Split(" ").Select(int.Parse).ToList())
                .ToList();

            _horizontalInfo = convertedContent.GetRange(0, _height);
            _verticalInfo = convertedContent.GetRange(_height, _width);

            _solver = new Solver(_width, _height);
        }

        public void Start()
        {
            Console.CursorVisible = false;
            Console.Clear();
            const int interval = 0;

            int startX = (Console.WindowWidth - _width * 2) / 2;
            int startY = (Console.WindowHeight - _height) / 2;
            PrintSolver(startX, startY);

            Queue<(int i, Board<Solver.Cell>.Direction direction)> works =
                new Queue<(int i, Board<Solver.Cell>.Direction direction)>(Lines());

            while (true)
            {
                if (works.Count == 0)
                {
                    Console.Write("\nCan't Solve");
                    break;
                }

                (int i, var direction) = works.Dequeue();
                var result = _solver.SolveLine(i, direction, GetInfo(i, direction));

                if (result.ChangeCount == 0) continue;

                var otherDirection =
                    direction == Board<Solver.Cell>.Direction.VERTICAL
                        ? Board<Solver.Cell>.Direction.HORIZONTAL
                        : Board<Solver.Cell>.Direction.VERTICAL;

                foreach (int pos in result.ChangePos) works.Enqueue((pos, otherDirection));

                foreach ((int x, int y) in direction == Board<Solver.Cell>.Direction.HORIZONTAL
                    ? result.ChangePos.Select(pos => (pos, i))
                    : result.ChangePos.Select(pos => (i, pos)))
                {
                    Console.SetCursorPosition(startX + x * 2, startY + y);
                    PrintCell(_solver.Board[x, y]);
                }

                if (_solver.IsMapClear()) break;

                Thread.Sleep(interval);
            }

            PrintSolver(startX, startY);
        }

        private IEnumerable<(int, Board<Solver.Cell>.Direction)> Lines()
        {
            for (var i = 0; i < _height; i++) yield return (i, Board<Solver.Cell>.Direction.HORIZONTAL);

            for (var i = 0; i < _width; i++) yield return (i, Board<Solver.Cell>.Direction.VERTICAL);
        }

        private List<int> GetInfo(int i, Board<Solver.Cell>.Direction direction)
        {
            return direction == Board<Solver.Cell>.Direction.HORIZONTAL ? _horizontalInfo[i] : _verticalInfo[i];
        }

        private void PrintSolver(int x, int y)
        {
            int height = _solver.Board.Height;
            for (var i = 0; i < height; i++)
            {
                Console.SetCursorPosition(x, y + i);
                List<Solver.Cell> line = _solver.Board.GetLine(i, Board<Solver.Cell>.Direction.HORIZONTAL).ToList();
                foreach (var cell in line) PrintCell(cell);
            }
        }

        private static void PrintCell(Solver.Cell cell)
        {
            switch (cell)
            {
                case Solver.Cell.BLOCK:
                    Console.BackgroundColor = ConsoleColor.White;
                    Console.ForegroundColor = ConsoleColor.White;
                    break;

                case Solver.Cell.BLANK:
                    Console.ResetColor();
                    break;

                case Solver.Cell.NONE:
                    Console.BackgroundColor = ConsoleColor.DarkGray;
                    Console.ForegroundColor = ConsoleColor.DarkGray;
                    break;

                case Solver.Cell.CRASH:
                    Console.BackgroundColor = ConsoleColor.Red;
                    Console.ForegroundColor = ConsoleColor.Red;
                    break;
            }

            Console.Write("  ");
            Console.ResetColor();
        }
    }
}