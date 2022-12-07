file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

directories = {}
directories["/"] = {parent: nil, files: [], sizes: [], directories: []}
path = []
current_directory = ""
lines.each do |line|
  puts current_directory
  if line[0] == '$' # command
    puts line
    if line[0..3] == '$ cd'
      if line == '$ cd ..' # move up
        path.pop
      else
        path << line[5..]
      end
      puts path.inspect
      current_directory = path.join("/")
    elsif line[0..3] == '$ ls'
      # anything to do here ?
    end
  else
    parts = line.split(" ")
    if parts[0] == 'dir'
      new_directory_path = (path + [parts[1]]).join("/")
      directories[new_directory_path] = {parent: current_directory, files: [], sizes: [], directories: []}
      directories[current_directory][:directories] << new_directory_path
    else
      directories[current_directory][:files] << parts[1]
      directories[current_directory][:sizes] << parts[0].to_i
    end
  end
end

def sumup(dir_name, directories, level = 0)
  directory = directories[dir_name]
  total = directory[:sizes].sum
  directory[:directories].each do |dir|
    total += sumup(dir, directories, level + 1)
  end
  directories[dir_name][:total] = total
  total
end

sum_below_100k = directories.map {|name, dir| sumup(name, directories)}.select{|size| size <= 100000}.sum
puts "Result1: #{sum_below_100k}"

total_space = 70000000
min_free = 30000000
current_free = total_space - directories["/"][:total]
required_space = min_free - current_free

min = directories.values.map{|dir| dir[:total] }.select{|size| size >= required_space}.min
puts "Result2: #{min}"