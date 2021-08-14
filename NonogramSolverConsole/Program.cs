using System;
using System.IO;

namespace NonogramSolverConsole
{
    internal static class Program
    {
        private static void Main(string[] args)
        {
            string file;
            while (true)
            {
                Console.Write("Enter file>>");
                file = Console.ReadLine();

                if (file == null || !File.Exists(file)) continue;
                break;
            }

            int delay;
            while (true)
            {
                Console.Write("Enter delay>>");
                string input = Console.ReadLine();

                if (input == null || !int.TryParse(input, out delay)) continue;

                break;
            }

            var app = new ConsoleApp(file, delay);
            app.Start();

            Console.Out.WriteLine("\nPress any key to continue...");
            Console.ReadKey(true);
        }
    }
}