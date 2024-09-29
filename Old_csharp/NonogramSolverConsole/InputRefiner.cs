using System;

namespace NonogramSolverConsole
{
    public class InputRefiner<T>
    {
        private readonly Action _beforeGet;
        private readonly Func<string> _getter;
        private readonly Func<string, (T, bool)> _parser;

        public InputRefiner(Func<string> getter, Func<string, (T, bool)> parser, Action beforeGet)
        {
            _getter = getter;
            _parser = parser;
            _beforeGet = beforeGet;
        }

        public T GetValue()
        {
            while (true)
            {
                _beforeGet.Invoke();
                (var value, bool isSuccess) = _parser.Invoke(_getter.Invoke());
                if (isSuccess) return value;
            }
        }
    }
}