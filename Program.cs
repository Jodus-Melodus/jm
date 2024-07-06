using System.Diagnostics;

class Program
{
    static void Main()
    {
        string domainName = Environment.UserDomainName;
        string userName = Environment.UserName;

        while (true)
        {
            string cwd = Directory.GetCurrentDirectory();
            DateTime now = DateTime.Now;
            Console.ForegroundColor = ConsoleColor.Blue;
            string[] command = ReadLine($"{userName} @ {domainName} ({now:yyyy-MM-dd HH:mm:ss})\n> ").Split(' ');
            Console.ForegroundColor = ConsoleColor.White;

            if (command.Length == 0)
                continue;

            string cmd = command[0].ToLower();
            string[] args = command.Skip(1).ToArray();

            switch (cmd)
            {
                case "backup":
                    HandleBackup(args, cwd);
                    break;
                case "bin2dec":
                    HandleBaseConversion(args, 2);
                    break;
                case "oct2dec":
                    HandleBaseConversion(args, 8);
                    break;
                case "hex2dec":
                    HandleBaseConversion(args, 16);
                    break;
                case "dec2bin":
                    HandleDecimalConversion(args, 2);
                    break;
                case "dec2oct":
                    HandleDecimalConversion(args, 8);
                    break;
                case "dec2hex":
                    HandleDecimalConversion(args, 16);
                    break;
                case "rm":
                case "del":
                    HandleFileAndDirectoryRemoval(args, cwd);
                    break;
                case "ls":
                case "dir":
                    HandleListDirectory(cwd);
                    break;
                case "cwd":
                    Console.WriteLine(cwd);
                    break;
                case "cd":
                    HandleChangeDirectory(args, cwd);
                    break;
                case "find":
                    HandleFind(args, cwd);
                    break;
                case "findlike":
                    HandleFindLike(args, cwd);
                    break;
                case "tree":
                    HandleTree(args, cwd);
                    break;
                case "cls":
                case "clear":
                    Console.Clear();
                    break;
                case "exit":
                case "kill":
                case "quit":
                    Environment.Exit(0);
                    break;
                case "!":
                    HandleExternalCommand(args);
                    break;
                case "read":
                    HandleReadFile(args, cwd);
                    break;
                case "empty":
                    HandleEmpty(args, cwd);
                    break;
                case "help":
                    PrintHelp();
                    break;
                case "":
                    break;
                default:
                    PrintColored("Unknown internal or external command", "red");
                    break;
            }
        }
    }

    private static void HandleEmpty(string[] args, string cwd)
    {
        if (args.Length > 0)
        {
            string source = Path.Join(cwd, args[0]);
            CopyDirectoryRecursively(source, cwd, true);

            PrintColored("Finished emptying directory", "green");
        }
        else
        {
            PrintColored("Expected a directory path", "red");
        }
    }

