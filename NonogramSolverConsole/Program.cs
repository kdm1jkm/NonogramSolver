using System;
using System.IO;

namespace NonogramSolverConsole
{
    internal static class Program
    {
        private static void Main(string[] args)
        {
            string file = args.Length > 0 ? args[0] : null;
            while (file == null || !File.Exists(file))
            {
                Console.Write("Enter file>>");
                file = Console.ReadLine();
            }

            int delay;
            string input = args.Length > 1 ? args[1] : null;
            while (input == null || !int.TryParse(input, out delay))
            {
                Console.Write("Enter delay>>");
                input = Console.ReadLine();
            }

            var app = new SolverApp(file, delay);
            app.Start();

            Console.Out.WriteLine("\nPress any key to continue...");
            Console.ReadKey(true);
        }
    }
}