using System;
using System.Collections.Generic;
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
        }

        public void Start()
        {
            Console.CursorVisible = false;
            Console.Clear();

            int x = (Console.WindowWidth - _width * 2) / 2;
            int y = (Console.WindowHeight - _height) / 2;

            var isDrawable = true;

            if (x < 0 || y < 0)
            {
                Console.Out.WriteLine("Can't draw.");
                // TEMP
                // return;
                isDrawable = false;
            }

            if (isDrawable) PrintSolver(x, y);

            Queue<(int i, Direction direction)> works =
                new Queue<(int i, Direction direction)>(Lines());

            ulong count = 0;

            while (true)
            {
                Console.SetCursorPosition(0, isDrawable ? 0 : Console.GetCursorPosition().Top);
                Console.Write($"iter:{count++}");

                if (works.Count == 0)
                {
                    Console.Write("\nCan't Solve");
                    break;
                }

                (int i, var direction) = works.Dequeue();
                var result = _solver.SolveLine(i, direction);

                if (result.ChangeCount == 0) continue;

                var otherDirection =
                    direction == Direction.VERTICAL
                        ? Direction.HORIZONTAL
                        : Direction.VERTICAL;

                foreach (int pos in result.ChangePos) works.Enqueue((pos, otherDirection));

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

                Thread.Sleep(_delay);
            }

            PrintSolver(x, y);
        }

        private IEnumerable<(int, Direction)> Lines()
        {
            for (var i = 0; i < _height; i++) yield return (i, Direction.HORIZONTAL);

            for (var i = 0; i < _width; i++) yield return (i, Direction.VERTICAL);
        }

        private void PrintSolver(int x, int y)
        {
            for (var i = 0; i < _solver.Board.Height; i++)
            {
                Console.SetCursorPosition(x, y + i);
                List<Cell> line = _solver.Board.GetLine(i, Direction.HORIZONTAL).ToList();
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