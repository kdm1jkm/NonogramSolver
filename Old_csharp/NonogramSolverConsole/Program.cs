using System;
using System.Diagnostics;

namespace NonogramSolverConsole
{
    internal static class Program
    {
        private static void Main(string[] args)
        {
            var stopwatch = new Stopwatch();
            stopwatch.Start();
            var app = new SolverApp(args);
            app.Start();
            stopwatch.Stop();

            Console.Write($" {stopwatch.Elapsed}");
            Console.ReadKey(true);
            Console.Out.WriteLine();
        }
    }
}