    static void PrintHelp()
    {
        Console.WriteLine(@"
backup {source} {destination}           - copies the source dir to the destinatino dir
bin2dec {bin}                           - converts bin number to dec
oct2dec {oct}                           - converts oct number to dec
hex2dec {hex}                           - converts hex number to dec
dec2bin {dec}                           - converts dec number to bin
dec2oct {dec}                           - converts dec number to oct
dec2hex {dec}                           - converts dec number to hex
rm {file/dir name}                      - deletes the specified dir/file
del {file/dir name}                     - deletes the specified dir/file
ls                                      - displays the dirs/files in the current dir
dir                                     - displays the dirs/files in the current dir
cwd                                     - displays the current working directory
cd {new dir}                            - change the current directory to the new specified one
find {file name}                        - searches the subdirectories for a file with the specified name
findlike {match}                        - seraches the subdirectories for a file that matches the specified match
tree {depth}                            - displays the current directory and subdirectories as a tree with a specified depth
cls                                     - clears the screen
clear                                   - clears the screen
exit                                    - terminates the current instance of the terminal
kill                                    - terminates the current instance of the terminal
quit                                    - terminates the current instance of the terminal
! {command}                             - run an external command
read {file name}                        - display the contents of a file
empty {dir name}                        - takes the specified dirs contents and moves them to the directory's parent
help                                    - display this menu
    ");
    }

    private static void HandleFileAndDirectoryRemoval(string[] args, string cwd)
    {
        string path = Path.Join(cwd, string.Join(" ", args));

        try
        {
            if (Directory.Exists(path))
            {
                Directory.Delete(path, true);
                PrintColored($"Removed '{Path.GetFileName(path)}' successfully", "green");
            }
            else if (File.Exists(path))
            {
                File.Delete(path);
                PrintColored($"Removed '{Path.GetFileName(path)}' successfully", "green");
            }
            else
            {
                PrintColored($"'{Path.GetFileName(path)}' does not exist", "red");
            }
        }
        catch (IOException e)
        {
            PrintColored($"An error occurred while trying to delete '{Path.GetFileName(path)}'", "red");
            PrintColored(e.Message, "red");
        }
    }

    private static string[]? ReadFile(string path)
    {
        try
        {
            string[] lines = File.ReadAllLines(path);
            return lines;
        }
        catch (IOException e)
        {
            PrintColored($"An error occurred while reading '{Path.GetFileName(path)}'", "red");
            PrintColored(e.Message, "red");
            return null;
        }
    }

    private static void HandleReadFile(string[] args, string cwd)
    {
        string path = Path.Join(cwd, string.Join(" ", args));
        string[]? lines = ReadFile(path);

        if (lines is not null)
        {
            Console.WriteLine(string.Join("\n", lines));
        }
    }

    private static void HandleExternalCommand(string[] args)
    {
        if (args.Length > 0)
        {
            Process process = new();
            process.StartInfo.FileName = "cmd.exe";
            process.StartInfo.Arguments = $"/C {string.Join(' ', args)}";
            process.StartInfo.RedirectStandardOutput = true;
            process.StartInfo.RedirectStandardError = true;
            process.StartInfo.UseShellExecute = false;
            process.StartInfo.CreateNoWindow = true;

            process.Start();

            string output = process.StandardOutput.ReadToEnd();
            string error = process.StandardError.ReadToEnd();

            process.WaitForExit();

            if (!string.IsNullOrEmpty(output))
            {
                Console.WriteLine(output);
            }

            if (!string.IsNullOrEmpty(error))
            {
                Console.WriteLine(error);
            }
        }
    }

    static void HandleBackup(string[] args, string cwd)
    {
        if (args.Length > 0)
        {
            Backup(args[0], args.Length > 1 ? args[1] : cwd);
        }
        else
        {
            PrintColored("Expected source path", "red");
        }
    }

    static void HandleBaseConversion(string[] args, int fromBase)
    {
        if (args.Length > 0)
        {
            try
            {
                Console.WriteLine(Convert.ToInt32(args[0], fromBase));
            }
            catch (Exception ex)
            {
                PrintColored($"Conversion error: {ex.Message}", "red");
            }
        }
        else
        {
            PrintColored($"Expected a base-{fromBase} number", "red");
        }
    }

    static void HandleDecimalConversion(string[] args, int toBase)
    {
        if (args.Length > 0)
        {
            try
            {
                Console.WriteLine(Convert.ToString(int.Parse(args[0]), toBase));
            }
            catch (Exception ex)
            {
                PrintColored($"Conversion error: {ex.Message}", "red");
            }
        }
        else
        {
            PrintColored("Expected a decimal number", "red");
        }
    }

    static void HandleListDirectory(string cwd)
    {
        string[] directories = [.. Directory.GetDirectories(cwd)];
        string[] files = [.. Directory.GetFiles(cwd)];

        PrintColored("Directories:", "magenta");
        PrintColored(string.Format("{0,-20} {1,-25} {2,-25} {3,-25} {4,-25}", "Name", "", "Modified", "Accessed", "Created"), "green");
        foreach (string directory in directories)
        {
            string name = Path.GetFileName(directory) + '/';
            DirectoryInfo dirInfo = new(name);
            DateTime created = dirInfo.CreationTime;
            DateTime lastWrite = dirInfo.LastWriteTime;
            DateTime lastAccessed = dirInfo.LastAccessTime;

            PrintColored(string.Format("{0,-20} {1,-25} {2,-25} {3,-25} {4,-25}", name, null, lastWrite, lastAccessed, created), "yellow");
        }

        PrintColored("-----------------------------------------------------------------------------------------------------------------------------", "blue");

        PrintColored("Files:", "magenta");
        PrintColored(string.Format("{0,-20} {1,-25} {2,-25} {3,-25} {4,-25}", "Name", "Size", "Modified", "Accessed", "Created"), "green");
        foreach (string file in files)
        {
            string name = Path.GetFileName(file);
            FileInfo fileInfo = new(file);
            long size = fileInfo.Length;
            DateTime created = fileInfo.CreationTime;
            DateTime lastWrite = fileInfo.LastWriteTime;
            DateTime lastAccessed = fileInfo.LastAccessTime;

            PrintColored(string.Format("{0,-20} {1,-25} {2,-25} {3,-25} {4,-25}", name, size.ToString() + 'B', lastWrite, lastAccessed, created), "yellow");
        }
    }

    static void HandleChangeDirectory(string[] args, string cwd)
    {
        if (args.Length > 0)
        {
            string newDir = args[0] == ".." ? ".." : Path.Combine(cwd, string.Join(" ", args));
            if (Directory.Exists(newDir))
            {
                Directory.SetCurrentDirectory(newDir);
            }
            else
            {
                PrintColored("No such directory", "red");
            }

        }
        else
        {
            PrintColored("Expected directory path", "red");
        }

    }

    static void HandleFind(string[] args, string cwd)
    {
        if (args.Length > 0)
        {
            bool findRelativePath = args.Length > 1 && args[1] == "-r";
            string resultPath = FindFile(cwd, args[0]);

            if (string.IsNullOrEmpty(resultPath))
            {
                PrintColored("File not found", "red");
            }
            else
            {
                Console.WriteLine(findRelativePath ? GetRelativePath(cwd, resultPath) : resultPath);
            }
        }
        else
        {
            PrintColored("Expected filename", "red");
        }
    }

    static void HandleFindLike(string[] args, string cwd)
    {
        if (args.Length > 0)
        {
            bool findRelativePath = args.Length > 1 && args[1] == "-r";
            string[] matches = FindLikeFile(cwd, args[0]);
            PrintColored($"Found {matches.Length} matches.", "magenta");

            foreach (string match in matches)
            {
                Console.WriteLine(findRelativePath ? GetRelativePath(cwd, match) : match);
            }
        }
        else
        {
            PrintColored("Expected filename", "red");
        }
    }

    static void HandleTree(string[] args, string cwd)
    {
        int depth = args.Length > 0 ? int.Parse(args[0]) : 2;
        Console.WriteLine(TreeView(cwd, depth, 0, "", "  "));
    }

    private static void Backup(string sourcePath, string destinationPath)
    {
        if (File.Exists(sourcePath))
        {
            try
            {
                string destFilePath = Path.Combine(destinationPath, Path.GetFileName(sourcePath));
                File.Copy(sourcePath, destFilePath, true);
                Console.WriteLine($"File '{sourcePath}' backed up to '{destFilePath}'");
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error copying file '{sourcePath}': {ex.Message}");
            }
        }
        else if (Directory.Exists(sourcePath))
        {
            try
            {
                string destDirPath = Path.Combine(destinationPath, Path.GetFileName(sourcePath));
                CopyDirectoryRecursively(sourcePath, destDirPath);
                Console.WriteLine($"Directory '{sourcePath}' backed up to '{destDirPath}'");
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error copying directory '{sourcePath}': {ex.Message}");
            }
        }
        else
        {
            Console.WriteLine($"The path '{sourcePath}' doesn't exist");
        }
    }

    private static void CopyDirectoryRecursively(string sourceDirectory, string destinationDirectory, bool deleteAfterCopy = false)
    {
        Directory.CreateDirectory(destinationDirectory);

        foreach (string file in Directory.GetFiles(sourceDirectory))
        {
            string destFilePath = Path.Combine(destinationDirectory, Path.GetFileName(file));
            try
            {
                File.Copy(file, destFilePath, true);
                if (deleteAfterCopy)
                {
                    File.Delete(file);
                }
                Console.WriteLine($"Copied file '{file}' to '{destFilePath}'");
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error copying file '{file}': {ex.Message}");
            }
        }

        foreach (string directory in Directory.GetDirectories(sourceDirectory))
        {
            string destSubDir = Path.Combine(destinationDirectory, Path.GetFileName(directory));
            CopyDirectoryRecursively(directory, destSubDir);
        }
    }


    private static string TreeView(string CWD, int Depth = 2, int Indentation = 0, string Tree = "", string IndentationString = "\t")
    {
        string[] SubDirectories = Directory.GetDirectories(CWD);
        string[] Files = Directory.GetFiles(CWD);

        foreach (string SubDirectory in SubDirectories)
            if (Depth > 0)
            {
                Tree += string.Concat(Enumerable.Repeat(IndentationString, Indentation)) + (Indentation == 0 ? "" : '└') + Path.GetFileName(SubDirectory) + '\n';
                Tree = TreeView(SubDirectory, Depth - 1, Indentation + 1, Tree);
            }

        foreach (string File in Files)
            Tree += string.Concat(Enumerable.Repeat(IndentationString, Indentation)) + (Indentation == 0 ? "" : '└') + Path.GetFileName(File) + '\n';

        return Tree;
    }

    private static string[] FindLikeFile(string CWD, string FileName, string[]? Found = null)
    {
        Found ??= [];
        string[] Files = Directory.GetFiles(CWD);
        string[] SubDirectories = Directory.GetDirectories(CWD);

        var MatchingFiles = Files.Where(File => File.Contains(FileName)).ToArray();
        Found = [.. Found, .. MatchingFiles];

        foreach (string SubDirectory in SubDirectories)
            Found = FindLikeFile(SubDirectory, FileName, Found);

        return Found;
    }

    static string FindFile(string CWD, string FileName)
    {
        string[] SubDirectories = Directory.GetDirectories(CWD);
        string[] Files = Directory.GetFiles(CWD);

        foreach (string File in Files)
        {
            if (Path.GetFileName(File) == FileName)
                return File;
        }

        foreach (string SubDirectory in SubDirectories)
        {
            string result = FindFile(SubDirectory, FileName);
            if (result is not "")
                return result;
        }

        return "";
    }

    static void PrintColored(string data, string color = "")
    {
        Console.ForegroundColor = color switch
        {
            "red" => ConsoleColor.Red,
            "dred" => ConsoleColor.DarkRed,
            "blue" => ConsoleColor.Blue,
            "dblue" => ConsoleColor.DarkBlue,
            "green" => ConsoleColor.Green,
            "dgreen" => ConsoleColor.DarkGreen,
            "yellow" => ConsoleColor.Yellow,
            "dyellow" => ConsoleColor.DarkYellow,
            "cyan" => ConsoleColor.Cyan,
            "dcyan" => ConsoleColor.DarkCyan,
            "magenta" or "purple" => ConsoleColor.Magenta,
            "dmagenta" => ConsoleColor.DarkMagenta,
            _ => ConsoleColor.White,
        };
        Console.WriteLine(data);
        Console.ForegroundColor = ConsoleColor.White;
    }

    static string ReadLine(string prompt)
    {
        Console.Write(prompt);
        string? res = Console.ReadLine();
        return res switch
        {
            not null => res,
            _ => "",
        };
    }

    static string GetRelativePath(string BasePath, string AbsolutePath)
    {
        Uri baseUri = new(BasePath);
        Uri absoluteUri = new(AbsolutePath);
        Uri relativeUri = baseUri.MakeRelativeUri(absoluteUri);
        string relativePath = Uri.UnescapeDataString(relativeUri.ToString());

        return relativePath.Replace('/', Path.DirectorySeparatorChar);
    }
}