file = File.open(ARGV[0])
data = {}
file.readlines.map(&:chomp).map do |l|
  part = l.split(': ')
  data[part[0]] = part[1].split(' ')
end

paths = Set.new
paths << ['you']
done = Set.new
loop do
  new_paths = Set.new
  paths.each do |path|
    data[path[-1]].each do |n|
      new_path = path + [n]
      if n == 'out'
        puts "Done: #{new_path}"
        done << new_path
      else
        new_paths << new_path
        puts "Continue: #{new_path}"
      end
    end
  end
  break if new_paths.empty?

  paths = new_paths
end

puts "Part 1: #{done.size}"
