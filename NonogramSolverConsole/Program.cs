using System;

namespace NonogramSolverConsole
{
    internal static class Program
    {
        private static void Main(string[] args)
        {
            var app = new ConsoleApp(args);
            app.Start();

            Console.Out.WriteLine("\nPress any key to continue...");
            Console.ReadKey(true);
        }
    }
}