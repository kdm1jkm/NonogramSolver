using System;

namespace NonogramSolverConsole
{
    internal static class Program
    {
        private static void Main(string[] args)
        {
            var app = new SolverApp(args);
            app.Start();

            Console.Write("End...");
            Console.ReadKey(true);
            Console.Out.WriteLine();
        }
    }
}