using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Net.Sockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

class Char
{
    internal static void Server()
    {
        Console.WriteLine("Waiting for connection");
        TcpListener tcpListener = TcpListener.Create(8080);
        tcpListener.Start();

        while (true)
        {
            TcpClient tcpClient = tcpListener.AcceptTcpClient();
            tcpClient.NoDelay = true;
            Thread thread = new(() =>
            {
                var x = 0.0;
                var count = 0;
                using NetworkStream networkStream = tcpClient.GetStream();
                using var _ = tcpClient;
                // var charIndexes = new int[char.MaxValue];
                // for (int i = 0; i < charIndexes.Length; i++)
                // {
                //     charIndexes[i] = -1;
                // }
                var buf = new byte[10240];
                var reader = new StreamReader(networkStream);
                Stopwatch stopwatch = Stopwatch.StartNew();
                while (true)
                {
                    // Console.WriteLine('1');
                    int v = 0;
                    try
                    {
                        v = networkStream.Read(buf);
                    }
                    catch { v = 0; }
                    // Console.WriteLine('2');

                    if (v == 0)
                    {
                        Console.WriteLine($"Disconnected , toke {x * 1_000_000_000.0 / count} ms");
                        break;
                    }
                    var str = Encoding.UTF8.GetString(buf, 0, v);
                    // Console.WriteLine(str);
                    // Console.WriteLine("Recieved " + v);
                    stopwatch.Reset();
                    stopwatch.Start();
                    string result = Find(str) + "\n";
                    x += stopwatch.Elapsed.TotalSeconds;
                    count += 1;
                    v = Encoding.UTF8.GetBytes(result, buf);
                    networkStream.Write(buf, 0, v);
                    networkStream.Flush();
                }
            });
            thread.Start();

        }



    }
    internal static Task ServerAsync()
    {
        Console.WriteLine("Waiting for connection");
        TcpListener tcpListener = TcpListener.Create(8080);
        tcpListener.Start();

        while (true)
        {
            TcpClient tcpClient = tcpListener.AcceptTcpClient();
            Console.WriteLine("recived connection ");
            tcpClient.NoDelay = true;

            Task.Run(async () =>
            {

                // });
                // Thread thread = new(async () =>
                // {
                var x = 0.0;
                var count = 0;
                using NetworkStream networkStream = tcpClient.GetStream();
                using var _ = tcpClient;
                // var charIndexes = new int[char.MaxValue];
                var buf = new byte[10240];
                Stopwatch stopwatch = Stopwatch.StartNew();
                var reader = new StreamReader(networkStream);
                while (true)
                {
                    // Console.WriteLine('1');
                    // int v = 0;
                    string str;
                    try
                    {
                        str = await reader.ReadLineAsync();
                    }
                    catch
                    {
                        // v = 0;
                        str = null;
                    }
                    // Console.WriteLine('2');

                    if (string.IsNullOrEmpty(str))
                    {
                        Console.WriteLine($"Disconnected , toke {x * 1_000_000_000.0 / count} ms");
                        break;
                    }
                    // var str = Encoding.UTF8.GetString(buf, 0, v);
                    // Console.WriteLine(str);
                    // Console.WriteLine("Recieved " + v);
                    stopwatch.Reset();
                    stopwatch.Start();
                    string result = Find(str) + "\n";
                    x += stopwatch.Elapsed.TotalSeconds;
                    count += 1;
                    var v = Encoding.UTF8.GetBytes(result, buf);
                    await networkStream.WriteAsync(buf.AsMemory(0, v));
                    // await networkStream.FlushAsync();
                }
            });

        }



    }

    internal static string Find(string str)
    {
        var len = str.Length;
        var charIndexes = new Dictionary<char, int>();

        var matchIndex = 0;
        var matchLength = 0;
        var index = 0;
        for (int charIndex = 0; charIndex < len; charIndex++)
        {
            var chr = str[charIndex];
            bool charExist = charIndexes.TryGetValue(chr, out var g) && g >= index && g < charIndex;

            var isTheEnd = charIndex == len - 1;

            if (charExist || isTheEnd)
            {
                var length = charIndex - index;


                if (!charExist) length += 1;

                if (length > matchLength)
                {
                    matchIndex = index;
                    matchLength = length;
                }
                if (charExist)
                {
                    index = charIndexes[chr] + 1;
                }
            }
            charIndexes[chr] = charIndex;

        }
        var result = (matchIndex, matchLength) switch
        {
            (0, 0) => null,
            _ => str.Substring(matchIndex, matchLength),
        };
        return result;
    }

}